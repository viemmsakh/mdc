use std::fs;
use std::path::{Path, PathBuf};
use std::convert::Infallible;

use anyhow::Result;
use axum::{
    response::sse::{Event as SseEvent, Sse},
    routing::get, 
    Router
};
use clap::Parser;
use notify::{recommended_watcher, Watcher, RecursiveMode, Event};
use tokio::signal;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

// Import custom modules
mod helper;
mod structs;
use structs::Args;
use structs::RenderOptions;

#[derive(Clone, Copy)]
pub enum OUTPUTS {
    FILE,
    STDOUT,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let watch = args.watch;
    let input_path = helper::validate_input_file(&args.input)?;

    if !watch {
        let output_type = if args.output.is_some() {
            OUTPUTS::FILE
        } else {
            OUTPUTS::STDOUT
        };
        let options = RenderOptions {
            boilerplate: args.html,
            output: output_type
        };
        let output_path = args.output;
        let _ = helper::render(input_path, output_path, options);
    } else {
        let output_path = match args.output.as_ref() {
            Some(path) => path.clone(),
            None => Path::new("_watch.html").to_path_buf(),
        };

        let is_temp_file = args.output.is_none();
        let cleanup_path = output_path.clone();

        let options = RenderOptions {
            boilerplate: true,
            output: OUTPUTS::FILE
        };

        let (tx, _rx) = broadcast::channel::<()>(16);
        let tx_watcher = tx.clone();
        let tx_server = tx.clone();

        helper::render(input_path.clone(), Some(output_path.clone()), options)?;

        // 4. Setup File Watcher
        let watch_input = input_path.clone();
        let watch_output = output_path.clone();

        let mut watcher = recommended_watcher(move |res: notify::Result<Event>| {
            match res {
                Ok(_) => {
                    println!("Change detected, re-rendering...");
                    // Watcher handles the render and disk write
                    if let Err(e) = helper::render(watch_input.clone(), Some(watch_output.clone()), options) {
                        eprintln!("Watcher render error: {:?}", e);
                    }
                    // Notify the browser to reload
                    let _ = tx_watcher.send(());
                }
                Err(e) => println!("Watcher error: {:?}", e),
            }
        })?;

        watcher.watch(&input_path, RecursiveMode::NonRecursive)?;

        let server_output = output_path.clone();

        let app = Router::new()
            .route("/", get(move || async move {
                // Server ONLY reads the file, it does not re-render
                match tokio::fs::read_to_string(&server_output).await {
                    Ok(content) => axum::response::Html(content),
                    Err(_) => axum::response::Html("<h1>Error reading rendered file</h1>".to_string()),
                }
            }))
            .route("/reload", get(move || async move {
                let rx = tx_server.subscribe();
                let stream = BroadcastStream::new(rx)
                    .map(|_| Ok::<SseEvent, Infallible>(SseEvent::default().data("reload")));
                Sse::new(stream)
            }));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
        println!("Watching {:?} and serving at http://127.0.0.1:3000", input_path);
        
        if let Err(e) = serve_with_cleanup(listener, app, is_temp_file, cleanup_path).await {
            eprintln!("Server error: {}", e);
        }
    }
    
    Ok(())
}

async fn serve_with_cleanup(
    listener: tokio::net::TcpListener, 
    app: Router, 
    should_cleanup: bool, 
    path: PathBuf
) -> Result<()> {
    let shutdown_signal = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    tokio::select! {
        _ = axum::serve(listener, app) => {
            // This happens if the server crashes or stops on its own
        },
        _ = shutdown_signal => {
            println!("\nShutdown signal received...");
        },
    }

    if should_cleanup && path.exists() {
        println!("Cleaning up temporary file: {:?}", path);
        let _ = fs::remove_file(path);
    }
    
    println!("Exiting gracefully.");
    Ok(())
}
use clap::Parser;
use std::path::PathBuf;

use crate::OUTPUTS;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "A simple Markdown to HTML converter",
    long_about = None
)]
pub struct Args {
    #[arg(short, long, value_name = "FILE", help = "Input markdown file path")]
    pub input: PathBuf,
    #[arg(short, long, value_name = "FILE", help = "Output file path, extension is infered if not given.")]
    pub output: Option<PathBuf>,
    #[arg(long, help = "Output HTML with boilerplate")]
    pub html: bool,
    #[arg(long, help = "Output PDF, requries --output")]
    pub pdf: bool,
    #[arg(short, long, help = "Watch the input file for changes and display updates live in browser")]
    pub live: bool,
    #[arg(short, long, help = "Port to serve the live preview on (default: 3000)", value_name = "PORT")]
    pub port: Option<u16>,
}


#[derive(Clone, Copy)]
pub struct RenderOptions {
    pub boilerplate: bool,
    pub live: bool,
    pub output: OUTPUTS,
}
use anyhow::{Context, Ok, Result, anyhow};
use ironpress::html_to_pdf;
use pulldown_cmark::{html, Options, Parser as MarkdownParser};
use std::fs;
use std::path::{Path, PathBuf};

use crate::OUTPUTS;
use crate::structs::RenderOptions;

pub fn convert_html_to_pdf(html: String) -> Result<Vec<u8>> {
    let pdf = html_to_pdf(&html)?;
    Ok(pdf)
}

pub fn convert_markdown_to_html(markdown: String) -> Result<String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_GFM);
    let parser = MarkdownParser::new_ext(&markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Ok(html_output)
}

pub fn create_output_path_if_not_exist(path: &Path) -> Result<()>{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory structure for {:?}", parent))?;
    }
    Ok(())
}

pub fn indent_html(html: String) -> String {
    let mut indented_content = String::with_capacity(html.len() * 2);
    let mut lines = html.lines().peekable();

    while let Some(line) = lines.next() {
        indented_content.push_str("    "); // Because spaces > tabs. Fight me!
        indented_content.push_str(line);
        if lines.peek().is_some() {
            indented_content.push('\n');
        }
    }
    indented_content
}

pub fn read_markdown_file(path: PathBuf) -> Result<String> {
    fs::read_to_string(&path).with_context(|| format!("Failed to read file: {:?}", path))
}

pub fn render(input_path: PathBuf, output_path: Option<PathBuf>, options: RenderOptions) -> Result<()> {
    let input_markdown = read_markdown_file(input_path)?;
    let html_output = convert_markdown_to_html(input_markdown)?;

    let final_html = if options.boilerplate {
        wrap_in_html_boilerplate(html_output, options.live)
    } else {
        html_output
    };

        
    match options.output {
        OUTPUTS::HTML => {
            if let Some(path) = output_path {
                let mut path = path;
                if path.extension().is_none() {
                    path.set_extension("html");
                }
                write_html_to_file(path, final_html)?;
                return Ok(());
            }
        },
        OUTPUTS::PDF => {
            if let Some(path) = output_path {
                let mut path = path;
                if path.extension().is_none() {
                    path.set_extension("pdf");
                }
                write_html_to_pdf_file(path, final_html)?;
                return Ok(());
            }
        },
        OUTPUTS::STDOUT => {
            write_html_to_stdout(final_html)?;
        }
    }
    Ok(())
}

pub fn validate_input_file(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Err(anyhow!("The path '{:?}' does not exist.", path));
    }
    if !path.is_file() {
        return Err(anyhow!("The path '{:?}' is not a file.", path));
    }
    Ok(path.to_path_buf())
}

pub fn wrap_in_html_boilerplate(html_content: String, live: bool) -> String {
    let formatted_html = indent_html(html_content);
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Markdown Export</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/water.css@2/out/water.css">
    <style>
        body {{ max-width: 800px; margin: 40px auto; padding: 0 20px; }}
    </style>
</head>
<body>
{}{}
</body>
</html>"#,
        formatted_html,
        if live { "\n    <script>
        const eventSource = new EventSource('/reload');
        eventSource.onmessage = (event) => {{
            if (event.data === 'reload') {{
                location.reload();
            }}
        }};
    </script>".to_string() } else { "".to_string() }
    )
}

pub fn write_html_to_file(path: PathBuf, html_content: String) -> Result<()> {
    create_output_path_if_not_exist(&path).ok();

    fs::write(&path, html_content)
        .with_context(|| format!("Failed to write HTML output to {:?}", path))?;

    println!("HTML output successfully written to {:?}", path);
    Ok(())
}

pub fn write_html_to_pdf_file(path: PathBuf, html_content: String) -> Result<()> {
    let pdf = convert_html_to_pdf(html_content)?;
    fs::write(&path, pdf)
        .with_context(|| format!("Failed to write PDF output to {:?}", path))?;
    
    
    println!("PDF output successfully written to {:?}", path);
    Ok(())
}

pub fn write_html_to_stdout(html_content: String) -> Result<()> {
    println!("{}", html_content);
    Ok(())
}
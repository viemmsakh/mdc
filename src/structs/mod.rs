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
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
    #[arg(long, help = "Output HTML with boilerplate")]
    pub html: bool,
    #[arg(short, long, help = "Watch the input file for changes and display updates live in browser")]
    pub live: bool,
}


#[derive(Clone, Copy)]
pub struct RenderOptions {
    pub boilerplate: bool,
    pub output: OUTPUTS,
}
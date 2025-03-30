pub mod cmark;
pub mod context;
pub mod markdown;
pub mod metadata;
pub mod utils;

use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// If set, strip any personally identifying information from the manuscript
    #[arg(long, action=ArgAction::SetTrue)]
    pub anonymous: Option<bool>,

    /// The file or directory containing the manuscript in Markdown format
    pub filename_or_path: String,

    /// The font to use in the manuscript
    #[arg(long, value_name = "Times New Roman")]
    pub font: Option<String>,

    /// The font size to use in the manuscript
    #[arg(long, value_name = "12")]
    pub font_size: Option<usize>,

    /// The output directory
    #[arg(short, long, value_name = "FILE")]
    pub output_dir: Option<PathBuf>,
}

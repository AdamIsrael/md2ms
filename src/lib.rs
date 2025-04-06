pub mod cmark;
pub mod context;
pub mod markdown;
pub mod metadata;
pub mod pii;
pub mod utils;

use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compile Markdown file(s) into Standard Manuscript Format
    Compile(CompileArgs),
    /// Install Obsidian integration
    Obsidian {},
    /// Get the estimated word count of the Markdown file(s)
    WordCount {},
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CompileArgs {
    /// If set, strip any personally identifying information from the manuscript
    #[arg(long, action=ArgAction::SetTrue)]
    pub anonymous: Option<bool>,

    /// Use classic manuscript format
    #[arg(long, action=ArgAction::SetTrue)]
    pub classic: Option<bool>,

    // #[arg(long, action=ArgAction::SetTrue)]
    // pub modern: Option<bool>,
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

    /// Personally Identifying Information in Markdown format
    /// Without this information, the manuscript will be anonymized.
    #[arg(long, value_name = "FILENAME")]
    pub pii: Option<String>,
}

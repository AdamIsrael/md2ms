pub mod cmark;
pub mod context;
pub mod error;
pub mod markdown;
pub mod metadata;
pub mod obsidian;
pub mod obsidian_shellcommands;
pub mod obsidian_commander;
pub mod pii;
pub mod utils;

use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compile Markdown file(s) into Standard Manuscript Format
    Compile(CompileArgs),
    /// Install Obsidian integration
    Obsidian(ObsidianArgs),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CompileArgs {
    /// If set, strip any personally identifying information from the manuscript
    #[arg(long, action=ArgAction::SetTrue, group="manuscript")]
    pub anonymous: Option<bool>,

    /// Use classic manuscript format
    #[arg(long, action=ArgAction::SetTrue, group="manuscript")]
    pub classic: Option<bool>,

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

    /// Display the word count and exit.
    #[arg(long, action=ArgAction::SetTrue)]
    pub word_count: Option<bool>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ObsidianArgs {
    /// The directory containing the Obsidian vault to integrate with.
    pub obsidian_path: String,

    /// Manuscript export path
    #[arg(long, default_value = "~/Writing/Drafts/")]
    pub export_path: Option<String>,

    /// The folder to use for the writing, defaulting to `Writing`
    /// This can be a relative or absolute path, from the vault root.
    #[arg(long, default_value = "Writing")]
    pub vault_folder: Option<String>,

    /// If set, overwrite existing shell commands.
    #[arg(long, action=ArgAction::SetTrue)]
    pub overwrite: Option<bool>,

    /// Uninstall the integration with Obsidian
    #[arg(long, action=ArgAction::SetTrue)]
    pub uninstall: Option<bool>,
}

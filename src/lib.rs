pub mod cmark;
pub mod constants;
pub mod context;
pub mod error;
pub mod markdown;
pub mod metadata;
pub mod obsidian;
pub mod obsidian_commander;
pub mod obsidian_shellcommands;
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
    /// The file or directory containing the manuscript in Markdown format
    pub filename_or_path: String,

    /// The directory to output the manuscripts to.
    #[arg(short, long, value_name = "DIRECTORY")]
    pub output_dir: Option<PathBuf>,

    /// Personally Identifying Information in Markdown format
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

use crate::markdown::{parse_markdown, parse_pii};
use crate::metadata::Metadata;
use crate::pii::PII;
use crate::utils::{get_base_filename, get_file_basedir, slurp};
use crate::{Cli, CompileArgs};

use std::collections::HashMap;
use std::fs::metadata;
use std::path::{Path, PathBuf};
use yaml_front_matter::Document;

/// The context for a manuscript
// #[derive(Copy, Debug)]
pub struct Context {
    /// Whether the manuscript should be anonymous or identifying
    pub anonymous: bool,

    /// Whether the manuscript should be formatted in classic style
    pub classic: bool,

    /// Whether the manuscript should be formatted in modern style
    // pub modern: bool,
    pub basedir: String,

    pub files: HashMap<String, Document<Metadata>>,

    /// The font to use for the docx
    pub font: String,

    /// The font size to use for the docx
    pub font_size: usize,

    /// Personally Identifiable Information
    pub pii: Option<Document<PII>>,

    /// The folder to create the manuscript in.
    pub output_dir: PathBuf,

    /// Only display the word count for the manuscript
    pub word_count: bool,
}

impl Context {
    pub fn new(args: &CompileArgs) -> Self {
        let basedir = args.filename_or_path.clone();

        // Every author has a different place for this. We just need a sane default
        let default_output_dir = PathBuf::from(
            shellexpand::tilde("~/Documents/Writing")
                .to_string()
                .to_owned(),
        );

        let mut s = Self {
            anonymous: args.anonymous.unwrap_or(false),
            basedir: basedir.clone(),
            classic: args.classic.unwrap_or(false),
            files: HashMap::new(),
            font: args.font.clone().unwrap_or("Times New Roman".to_string()),
            // For whatever reason, we have to double the font size to get the right size in the docx
            font_size: args.font_size.unwrap_or(24),
            // modern: args.modern.unwrap_or(true),
            pii: None,
            output_dir: args
                .output_dir
                .clone()
                .unwrap_or(default_output_dir.clone()),
            word_count: args.word_count.unwrap_or(false),
        };

        // TODO: read/parse in the PII so that it's available via Context
        if !s.anonymous {
            if let Some(pii) = args.pii.clone() {
                let pii = slurp(pii);
                if let Ok(pii) = parse_pii(pii) {
                    s.pii = Some(pii);
                }
            }
        }

        s.files = s.read_files(basedir.clone());

        s
    }

    pub fn get_file(&mut self, filename: String) -> Option<Document<Metadata>> {
        let file = self.files.get(&filename).unwrap();
        Some(Document {
            metadata: file.metadata.clone(),
            content: file.content.clone(),
        })
    }

    /// Not sure if this is needed anymore, or in its current state.
    /// I decided to move the header metadata into the scene, to make it easier to manage.
    /// I still need to handle "section" metadata, which this function doesn't quite cover.
    pub fn get_file_metadata(&mut self, file: String) -> Metadata {
        let mut p = get_file_basedir(format!("{}/{}", self.basedir, file));
        // Remove the basedir from the path

        p.push_str("/metadata.md");
        p = p.trim_start_matches('/').to_string();

        if let Ok(md) = metadata(&p) {
            if md.is_file() {
                let md = slurp(p);
                if let Ok(md) = parse_markdown(md) {
                    return md.metadata;
                }
            }
        }
        Metadata {
            author: None,
            short_author: None,
            heading: None,
            include: None,
            short_title: None,
            title: None,
            content_warnings: None,
        }
    }

    /// Determine if the file is Markdown, based on extension.
    fn is_markdown(&mut self, path: &String) -> bool {
        let mut markdown = false;
        if let Some(extension) = Path::new(path.as_str()).extension() {
            if extension == "md" {
                markdown = true;
            }
        }
        markdown
    }

    fn read_files(&mut self, path: String) -> HashMap<String, Document<Metadata>> {
        let mut files: HashMap<String, Document<Metadata>> = HashMap::new();

        let md = metadata(&path).unwrap();
        if md.is_file() {
            if self.is_markdown(&path) {
                let md = slurp(path.clone());
                if let Ok(md) = parse_markdown(md) {
                    files.insert(get_base_filename(self.basedir.clone(), path), md);
                }
            }
        } else {
            for entry in std::fs::read_dir(&path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                let spath = path.as_os_str().to_str().unwrap().to_string();

                if path.is_file() {
                    if self.is_markdown(&spath) {
                        let p = path.as_os_str().to_str().unwrap().to_string();
                        let md = slurp(p.clone());
                        if let Ok(md) = parse_markdown(md) {
                            // TODO: need to make sure get_base_filename returns the path, i.e. Act 1/Chapter 1/scene1.md
                            files.insert(
                                get_base_filename(self.basedir.clone(), spath),
                                // .to_lowercase(),
                                md,
                            );
                        } else {
                            println!("Failed to parse {:?}", p);
                        }
                    }
                } else if path.is_dir() {
                    // Fun with recursion goes here
                    files.extend(self.read_files(spath));
                } else {
                    println!("Skipping '{}'", path.display());
                }
            }
        }
        files
    }

    pub fn get_file_path(self, filename: String) -> String {
        format!("{}/{}", self.basedir, filename)
    }
}

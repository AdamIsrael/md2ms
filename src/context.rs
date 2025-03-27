use crate::markdown::parse_markdown;
use crate::metadata::Metadata;
use crate::utils::{get_base_filename, get_file_basedir, slurp};

use std::collections::HashMap;
use std::fs::metadata;
use yaml_front_matter::Document;

/// The context for a manuscript
// #[derive(Copy, Debug)]
pub struct Context {
    pub basedir: String,

    pub files: HashMap<String, Document<Metadata>>,
}

impl Context {
    pub fn new(basedir: String) -> Self {
        let mut s = Self {
            basedir: basedir.clone(),
            files: HashMap::new(),
        };

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

    fn read_files(&mut self, path: String) -> HashMap<String, Document<Metadata>> {
        let mut files: HashMap<String, Document<Metadata>> = HashMap::new();

        let md = metadata(&path).unwrap();
        if md.is_file() {
            let md = slurp(path.clone());
            if let Ok(md) = parse_markdown(md) {
                files.insert(get_base_filename(self.basedir.clone(), path), md);
            }
        } else {
            for entry in std::fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() {
                    let p = path.as_os_str().to_str().unwrap().to_string();
                    let md = slurp(p.clone());
                    if let Ok(md) = parse_markdown(md) {
                        // TODO: need to make sure get_base_filename returns the path, i.e. Act 1/Chapter 1/scene1.md
                        files.insert(
                            get_base_filename(
                                self.basedir.clone(),
                                path.as_os_str().to_str().unwrap().to_string(),
                            ),
                            md,
                        );
                    } else {
                        println!("Failed to parse {:?}", p);
                    }
                } else {
                    if path.is_dir() {
                        // println!("This is a directory!");
                        // Fun with recursion goes here
                        files.extend(
                            self.read_files(path.as_os_str().to_str().unwrap().to_string()),
                        );
                    } else {
                        println!("Skipping {:?}", path);
                    }
                    // TODO: Need to handle if this is a directory, and then read the files _in_ the directory
                }
            }
        }
        files
    }

    pub fn get_file_path(self, filename: String) -> String {
        format!("{}/{}", self.basedir, filename)
    }
}

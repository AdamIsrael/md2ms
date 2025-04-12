// TODO: Implement Obsidian integration
//
// Manipulate a given Obsidian vault, including but not limited to:
// - Creating notes
// - Deleting notes
// - Updating notes
// - Modifying JSON in `.obsidian` to change settings and install plugins
//
// Creating and manipulating notes is easy. On install, we'll want to create the PII file and maybe create the
// Writing/ folder structure.
//
use serde::{Deserialize, Serialize};
// use serde_json::Result;
use serde_json::{Result, Value};
use std::fs::{create_dir_all, remove_file, File};
use std::io::{copy, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
// use tempfile::Builder;
use crate::utils::slurp;
use std::time::UNIX_EPOCH;

pub struct Obsidian {
    pub vault_path: PathBuf,
    pub config_path: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct CommunityPlugin {
    pub id: String,
    pub author: String,
    pub name: String,
    pub description: String,
    pub repo: String,
}

impl CommunityPlugin {
    pub fn get_repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }
}

pub struct ObsidianReleases {
    pub community_plugins: Vec<CommunityPlugin>,
}

impl ObsidianReleases {
    pub fn new() -> Self {
        let mut s = Self {
            community_plugins: Vec::new(),
        };
        s.refresh_community_plugins();

        s
    }
    fn get_config_path(&self) -> PathBuf {
        PathBuf::from(
            shellexpand::tilde("~/.md2ms/obsidian/")
                .to_string()
                .to_owned(),
        )
    }

    fn refresh_community_plugins(&mut self) {
        // Check a locally cached version of the file
        let config = self.get_config_path();

        create_dir_all(config).unwrap();

        let cache = PathBuf::from(
            shellexpand::tilde("~/.md2ms/obsidian/community-plugins.json")
                .to_string()
                .to_owned(),
        );
        if cache.exists() && cache.is_file() {
            let file = File::open(&cache).expect("failed to open file");
            let seconds = file
                .metadata()
                .unwrap()
                .created()
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let now = UNIX_EPOCH.elapsed().unwrap().as_secs();
            let age = now - seconds;

            // For now, if the file is more than an hour old, fetch it again
            if age > 3600 {
                let _ = remove_file(&cache);
            } else {
                let contents = slurp(&cache);

                let p: Vec<CommunityPlugin> = serde_json::from_str(&contents).unwrap();
                self.community_plugins = p;
                return;
            }
        }

        // Fetch community plugins from GitHub
        let resp = reqwest::blocking::get(
            "https://raw.githubusercontent.com/obsidianmd/obsidian-releases/refs/heads/master/community-plugins.json",
        )
        .expect("request failed");
        let body = resp.text().expect("body invalid");
        let mut out = File::create(cache).expect("failed to create file");
        copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");

        // println!("Body: {}", body);

        // Parse the JSON response
        let p: Vec<CommunityPlugin> = serde_json::from_str(&body).unwrap();

        // Update the community_plugins field
        self.community_plugins = p;
    }
}

//
// #[derive(Serialize, Deserialize)]
// pub struct CommunityPlugins {
//     pub plugins: Vec<String>,
// }

// impl CommunityPlugins {
//     pub fn new() -> Self {
//         CommunityPlugins {
//             plugins: Vec::new(),
//         }
//     }
// }

impl Obsidian {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let vault_path: PathBuf = path.as_ref().to_string_lossy().into_owned().into();
        let config_path = vault_path.join(".obsidian");

        Obsidian {
            vault_path,
            config_path,
        }
    }

    pub fn is_vault(&self) -> bool {
        self.config_path.is_dir() && self.config_path.ends_with(".obsidian")
    }

    fn download_plugin(&self, plugin: String, url: String) -> bool {
        // Download the plugin from the given URL
        // Save it to the plugin folder
        // extract the plugin to ~/plugins/<plugin_name>

        // Return true if successful, false otherwise
        true
    }

    pub fn get_community_plugins(&self) -> Result<Vec<serde_json::Value>> {
        let file = self.config_path.join("community-plugins.json");
        if file.exists() {
            let content = std::fs::read_to_string(file).expect("Couldn't parse JSON");
            let plugins: Value = serde_json::from_str(content.as_str())?;
            println!("Plugins: {:?}", plugins);
            return Ok(plugins.as_array().unwrap().clone());
            // if let Ok(p) = plugins[0].as_array() {}
            // return Ok(plugins[0].as_array().unwrap().clone());
        }
        // Ok(vec![Value::Null])
        Ok(vec![])
    }

    pub fn add_community_plugin(&mut self, plugin: String, url: String) -> bool {
        if let Ok(mut plugins) = self.get_community_plugins() {
            // Install the plugin

            // create the plugin folder, i.e., plugins/<plugin_name>
            create_dir_all(self.config_path.join("plugins").join(&plugin)).unwrap();

            // query the metadata for the plugin? There may not be an API to do this. I might need
            // to install from the plugin's GitHub repository instead. Not ideal, especially for spinning this code
            // into a reusable crate, but workable for md2ms.

            // download the plugin
            if self.download_plugin(plugin.clone(), url) {
                // write the json file to install (enable?) the plugin
                let p = serde_json::to_value(plugin).unwrap();
                plugins.push(p);
                // write the file
                return self.write(plugins, self.config_path.join("community-plugins.json"));
            }
        }
        false
    }

    pub fn remove_community_plugin(&mut self, plugin: String) -> bool {
        if let Ok(mut plugins) = self.get_community_plugins() {
            // Iterate through the plugins and remove the one that matches
            let index = plugins.iter().position(|x| *x == plugin).unwrap();
            plugins.remove(index);

            // Remove the plugin from the filesystem
            // TODO: Test this
            // let plugin_path = self.config_path.join("plugins").join(plugin);
            // let _ = remove_dir_all(plugin_path);

            let path = self.config_path.join("community-plugins.json");
            if plugins.len() == 0 {
                let _ = remove_file(path);
                return true;
            } else {
                // write the file
                return self.write(plugins, path);
            }
        }
        false
    }

    fn write(&mut self, values: Vec<Value>, path: PathBuf) -> bool {
        if let Ok(file) = File::create(path) {
            let mut writer = BufWriter::new(file);
            let _ = serde_json::to_writer(&mut writer, &values);
            let _ = writer.flush();
        }
        true
    }
}

// Add tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obsidian_releases_refresh_community_plugins() {
        let mut or = ObsidianReleases::new();
        or.refresh_community_plugins();
        assert!(or.community_plugins.len() > 0);
    }

    #[test]
    fn test_obsidian_release_community_plugins() {
        let data = r#"
            [
            {
                "id": "nldates-obsidian",
                "name": "Natural Language Dates",
                "author": "Argentina Ortega Sainz",
                "description": "Create date-links based on natural language.",
                "repo": "argenos/nldates-obsidian"
            },
            {
                "id": "hotkeysplus-obsidian",
                "name": "Hotkeys++",
                "author": "Argentina Ortega Sainz",
                "description": "Additional hotkeys to do common things.",
                "repo": "argenos/hotkeysplus-obsidian"
            },
            {
                "id": "obsidian-advanced-uri",
                "name": "Obsidian Advanced URI",
                "author": "Argentina Ortega Sainz",
                "description": "Advanced URI support for Obsidian.",
                "repo": "argenos/obsidian-advanced-uri"
            },
            {
                "id": "obsidian-enhancing-export",
                "name": "Obsidian Enhancing Export",
                "author": "Argentina Ortega Sainz",
                "description": "Enhancing export for Obsidian.",
                "repo": "argenos/obsidian-enhancing-export"
            },
            {
                "id": "cmdr",
                "name": "CMDR",
                "author": "Argentina Ortega Sainz",
                "description": "Command line interface for Obsidian.",
                "repo": "argenos/cmdr"
            },
            {
                "id": "obsidian-shellcommands",
                "name": "Obsidian Shell Commands",
                "author": "Argentina Ortega Sainz",
                "description": "Shell commands for Obsidian.",
                "repo": "argenos/obsidian-shellcommands"
            },
            {
                "id": "dataview",
                "name": "DataView",
                "author": "Argentina Ortega Sainz",
                "description": "DataView for Obsidian.",
                "repo": "argenos/dataview"
            },
            {
                "id": "templater-obsidian",
                "name": "Templater Obsidian",
                "author": "Argentina Ortega Sainz",
                "description": "Templater for Obsidian.",
                "repo": "argenos/templater-obsidian"
            }
            ]"#;

        let p: Vec<CommunityPlugin> = serde_json::from_str(data).unwrap();
        assert_eq!(p.len(), 8);
        assert_eq!(p[5].id, "obsidian-shellcommands");
        assert_eq!(p[5].name, "Obsidian Shell Commands");
        assert_eq!(p[5].author, "Argentina Ortega Sainz");
        assert_eq!(p[5].description, "Shell commands for Obsidian.");
        assert_eq!(p[5].repo, "argenos/obsidian-shellcommands");
    }

    #[test]
    fn test_community_plugins() {
        // let data = r#"
        //     [
        //       "metadata-extractor",
        //       "obsidian-advanced-uri",
        //       "obsidian-enhancing-export",
        //       "cmdr",
        //       "obsidian-shellcommands",
        //       "dataview",
        //       "templater-obsidian"
        //     ]
        // "#;
        let vault_path = PathBuf::from("./examples/Obsidian/Blank/md2ms");
        let mut obsidian = Obsidian::new(vault_path);

        let plugins = obsidian.get_community_plugins();

        assert_eq!(plugins.unwrap().len(), 0);

        // Add a plugin
        // I may not want to actually download it during unit tests?
        // I could maybe add a fake plugin into my git repo, though, so I can test the code.
        obsidian.add_community_plugin(
            "obsidian-shellcommands".to_string(),
            // Currently part of a pull request. I'll have to update the URL once it's merged and the branch deleted.
            "https://github.com/AdamIsrael/md2ms/raw/7a902de0a68b959e376ae15eea75010c44fe7e8f/tests/obsidian-sample-plugin-1.0.0.tar.gz"
                .to_string(),
        );
        let plugins = obsidian.get_community_plugins();

        assert_eq!(plugins.unwrap().len(), 1);

        // Remove a plugin
        obsidian.remove_community_plugin("obsidian-shellcommands".to_string());
        let plugins = obsidian.get_community_plugins();

        assert_eq!(plugins.unwrap().len(), 0);
    }

    #[test]
    fn test_new() {
        let vault_path = PathBuf::from("./examples/Obsidian/Blank/md2ms");
        let obsidian = Obsidian::new(vault_path);

        assert_eq!(
            obsidian.vault_path,
            PathBuf::from("./examples/Obsidian/Blank/md2ms")
        );
        assert_eq!(
            obsidian.config_path,
            PathBuf::from("./examples/Obsidian/Blank/md2ms/.obsidian")
        );
    }

    #[test]
    fn test_is_vault() {
        let vault_path = PathBuf::from("./examples/Obsidian/Blank/md2ms");
        let obsidian = Obsidian::new(vault_path);

        assert!(obsidian.is_vault());
    }
}

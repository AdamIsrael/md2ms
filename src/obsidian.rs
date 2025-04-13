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
use crate::error::ObsidianError;
use crate::utils::{file_exists, slurp, slurp_url};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use std::fs::{create_dir_all, remove_file, File};
use std::io::{copy, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

pub struct Obsidian {
    pub vault_path: PathBuf,
    pub config_path: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct PluginManifest {
    id: String,
    name: String,
    version: String,
    #[serde(alias = "minAppVersion")]
    min_app_version: String,
    description: String,
    author: String,
    #[serde(alias = "authorUrl")]
    author_url: String,
    #[serde(alias = "fundingUrl")]
    funding_url: String,
    #[serde(alias = "isDesktopOnly")]
    is_desktop_only: bool,
}

impl PluginManifest {
    pub fn from_manifest(manifest: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(manifest)
    }
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

impl Default for ObsidianReleases {
    fn default() -> Self {
        Self::new()
    }
}

impl ObsidianReleases {
    pub fn new() -> Self {
        let mut s = Self {
            community_plugins: Vec::new(),
        };
        s.refresh_community_plugins().unwrap();

        s
    }
    fn get_config_path(&self) -> PathBuf {
        PathBuf::from(
            shellexpand::tilde("~/.md2ms/obsidian/")
                .to_string()
                .to_owned(),
        )
    }

    /// Refresh the community plugins list
    /// Populates the community_plugins field with the latest list of community plugins via cached file
    /// or fetches it from the internet if the cached file is not available or outdated.
    fn refresh_community_plugins(&mut self) -> Result<(), ObsidianError> {
        // Check a locally cached version of the file
        let config = self.get_config_path();

        if create_dir_all(config).is_err() {
            // Bail out if we can't create the directory
            return Err(ObsidianError::DirectoryCreationError);
        }

        let cache = self.get_config_path().join("community-plugins.json");
        if cache.exists() && cache.is_file() {
            if let Ok(file) = File::open(&cache) {
                // Checking the age of the cached filed is kinda ugly
                // TODO: Need to check that this works cross-platform.
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

                    if let Ok(p) = serde_json::from_str(&contents) {
                        self.community_plugins = p;
                        return Ok(());
                    } else {
                        return Err(ObsidianError::ParseError);
                    }
                }
            }
        }

        // Fetch community plugins from GitHub
        let data = slurp_url("https://raw.githubusercontent.com/obsidianmd/obsidian-releases/refs/heads/master/community-plugins.json".to_string());

        if let Ok(mut out) = File::create(cache) {
            if copy(&mut data.as_bytes(), &mut out).is_ok() {
                // Parse the JSON response
                let p: Vec<CommunityPlugin> = serde_json::from_str(&data).unwrap();

                // Update the community_plugins field
                self.community_plugins = p;
                return Ok(());
            }
        }

        Err(ObsidianError::OtherError)
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

    fn download_plugin(&self, url: String, _path: PathBuf) -> bool {
        // Download and parse the plugin's manifest
        let manifest_url = url.clone() + "/manifest.json";
        let manifest_string = slurp_url(manifest_url);
        if let Ok(manifest) = PluginManifest::from_manifest(&manifest_string) {
            // https://github.com/Taitava/obsidian-shellcommands/archive/refs/tags/0.23.0.tar.gz
            let _release_url = format!("{}/archive/refs/tags/{}.tar.gz", url, manifest.version);

            // Download the plugin from the given URL
            // Save it to the plugin folder
            // extract the plugin to ~/plugins/<plugin_name>
        }

        // Return true if successful, false otherwise
        true
    }

    pub fn get_community_plugins(&self) -> serde_json::Result<Vec<serde_json::Value>> {
        let path = self.config_path.join("community-plugins.json");
        if file_exists(&path) {
            let contents = slurp(&path);
            if !contents.is_empty() {
                let json: Value = serde_json::from_str(&contents).unwrap();
                return Ok(json.as_array().unwrap().clone());
            }
        }
        Ok(vec![])
    }

    pub fn install_community_plugin(&mut self, id: String) -> bool {
        if let Ok(mut plugins) = self.get_community_plugins() {
            if let Some(plugin) = ObsidianReleases::new()
                .community_plugins
                .iter()
                .find(|p| p.id == id)
            {
                // Install the plugin
                // create the plugin folder, i.e., plugins/<plugin_name>
                let path = self.config_path.join("plugins").join(&plugin.id);
                create_dir_all(&path).unwrap();

                let url = plugin.get_repo_url();

                // download the plugin into the proper folder
                if self.download_plugin(url, path) {
                    plugins.push(serde_json::to_value(&plugin.id).unwrap());
                    // write the file
                    return self.write(plugins, self.config_path.join("community-plugins.json"));
                }
            }
        }
        false
    }

    pub fn uninstall_community_plugin(&mut self, plugin: String) -> bool {
        if let Ok(mut plugins) = self.get_community_plugins() {
            // Iterate through the plugins and remove the one that matches
            let index = plugins.iter().position(|x| *x == plugin).unwrap();
            plugins.remove(index);

            // Remove the plugin from the filesystem
            // TODO: Test this
            // let plugin_path = self.config_path.join("plugins").join(plugin);
            // let _ = remove_dir_all(plugin_path);

            let path = self.config_path.join("community-plugins.json");
            if plugins.is_empty() {
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
        let _ = or.refresh_community_plugins();
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
        obsidian.install_community_plugin("obsidian-shellcommands".to_string());
        let plugins = obsidian.get_community_plugins();

        assert_eq!(plugins.unwrap().len(), 1);

        // Remove a plugin
        obsidian.uninstall_community_plugin("obsidian-shellcommands".to_string());
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

    #[test]
    fn test_plugin_manifest() {
        let data = r#"
            {
           	"id": "obsidian-shellcommands",
           	"name": "Shell commands",
           	"version": "0.23.0",
           	"minAppVersion": "1.4.0",
           	"description": "You can predefine system commands that you want to run frequently, and assign hotkeys for them. For example open external applications. Automatic execution is also supported, and execution via URI links.",
           	"author": "Jarkko Linnanvirta",
           	"authorUrl": "https://github.com/Taitava",
            "fundingUrl": "https://publish.obsidian.md/shellcommands/Donate",
           	"isDesktopOnly": true
            }"#;

        let pm = PluginManifest::from_manifest(data).unwrap();
        assert_eq!(pm.id, "obsidian-shellcommands");
        assert_eq!(pm.name, "Shell commands");
        assert_eq!(pm.version, "0.23.0");
        assert_eq!(pm.min_app_version, "1.4.0");
        assert_eq!(
            pm.funding_url,
            "https://publish.obsidian.md/shellcommands/Donate"
        );
        assert_eq!(pm.is_desktop_only, true);
    }
}

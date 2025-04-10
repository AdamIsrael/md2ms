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
// use serde::{Deserialize, Serialize};
// use serde_json::Result;
use serde_json::{Result, Value};
use std::fs::{remove_file, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

pub struct Obsidian {
    pub vault_path: PathBuf,
    pub config_path: PathBuf,
}

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
        // pub fn new<P: AsRef<PathBuf>>(path: P) -> Self {

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
            // create the plugin folder

            // query the metadata for the plugin? There may not be an API to do this. I might need
            // to install from the plugin's GitHub repository instead. Not ideal, especially for spinning this code
            // into a reusable crate, but workable for md2ms.
            // download the plugin

            // extract the plugin to ~/plugins/<plugin_name>

            // write the json file to install (enable?) the plugin
            let p = serde_json::to_value(plugin).unwrap();
            plugins.push(p);
            // write the file
            return self.write(plugins, self.config_path.join("community-plugins.json"));
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
            "https://github.com/adamisrael/md2ms/tests/obsidian-sample-plugin-1.0.0.tar.gz"
                .to_string(),
        );
        let plugins = obsidian.get_community_plugins();

        assert_eq!(plugins.unwrap().len(), 1);

        // Remove a plugin
        // obsidian.remove_community_plugin("obsidian-shellcommands".to_string());
        // let plugins = obsidian.get_community_plugins();

        // assert_eq!(plugins.unwrap().len(), 0);
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

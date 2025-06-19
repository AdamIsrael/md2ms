// Used https://transform.tools/json-to-rust-serde to generate the structs
// Interface to generate configuration for Commander
use obsidian_rs::Obsidian;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::fs::File;
use std::io::{Error, Read};
use std::path::{Path, PathBuf};

pub struct ObsidianCommander {
    pub data: Root,
    pub obsidian: Obsidian,
    // pub export_path: PathBuf,
    pub vault_folder: PathBuf,
    // pub overwrite: bool,
}

impl ObsidianCommander {
    pub fn new<P: AsRef<Path>>(obsidian: Obsidian, vault_folder: P) -> Result<Self, Error> {
        let mut path = obsidian.config_path.clone();
        if !path.ends_with(".obsidian") {
            path.push(".obsidian");
        }
        path.push("plugins/cmdr/data.json");
        println!("Looking for data at {}", path.display());

        // Return an error, i.e., if the path is bad.
        // Load the current configuration
        print!("Loading data...");
        let mut root: Root = Root::default();
        if let Ok(r) = Root::load(&path) {
            root = r;
        }
        println!("Done.");

        Ok(Self {
            data: root,
            obsidian,
            vault_folder: vault_folder.as_ref().to_path_buf(),
        })
    }

    pub fn add_file_menu(&mut self, file_menu: FileMenu) {
        self.data
            .file_menu
            .retain(|x| !x.name.eq(file_menu.name.as_str()));

        self.data.file_menu.push(file_menu);
    }

    pub fn save(&self) -> Result<(), Error> {
        let mut path = self.obsidian.config_path.clone();
        // TODO: De-dupe the next three lines (also in new above)
        if !path.ends_with(".obsidian") {
            path.push(".obsidian");
        }
        path.push("plugins/cmdr/data.json");

        self.data.save(&path)
    }

    pub fn sync(&mut self) {
        println!("Syncing cmdr...");

        // self.data.shell_commands.retain(|x| !x.alias.eq(command));
        // self.data.shell_commands.retain(|x| !x.alias.eq(command));
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "confirmDeletion")]
    pub confirm_deletion: bool,
    #[serde(rename = "showAddCommand")]
    pub show_add_command: bool,
    pub debug: bool,
    #[serde(rename = "editorMenu")]
    pub editor_menu: Vec<Value>,
    #[serde(rename = "fileMenu")]
    pub file_menu: Vec<FileMenu>,
    #[serde(rename = "leftRibbon")]
    pub left_ribbon: Vec<Value>,
    #[serde(rename = "rightRibbon")]
    pub right_ribbon: Vec<Value>,
    #[serde(rename = "titleBar")]
    pub title_bar: Vec<Value>,
    #[serde(rename = "statusBar")]
    pub status_bar: Vec<Value>,
    #[serde(rename = "pageHeader")]
    pub page_header: Vec<Value>,
    pub macros: Vec<Value>,
    pub explorer: Vec<Value>,
    pub hide: Hide,
    pub spacing: i64,
    #[serde(rename = "advancedToolbar")]
    pub advanced_toolbar: AdvancedToolbar,
}

impl Root {
    pub fn new() -> Self {
        Self {
            confirm_deletion: true,
            show_add_command: true,
            debug: false,
            editor_menu: Vec::new(),
            file_menu: Vec::new(),
            left_ribbon: Vec::new(),
            right_ribbon: Vec::new(),
            title_bar: Vec::new(),
            status_bar: Vec::new(),
            page_header: Vec::new(),
            macros: Vec::new(),
            explorer: Vec::new(),
            hide: Hide::default(),
            spacing: 0,
            advanced_toolbar: AdvancedToolbar::default(),
        }
    }

    /// Save the plugin configuration
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        println!("Saving plugin configuration to {}", path.as_ref().display());
        let mut file = File::create(path)?;
        serde_json::to_writer_pretty(&mut file, self)?;
        Ok(())
    }

    /// Load the plugin configuration
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(serde_json::from_str(&contents)?)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileMenu {
    pub id: String,
    pub icon: String,
    pub name: String,
    pub mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hide {
    pub statusbar: Vec<Value>,
    #[serde(rename = "leftRibbon")]
    pub left_ribbon: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdvancedToolbar {
    #[serde(rename = "rowHeight")]
    pub row_height: i64,
    #[serde(rename = "rowCount")]
    pub row_count: i64,
    pub spacing: i64,
    #[serde(rename = "buttonWidth")]
    pub button_width: i64,
    #[serde(rename = "columnLayout")]
    pub column_layout: bool,
    #[serde(rename = "mappedIcons")]
    pub mapped_icons: Vec<Value>,
    pub tooltips: bool,
    #[serde(rename = "heightOffset")]
    pub height_offset: i64,
}

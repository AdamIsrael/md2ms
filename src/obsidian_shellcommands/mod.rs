// Used https://transform.tools/json-to-rust-serde to generate the structs
use obsidian_rs::Obsidian;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::fs::File;
use std::io::{Error, Read};
use std::path::{Path, PathBuf};

use rand::distr::{Alphanumeric, SampleString};

const COMMANDS: &[&str] = &[
    "Export to Standard Manuscript Format (Classic)",
    "Export to Standard Manuscript Format (Modern)",
    "Word Count",
];

pub struct ObsidianShellcommands {
    pub data: Root,
    pub obsidian: Obsidian,
    pub export_path: PathBuf,
    pub vault_folder: PathBuf,
    pub overwrite: bool,
}

impl ObsidianShellcommands {
    pub fn new<P: AsRef<Path>>(
        obsidian: Obsidian,
        export_path: P,
        vault_folder: P,
        overwrite: bool,
    ) -> Result<Self, Error> {
        let mut path = obsidian.config_path.clone();
        if !path.ends_with(".obsidian") {
            path.push(".obsidian");
        }
        path.push("plugins/obsidian-shellcommands/data.json");

        // Return an error, i.e., if the path is bad.
        // Load the current configuration
        let mut root: Root = Root::new();

        if let Ok(r) = Root::load(&path) {
            root = r;
        }

        Ok(ObsidianShellcommands {
            data: root,
            obsidian: obsidian.clone(),
            export_path: export_path.as_ref().to_path_buf(),
            vault_folder: vault_folder.as_ref().to_path_buf(),
            overwrite,
        })
    }

    pub fn save(&self) -> Result<(), Error> {
        let mut path = self.obsidian.config_path.clone();
        // TODO: De-dupe the next three lines (also in new above)
        if !path.ends_with(".obsidian") {
            path.push(".obsidian");
        }
        path.push("plugins/obsidian-shellcommands/data.json");

        self.data.save(&path)
    }

    fn get_cmd_word_count(&self) -> ShellCommand {
        let mut command = ShellCommand::new(generate_id().as_str(), vec!["hello"]);

        command.alias = "Word Count".to_string();

        // Set the command's icon
        command.icon = "lucide-book".to_string();

        // For checking the word count, we don't need to output the manuscript or worry about PII. We just need to
        // know the path to the draft inside the vault.
        let platform_specific_commands = PlatformSpecificCommands {
            default: "md2ms compile --word-count {{folder_path:absolute}}".to_string(),
        };

        command.platform_specific_commands = platform_specific_commands;

        // Set Output Handlers
        command.output_handlers.stdout.handler = "notification".to_string();
        command.output_handlers.stdout.convert_ansi_code = true;
        command.output_handlers.stderr.handler = "notification".to_string();
        command.output_handlers.stderr.convert_ansi_code = true;

        command.command_palette_availability = "enabled".to_string();

        command.output_channel_order = "stdout-first".to_string();
        command.output_handling_mode = "buffered".to_string();
        command.execution_notification_mode = "disabled".to_string();
        command
    }

    /// Get the base command for exporting a manuscript.
    fn get_cmd_export_base(&self) -> ShellCommand {
        let mut command = ShellCommand::new(generate_id().as_str(), vec!["hello"]);

        // Set the command's icon
        command.icon = "lucide-book".to_string();

        let mut platform_specific_commands = PlatformSpecificCommands::default();

        let mut pii = self.obsidian.vault_path.clone();
        pii.push(self.vault_folder.clone());
        pii.push("PII.md");

        // TODO: need to figure out how to make the path to PII more generic
        // I've added Obsidian as an argument, but I'll need to extract it from the vault_path
        platform_specific_commands.default = format!(
            "md2ms compile {{folder_path:absolute}} --output-dir \"{}\" --pii \"{}\"",
            self.export_path.display(),
            pii.display()
        );

        command.platform_specific_commands = platform_specific_commands;

        // Set Output Handlers
        command.output_handlers.stdout.handler = "notification".to_string();
        command.output_handlers.stdout.convert_ansi_code = true;
        command.output_handlers.stderr.handler = "notification".to_string();
        command.output_handlers.stderr.convert_ansi_code = true;

        command.command_palette_availability = "enabled".to_string();

        command
    }

    fn get_cmd_export_to_standard_manuscript_format(&self) -> ShellCommand {
        let mut command = self.get_cmd_export_base();
        command.alias = "Export to Standard Manuscript Format".to_string();
        command
            .platform_specific_commands
            .default
            .push_str(" --modern");

        command
    }

    /// Syncronize the plugin's configuration to disk
    pub fn sync(&mut self) {
        println!("Syncing shell command(s)...");
        for command in COMMANDS {
            let c = match *command {
                "Word Count" => self.get_cmd_word_count(),
                // The default
                _ => self.get_cmd_export_to_standard_manuscript_format(),
            };
            self.data.shell_commands.retain(|x| !x.alias.eq(command));
            // Only append if the command is not already present
            if !self.data.shell_commands.iter().any(|x| x.alias.eq(&c.alias)) {
                self.data.shell_commands.push(c.clone());
            }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub settings_version: String,
    pub debug: bool,
    pub obsidian_command_palette_prefix: String,
    pub preview_variables_in_command_palette: bool,
    pub show_autocomplete_menu: bool,
    pub working_directory: String,
    pub default_shells: DefaultShells,
    pub environment_variable_path_augmentations: EnvironmentVariablePathAugmentations,
    pub show_installation_warnings: bool,
    pub error_message_duration: i64,
    pub notification_message_duration: i64,
    pub execution_notification_mode: String,
    pub output_channel_clipboard_also_outputs_to_notification: bool,
    pub output_channel_notification_decorates_output: bool,
    pub output_channel_order: String,
    pub output_handling_mode: String,
    pub enable_events: bool,
    pub approve_modals_by_pressing_enter_key: bool,
    pub command_palette: CommandPalette,
    pub max_visible_lines_in_shell_command_fields: bool,
    pub shell_commands: Vec<ShellCommand>,
    pub prompts: Vec<Value>,
    pub builtin_variables: BuiltinVariables,
    pub custom_variables: Vec<Value>,
    pub custom_variables_notify_changes_via: CustomVariablesNotifyChangesVia,
    pub custom_shells: Vec<Value>,
    pub output_wrappers: Vec<Value>,
}

impl Root {
    pub fn new() -> Self {
        Root {
            settings_version: "0.23.0".to_string(),
            debug: false,
            obsidian_command_palette_prefix: "Execute: ".to_string(),
            preview_variables_in_command_palette: true,
            show_autocomplete_menu: true,
            working_directory: "".to_string(),
            default_shells: DefaultShells::default(),
            environment_variable_path_augmentations: EnvironmentVariablePathAugmentations::default(
            ),
            show_installation_warnings: true,
            error_message_duration: 20,
            notification_message_duration: 10,
            execution_notification_mode: "disabled".to_string(),
            output_channel_clipboard_also_outputs_to_notification: true,
            output_channel_notification_decorates_output: true,
            output_handling_mode: "".to_string(),
            output_channel_order: "".to_string(),
            enable_events: true,
            approve_modals_by_pressing_enter_key: true,
            command_palette: CommandPalette::default(),
            max_visible_lines_in_shell_command_fields: false,
            shell_commands: Vec::new(),
            prompts: Vec::new(),
            builtin_variables: BuiltinVariables::default(),
            custom_variables: Vec::new(),
            custom_variables_notify_changes_via: CustomVariablesNotifyChangesVia::default(),
            custom_shells: Vec::new(),
            output_wrappers: Vec::new(),
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
pub struct DefaultShells {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentVariablePathAugmentations {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandPalette {
    pub re_execute_last_shell_command: ReExecuteLastShellCommand,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReExecuteLastShellCommand {
    pub enabled: bool,
    pub prefix: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShellCommand {
    pub id: String,
    pub platform_specific_commands: PlatformSpecificCommands,
    pub shells: Shells,
    pub alias: String,
    pub icon: String,
    pub confirm_execution: bool,
    pub ignore_error_codes: Vec<Value>,
    pub input_contents: InputContents,
    pub output_handlers: OutputHandlers,
    pub output_wrappers: OutputWrappers,
    pub output_channel_order: String,
    pub output_handling_mode: String,
    pub execution_notification_mode: String,
    pub events: Events,
    pub debounce: Value,
    pub command_palette_availability: String,
    pub preactions: Vec<Value>,
    pub variable_default_values: VariableDefaultValues,
}

impl ShellCommand {
    pub fn new(id: &str, _args: Vec<&str>) -> Self {
        ShellCommand {
            id: id.to_string(),
            platform_specific_commands: PlatformSpecificCommands::default(),
            shells: Shells::default(),
            alias: String::new(),
            icon: String::new(),
            confirm_execution: false,
            ignore_error_codes: Vec::new(),
            input_contents: InputContents::default(),
            output_handlers: OutputHandlers::default(),
            output_wrappers: OutputWrappers::default(),
            output_channel_order: String::new(),
            output_handling_mode: String::new(),
            execution_notification_mode: String::new(),
            events: Events::default(),
            debounce: Value::default(),
            command_palette_availability: String::new(),
            preactions: Vec::new(),
            variable_default_values: VariableDefaultValues::default(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlatformSpecificCommands {
    pub default: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shells {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputContents {
    pub stdin: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputHandlers {
    pub stdout: Stdout,
    pub stderr: Stderr,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stdout {
    pub handler: String,
    pub convert_ansi_code: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stderr {
    pub handler: String,
    pub convert_ansi_code: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputWrappers {
    pub stdout: Value,
    pub stderr: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Events {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableDefaultValues {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuiltinVariables {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomVariablesNotifyChangesVia {
    pub obsidian_uri: bool,
    pub output_assignment: bool,
}

/// Generate a random ID
/// - alphanumeric
/// - lowercase
/// - minimum length of 10 characters
fn generate_id() -> String {
    Alphanumeric
        .sample_string(&mut rand::rng(), 10)
        .to_string()
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let obs = Obsidian::new("examples/Obsidian/Blank/md2ms");
        let shell =
            ObsidianShellcommands::new(obs, "~/Documents/Writing", "/Writing", true).unwrap();
        // println!("Got data: {:?}", shell.data);

        assert_eq!(shell.data.settings_version, "0.23.0");

        // let mut command = ShellCommand::new(generate_id().as_str(), vec!["hello"]);
        // let mut platform_specific_commands = PlatformSpecificCommands::default();

        // command.icon = "lucide-book".to_string();
        // command.alias = "Export to Times New Roman".to_string();
        // platform_specific_commands.default = "md2ms compile {{folder_path:absolute}} --output-dir ~/Documents/Writing --pii \"/Users/adam/Library/Mobile Documents/iCloud~md~obsidian/Documents/loci/Writing/PII.md\"".to_string();

        // command.platform_specific_commands = platform_specific_commands;

        // // Set Output Handlers
        // command.output_handlers.stdout.handler = "notification".to_string();
        // command.output_handlers.stdout.convert_ansi_code = true;
        // command.output_handlers.stderr.handler = "notification".to_string();
        // command.output_handlers.stderr.convert_ansi_code = true;

        // command.command_palette_availability = "enabled".to_string();

        // shell.data.shell_commands.push(command);

        // write out a data.json based on the struct defaults
        let data = serde_json::to_string_pretty(&shell.data).unwrap();
        std::fs::write("data.json", data).unwrap();
    }
}

// TODO: Implement Obsidian integration
// This started as a rough integration with Obsidian, but has grown complex enough to warrant splitting it
// into its own crate, obsidian-rs. Keeping this file in place, to serve as the integration with obsidian-rs,
// once it's ready to be used.
use obsidian_rs::Obsidian;

use std::path::Path;

use crate::obsidian_commander::{FileMenu, ObsidianCommander};
use crate::obsidian_shellcommands::ObsidianShellcommands;

// TODO: Might want to do required/recommended plugins in the future
const PLUGINS: &[&str] = &["cmdr", "obsidian-shellcommands"];

pub fn update_obsidian_vault<P: AsRef<Path>>(
    obsidian_path: P,
    export_path: P,
    vault_folder: P,
    overwrite: bool,
) {
    let mut o = Obsidian::new(obsidian_path);

    if o.is_vault() {
        // Install the required plugins
        for plugin in PLUGINS {
            o.install_community_plugin(plugin.to_string());

            // TODO: (re)configure the plugin?
            // It's not pretty, but I'll have to hard-code some plugin logic here.
            //     match *plugin {
            //         "cmdr" => {
            //             if let Ok(mut p) = ObsidianCommander::new(
            //                 o.clone(),
            //                 &vault_folder,
            //             ) {
            //                 print!("Syncing commander...");
            //                 print!("{:?}", p.data);

            //                 let fm = FileMenu {
            //                     id: "obsidian-shellcommands:shell-command-zbyzvt4l2k".to_string(),
            //                     icon: "lucide-book-template".to_string(),
            //                     name: "Export to Standard Manuscript Format (Classic)".to_string(),
            //                     mode: "desktop".to_string(),
            //                 };
            //                 p.add_file_menu(fm);
            //                 p.sync();

            //                 let _ = p.save();
            //                 println!("done!");
            //             }

            //             // o.configure_plugin("obsidian-commander", "commander.json");
            //         }
            //         "obsidian-shellcommands" => {
            //             // if let Ok(mut p) = ObsidianShellcommands::new(
            //             //     o.clone(),
            //             //     &export_path,
            //             //     &vault_folder,
            //             //     overwrite,
            //             // ) {
            //             //     print!("Syncing shell commands...");
            //             //     p.sync();

            //             //     let _ = p.save();
            //             //     println!("done!");
            //             // }

            //             // o.configure_plugin("obsidian-shellcommands", "shellcommands.json");
            //         }
            //         _ => {}
            //     }
        }

        // Sync the shell commands first
        if let Ok(mut p) =
            ObsidianShellcommands::new(o.clone(), &export_path, &vault_folder, overwrite)
        {
            print!("Syncing shell commands...");
            p.sync();

            let _ = p.save();
            println!("done!");

            if let Ok(mut cmdr) = ObsidianCommander::new(o.clone(), &vault_folder) {
                print!("Syncing commander...");
                print!("{:?}", cmdr.data);

                for cmd in p.data.shell_commands {
                    let fm = FileMenu {
                        id: format!("obsidian-shellcommands:shell-command-{}", cmd.id),
                        icon: "lucide-book-template".to_string(),
                        name: cmd.alias,
                        mode: "desktop".to_string(),
                    };
                    cmdr.add_file_menu(fm);
                }
                cmdr.sync();

                let _ = cmdr.save();
                println!("done!");
            }
        }

        // Sync Commander, which requires data from ObsidianShellCommands.
    }

    // o.update_vault();
}

/// Add tests
#[cfg(test)]
mod tests {
    // use super::*;
}

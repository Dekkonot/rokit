use anyhow::{Context, Result};
use clap::Parser;

use aftman::storage::Home;

/// Installs / re-installs Aftman, and updates all tool links.
#[derive(Debug, Parser)]
pub struct SelfInstallSubcommand {}

impl SelfInstallSubcommand {
    pub async fn run(&self, home: &Home) -> Result<()> {
        let storage = home.tool_storage();

        let (had_aftman_installed, was_aftman_updated) =
            storage.recreate_all_links().await.context(
                "Failed to recreate tool links!\
                \nYour installation may be corrupted.",
            )?;

        // TODO: Automatically populate the PATH variable
        let path_was_populated = false;
        let path_message_lines = if !path_was_populated {
            "\nBinaries for Aftman and tools have been added to your PATH.\
            \nPlease restart your terminal for the changes to take effect."
        } else {
            ""
        };

        let main_message = if !had_aftman_installed {
            "Aftman has been installed successfully!"
        } else if was_aftman_updated {
            "Aftman was re-linked successfully!"
        } else {
            "Aftman is already up-to-date."
        };

        tracing::info!("{main_message}{path_message_lines}");

        Ok(())
    }
}
use aftman::storage::Home;
use anyhow::{Context, Result};
use clap::Parser;
use tokio::time::Instant;

mod add;
mod install;
mod list;
mod trust;

use self::add::AddSubcommand;
use self::install::InstallSubcommand;
use self::list::ListSubcommand;
use self::trust::TrustSubcommand;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        // 1. Load aftman data structures
        let start_home = Instant::now();
        let home = Home::load_from_env().await.context(
            "Failed to load Aftman home!\
            \nYour installation or environment may be corrupted.",
        )?;
        tracing::trace!(
            elapsed = ?start_home.elapsed(),
            "Aftman loaded"
        );

        // 2. Run the subcommand and capture the result - note that we
        // do not (!!!) use the question mark operator here, because we
        // want to save our data below even if the subcommand fails.
        let start_command = Instant::now();
        let result = self.subcommand.run(&home).await;
        tracing::trace!(
            elapsed = ?start_command.elapsed(),
            success = result.is_ok(),
            "Aftman ran",
        );

        // 3. Save aftman data structures to disk
        let start_save = Instant::now();
        home.save().await.context(
            "Failed to save Aftman data!\
            \nChanges to trust, tools, and more may have been lost.",
        )?;
        tracing::trace!(
            elapsed = ?start_save.elapsed(),
            "Aftman saved"
        );

        // 4. Return the result of the subcommand
        result
    }
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    Add(AddSubcommand),
    List(ListSubcommand),
    Trust(TrustSubcommand),
    Install(InstallSubcommand),
}

impl Subcommand {
    pub async fn run(self, home: &Home) -> Result<()> {
        match self {
            Self::Add(cmd) => cmd.run(home).await,
            Self::List(cmd) => cmd.run(home).await,
            Self::Trust(cmd) => cmd.run(home).await,
            Self::Install(cmd) => cmd.run(home).await,
        }
    }
}

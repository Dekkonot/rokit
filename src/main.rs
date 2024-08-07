use std::process::exit;

use clap::Parser;
use tracing::error;

mod cli;
mod runner;
mod util;

use self::cli::Cli;
use self::runner::Runner;
use self::util::init_tracing;

#[tokio::main]
async fn main() {
    init_tracing();

    let runner = Runner::new();
    let result = if runner.should_run() {
        runner.run().await
    } else {
        Cli::parse().run().await
    };

    // NOTE: We use tracing for errors here for consistent
    // output between returned errors, and errors that
    // may be logged while the program is running.
    if let Err(e) = result {
        error!("{e:?}");
        exit(1);
    }
}

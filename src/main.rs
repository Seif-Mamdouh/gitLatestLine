use clap::{Parser, Subcommand};

mod reminding;

use crate::reminding::show_latest_changes;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Subcommand)]
enum Command {
    Remind,
}

fn main() {
    let args: Cli = Cli::parse();

    match args.command {
        Command::Remind => {
            show_latest_changes();
        }
    }
}

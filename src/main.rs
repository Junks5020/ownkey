mod cli;
mod commands;
mod keychain;
mod models;
mod session;
mod vault;
mod vault_store;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => commands::init::handle(args)?,
        Commands::Add(args) => commands::add::handle(args)?,
        Commands::List(args) => commands::list::handle(args)?,
        Commands::View(args) => commands::view::handle(args)?,
        Commands::Delete(args) => commands::delete::handle(args)?,
        Commands::Search(args) => commands::search::handle(args)?,
        Commands::RestoreBackup => commands::restore::handle()?,
    }

    Ok(())
}

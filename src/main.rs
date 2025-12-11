mod cli;
mod commands;
mod config;
mod sync;
mod keychain;
mod models;
mod session;
mod vault;
mod vault_store;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let _config = match config::load_or_init() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Warning: failed to load config: {}", err);
            config::Config::default()
        }
    };

    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => commands::init::handle(args)?,
        Commands::Add(args) => commands::add::handle(args)?,
        Commands::List(args) => commands::list::handle(args)?,
        Commands::View(args) => commands::view::handle(args)?,
        Commands::Copy(args) => commands::copy::handle(args)?,
        Commands::RotatePassword(args) => commands::rotate_password::handle(args)?,
        Commands::Delete(args) => commands::delete::handle(args)?,
        Commands::Search(args) => commands::search::handle(args)?,
        Commands::RestoreBackup => commands::restore::handle()?,
        Commands::Sync(_) => cli::sync_cmd::handle_sync(),
        Commands::Login(args) => cli::sync_cmd::handle_login(args.username.as_deref()),
        Commands::Logout => cli::sync_cmd::handle_logout(),
    }

    Ok(())
}

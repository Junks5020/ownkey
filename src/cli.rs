use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "ownkey", about = "A learning-friendly CLI password manager skeleton")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new vault at the given path
    Init(InitArgs),
    /// Add a new item to the vault
    Add(AddArgs),
    /// List items in the vault
    List(ListArgs),
    /// View a specific item by id
    View(ViewArgs),
    /// Delete an item by id
    Delete(DeleteArgs),
    /// Search items by keyword
    Search(SearchArgs),
    /// Restore the encrypted backup over the current vault (hidden)
    #[command(name = "restore-backup", hide = true)]
    RestoreBackup,
}

#[derive(clap::Args, Debug)]
pub struct InitArgs {
    /// Path to the vault file (defaults to ~/.ownkey/vault.json)
    pub path: Option<String>,
    /// Optional password (falls back to interactive prompt)
    #[arg(long)]
    pub password: Option<String>,
    /// Optional keychain account name (macOS only); when provided, password will be saved to Keychain
    #[arg(long)]
    pub keychain_account: Option<String>,
    /// Keychain service name (macOS only)
    #[arg(long, default_value = "ownkey")]
    pub keychain_service: String,
    /// Disable session cache usage for this command
    #[arg(long)]
    pub no_session: bool,
}

#[derive(clap::Args, Debug, Default)]
pub struct AddArgs {
    /// Optional path to the vault file
    #[arg(short, long)]
    pub path: Option<String>,
    /// Key name to store
    pub key: String,
    /// Value to store
    pub value: Option<String>,
    /// Optional notes (unused for KV mode, kept for future)
    #[arg(long)]
    pub notes: Option<String>,
    /// Optional password (falls back to interactive prompt)
    #[arg(long)]
    pub password: Option<String>,
    /// Optional keychain account name (macOS only)
    #[arg(long)]
    pub keychain_account: Option<String>,
    /// Keychain service name (macOS only)
    #[arg(long, default_value = "ownkey")]
    pub keychain_service: String,
    /// Disable session cache usage for this command
    #[arg(long)]
    pub no_session: bool,
}

#[derive(clap::Args, Debug, Default)]
pub struct ListArgs {
    /// Optional path to the vault file
    #[arg(short, long)]
    pub path: Option<String>,
    /// Optional password (falls back to interactive prompt)
    #[arg(long)]
    pub password: Option<String>,
    /// Optional keychain account name (macOS only)
    #[arg(long)]
    pub keychain_account: Option<String>,
    /// Keychain service name (macOS only)
    #[arg(long, default_value = "ownkey")]
    pub keychain_service: String,
    /// Disable session cache usage for this command
    #[arg(long)]
    pub no_session: bool,
}

#[derive(clap::Args, Debug)]
pub struct ViewArgs {
    /// Optional path to the vault file
    #[arg(short, long)]
    pub path: Option<String>,
    /// Key to view
    pub key: String,
    /// Optional password (falls back to interactive prompt)
    #[arg(long)]
    pub password: Option<String>,
    /// Optional keychain account name (macOS only)
    #[arg(long)]
    pub keychain_account: Option<String>,
    /// Keychain service name (macOS only)
    #[arg(long, default_value = "ownkey")]
    pub keychain_service: String,
    /// Disable session cache usage for this command
    #[arg(long)]
    pub no_session: bool,
    /// Output as JSON with key and value
    #[arg(long)]
    pub json: bool,
}

#[derive(clap::Args, Debug)]
pub struct DeleteArgs {
    /// Optional path to the vault file
    #[arg(short, long)]
    pub path: Option<String>,
    /// Key to delete
    pub key: String,
    /// Optional password (falls back to interactive prompt)
    #[arg(long)]
    pub password: Option<String>,
    /// Optional keychain account name (macOS only)
    #[arg(long)]
    pub keychain_account: Option<String>,
    /// Keychain service name (macOS only)
    #[arg(long, default_value = "ownkey")]
    pub keychain_service: String,
    /// Disable session cache usage for this command
    #[arg(long)]
    pub no_session: bool,
}

#[derive(clap::Args, Debug)]
pub struct SearchArgs {
    /// Optional path to the vault file
    #[arg(short, long)]
    pub path: Option<String>,
    /// Keyword to search for
    pub keyword: String,
    /// Optional password (falls back to interactive prompt)
    #[arg(long)]
    pub password: Option<String>,
    /// Optional keychain account name (macOS only)
    #[arg(long)]
    pub keychain_account: Option<String>,
    /// Keychain service name (macOS only)
    #[arg(long, default_value = "ownkey")]
    pub keychain_service: String,
    /// Disable session cache usage for this command
    #[arg(long)]
    pub no_session: bool,
    /// Only print exact key names (no previews)
    #[arg(long)]
    pub exact: bool,
}

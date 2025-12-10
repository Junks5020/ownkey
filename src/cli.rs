use clap::{Parser, Subcommand};

pub mod sync_cmd;

#[derive(Parser, Debug)]
#[command(
    name = "ownkey",
    about = "A learning-friendly CLI password manager",
    long_about = "ownkey is a small, learning-focused CLI password manager.\n\
It stores key/value secrets in an encrypted vault on disk and is designed to showcase\n\
practical Rust patterns for CLI apps, encryption, and error handling.",
    after_long_help = "Examples:\n  ownkey init                         # Create a new encrypted vault (~/.ownkey/vault.json)\n  ownkey init ./vault.json --password testpw\n  ownkey add gj_key 123123            # Add a secret\n  ownkey list                         # List stored keys\n  ownkey view gj_key                  # View a secret by key\n  ownkey delete gj_key --yes          # Delete a key without interactive prompt\n  ownkey search gj                    # Search keys by keyword\n"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new encrypted vault
    Init(InitArgs),
    /// Add a new key/value secret to the vault
    Add(AddArgs),
    /// List all keys stored in the vault
    List(ListArgs),
    /// View a secret by key
    View(ViewArgs),
    /// Delete a secret by key
    Delete(DeleteArgs),
    /// Search secrets by keyword in key or value
    Search(SearchArgs),
    /// Restore the encrypted backup over the current vault (hidden)
    #[command(name = "restore-backup", hide = true)]
    RestoreBackup,
    /// Sync vault (placeholder)
    Sync(SyncArgs),
    /// Login to sync backend (placeholder)
    Login(LoginArgs),
    /// Logout from sync backend (placeholder)
    Logout,
}

#[derive(clap::Args, Debug)]
pub struct InitArgs {
    /// Path to the vault file (defaults to ~/.ownkey/vault.json)
    pub path: Option<String>,
    /// Optional password (falls back to interactive prompt; using --password may leak in shell history)
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
    /// Optional password (falls back to interactive prompt; using --password may leak in shell history)
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
    /// Optional password (falls back to interactive prompt; using --password may leak in shell history)
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
    /// Optional password (falls back to interactive prompt; using --password may leak in shell history)
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
    /// Optional password (falls back to interactive prompt; using --password may leak in shell history)
    #[arg(long)]
    pub password: Option<String>,
    /// Optional keychain account name (macOS only)
    #[arg(long)]
    pub keychain_account: Option<String>,
    /// Keychain service name (macOS only)
    #[arg(long, default_value = "ownkey")]
    pub keychain_service: String,
    /// Skip interactive confirmation and delete immediately
    #[arg(short = 'y', long = "yes")]
    pub yes: bool,
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
    /// Optional password (falls back to interactive prompt; using --password may leak in shell history)
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

#[derive(clap::Args, Debug)]
pub struct SyncArgs {}

#[derive(clap::Args, Debug)]
pub struct LoginArgs {
    /// Optional username for sync backend
    #[arg(long)]
    pub username: Option<String>,
}

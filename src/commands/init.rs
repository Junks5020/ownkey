use anyhow::Result;

use crate::cli::InitArgs;
use crate::vault::{ensure_vault_exists_with_password, warn_if_insecure_cli_password, PasswordOptions};
use crate::vault_store;

pub fn handle(args: InitArgs) -> Result<()> {
    let path = if let Some(ref path) = args.path {
        path.clone()
    } else {
        vault_store::default_vault_path()?
    };

    let opts = PasswordOptions {
        password: args.password.as_deref(),
        keychain_account: args.keychain_account.as_deref(),
        keychain_service: &args.keychain_service,
        vault_path: &path,
        no_session: args.no_session,
    };

    warn_if_insecure_cli_password(&opts);

    println!("Initializing vault at {}", path);
    ensure_vault_exists_with_password(&path, &opts)?;
    Ok(())
}

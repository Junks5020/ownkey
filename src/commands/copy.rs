use anyhow::{anyhow, Result};
use arboard::Clipboard;

use crate::cli::CopyArgs;
use crate::vault::{
    ensure_vault_exists_with_password, load_vault_with_password, warn_if_insecure_cli_password,
    PasswordOptions,
};
use crate::vault_store;

pub fn handle(args: CopyArgs) -> Result<()> {
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

    ensure_vault_exists_with_password(&path, &opts)?;
    let vault = load_vault_with_password(&path, &opts)?;

    let value = vault
        .entries
        .get(&args.key)
        .ok_or_else(|| anyhow!("No entry found for key {}", args.key))?;

    let mut clipboard =
        Clipboard::new().map_err(|e| anyhow!("failed to access clipboard: {e}"))?;
    clipboard
        .set_text(value.to_string())
        .map_err(|e| anyhow!("failed to copy to clipboard: {e}"))?;

    println!("Value for key '{}' copied to clipboard.", args.key);
    Ok(())
}

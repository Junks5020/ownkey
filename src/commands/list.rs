use anyhow::Result;

use crate::cli::ListArgs;
use crate::vault::{ensure_vault_exists_with_password, load_vault_with_password, warn_if_insecure_cli_password, PasswordOptions};
use crate::vault_store;

pub fn handle(args: ListArgs) -> Result<()> {
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

    for key in vault.entries.keys() {
        println!("{key}");
    }

    Ok(())
}

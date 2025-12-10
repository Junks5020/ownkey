use anyhow::{anyhow, Result};
use serde_json::json;

use crate::cli::ViewArgs;
use crate::vault::{ensure_vault_exists_with_password, load_vault_with_password, warn_if_insecure_cli_password, PasswordOptions};
use crate::vault_store;

pub fn handle(args: ViewArgs) -> Result<()> {
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

    if let Some(value) = vault.entries.get(&args.key) {
        if args.json {
            let obj = json!({
                "key": args.key,
                "value": value,
            });
            println!("{}", obj);
        } else {
            println!("{}", value);
        }
        Ok(())
    } else {
        Err(anyhow!("No entry found for key {}", args.key))
    }
}

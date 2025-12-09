use anyhow::Result;

use crate::cli::DeleteArgs;
use crate::vault::{
    ensure_vault_exists_with_password, load_vault_with_password, save_vault_with_password,
    PasswordOptions,
};
use crate::vault_store;

pub fn handle(args: DeleteArgs) -> Result<()> {
    let path = args
        .path
        .unwrap_or_else(|| vault_store::default_vault_path().expect("cannot resolve default path"));

    let opts = PasswordOptions {
        password: args.password.as_deref(),
        keychain_account: args.keychain_account.as_deref(),
        keychain_service: &args.keychain_service,
        vault_path: &path,
        no_session: args.no_session,
    };
    ensure_vault_exists_with_password(&path, &opts)?;
    let mut vault = load_vault_with_password(&path, &opts)?;

    if vault.entries.remove(&args.key).is_some() {
        save_vault_with_password(&path, &vault, &opts)?;
    } else {
        println!("No entry found for key {}", args.key);
    }

    Ok(())
}

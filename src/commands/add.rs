use anyhow::{anyhow, Result};
use rpassword::prompt_password;

use crate::cli::AddArgs;
use crate::vault::{ensure_vault_exists_with_password, load_vault_with_password, save_vault_with_password, PasswordOptions};
use crate::vault_store;

pub fn handle(args: AddArgs) -> Result<()> {
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

    let name = args.key;
    let secret = match args.value {
        Some(v) => v,
        None => {
            let first = prompt_password("Value: ").map_err(|_| anyhow!("failed to read value"))?;
            let second =
                prompt_password("Confirm: ").map_err(|_| anyhow!("failed to read value"))?;
            if first != second {
                return Err(anyhow!("Values do not match. Aborting."));
            }
            first
        }
    };
    let _notes = args.notes;

    ensure_vault_exists_with_password(&path, &opts)?;
    let mut vault = load_vault_with_password(&path, &opts)?;
    vault.entries.insert(name.clone(), secret);
    // TODO: notes are currently unused in storage; consider persisting later.
    save_vault_with_password(&path, &vault, &opts)?;

    println!("{}", name);
    Ok(())
}

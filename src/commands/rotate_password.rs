use anyhow::Result;

use crate::cli::RotatePasswordArgs;
use crate::keychain;
use crate::vault::{
    load_vault_with_password, prompt_new_password, save_vault_with_password,
    warn_if_insecure_cli_password, PasswordOptions,
};
use crate::vault_store;

pub fn handle(args: RotatePasswordArgs) -> Result<()> {
    let path = if let Some(ref path) = args.path {
        path.clone()
    } else {
        vault_store::default_vault_path()?
    };

    // First, load the existing vault with the current password/keychain/session,
    // to verify access and obtain the decrypted contents.
    let current_opts = PasswordOptions {
        password: args.password.as_deref(),
        keychain_account: args.keychain_account.as_deref(),
        keychain_service: &args.keychain_service,
        vault_path: &path,
        no_session: args.no_session,
    };

    warn_if_insecure_cli_password(&current_opts);

    let vault = load_vault_with_password(&path, &current_opts)?;

    // Determine the new password: either from CLI or via interactive prompt.
    let new_pass = if let Some(p) = args.new_password {
        p
    } else {
        prompt_new_password()?
    };

    let new_opts = PasswordOptions {
        password: Some(&new_pass),
        keychain_account: current_opts.keychain_account,
        keychain_service: current_opts.keychain_service,
        vault_path: current_opts.vault_path,
        no_session: current_opts.no_session,
    };

    // Re-encrypt and save the vault with the new password.
    save_vault_with_password(&path, &vault, &new_opts)?;

    // Update keychain entry to reflect the new password.
    if let Some(account) = current_opts.keychain_account {
        let _ = keychain::store_password(current_opts.keychain_service, account, &new_pass);
    } else {
        #[cfg(target_os = "macos")]
        {
            if let Ok(username) = std::env::var("USER") {
                let _ = keychain::store_password(current_opts.keychain_service, &username, &new_pass);
            }
        }
    }

    println!("Vault password rotated successfully.");
    Ok(())
}


use anyhow::Result;

use crate::cli::SearchArgs;
use crate::vault::{ensure_vault_exists_with_password, load_vault_with_password, warn_if_insecure_cli_password, PasswordOptions};
use crate::vault_store;

pub fn handle(args: SearchArgs) -> Result<()> {
    let path = if let Some(ref path) = args.path {
        path.clone()
    } else {
        vault_store::default_vault_path()?
    };
    let keyword = args.keyword.to_lowercase();

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

    let matches = vault
        .entries
        .iter()
        .filter(|(k, v)| {
            k.to_lowercase().contains(&keyword) || v.to_lowercase().contains(&keyword)
        });

    if args.exact {
        for (k, _) in matches {
            println!("{k}");
        }
    } else {
        for (k, v) in matches {
            let preview: String = v.chars().take(12).collect();
            println!("{k}: {preview}");
        }
    }

    Ok(())
}

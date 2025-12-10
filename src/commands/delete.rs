use std::io::{self, Write};

use anyhow::Result;

use crate::cli::DeleteArgs;
use crate::vault::{
    ensure_vault_exists_with_password, load_vault_with_password, save_vault_with_password,
    warn_if_insecure_cli_password, PasswordOptions,
};
use crate::vault_store;

pub fn handle(args: DeleteArgs) -> Result<()> {
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
    ensure_vault_exists_with_password(&path, &opts)?;
    let mut vault = load_vault_with_password(&path, &opts)?;

    warn_if_insecure_cli_password(&opts);

    if !vault.entries.contains_key(&args.key) {
        println!("No entry found for key {}", args.key);
        return Ok(());
    }

    if !args.yes {
        print!(
            "This will permanently delete key \"{}\" from {}. Proceed? [y/N]: ",
            args.key, path
        );
        io::stdout().flush().ok();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let answer = input.trim().to_lowercase();
        if answer != "y" && answer != "yes" {
            println!("Delete cancelled.");
            return Ok(());
        }
    }

    vault.entries.remove(&args.key);
    save_vault_with_password(&path, &vault, &opts)?;
    println!("Deleted key {}", args.key);

    Ok(())
}

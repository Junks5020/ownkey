use anyhow::Result;

use crate::vault_store;

pub fn handle() -> Result<()> {
    println!("This will overwrite your existing vault. Continue? (y/N)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if !input.trim().eq_ignore_ascii_case("y") {
        return Ok(());
    }
    vault_store::restore_backup()?;
    println!("Backup restored.");
    Ok(())
}

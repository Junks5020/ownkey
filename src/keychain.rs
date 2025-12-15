#[cfg(target_os = "macos")]
mod platform {
    use anyhow::Result;
    use security_framework::passwords::{
        delete_generic_password, get_generic_password, set_generic_password,
    };

    pub fn store_password(service: &str, account: &str, password: &str) -> Result<()> {
        set_generic_password(service, account, password.as_bytes())?;
        Ok(())
    }

    pub fn retrieve_password(service: &str, account: &str) -> Result<Option<String>> {
        match get_generic_password(service, account) {
            Ok(bytes) => Ok(String::from_utf8(bytes).ok()),
            Err(err) => {
                // If the item is not found, return None, otherwise bubble up error.
                let code = err.code();
                if code == -25300 {
                    return Ok(None);
                }
                Err(err.into())
            }
        }
    }

    #[allow(dead_code)]
    pub fn delete_password(service: &str, account: &str) -> Result<()> {
        let _ = delete_generic_password(service, account);
        Ok(())
    }
}

#[cfg(not(target_os = "macos"))]
mod platform {
    use anyhow::Result;

    pub fn store_password(_service: &str, _account: &str, _password: &str) -> Result<()> {
        anyhow::bail!("Keychain backend is only available on macOS");
    }

    pub fn retrieve_password(_service: &str, _account: &str) -> Result<Option<String>> {
        anyhow::bail!("Keychain backend is only available on macOS");
    }

    pub fn delete_password(_service: &str, _account: &str) -> Result<()> {
        anyhow::bail!("Keychain backend is only available on macOS");
    }
}

pub use platform::{retrieve_password, store_password};

// delete_password is available but not currently used by the CLI.
// Keeping the implementation for future use (e.g., keychain cleanup commands).
#[allow(unused_imports)]
pub use platform::delete_password;


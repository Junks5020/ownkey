use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vault {
    pub entries: HashMap<String, String>,
}

impl Default for Vault {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

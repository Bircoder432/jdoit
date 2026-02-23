use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::cli::UserArg;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Pattern {
    Regex(String),
    Text(String),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Hook {
    pub pattern: Pattern,
    pub command: String,
    pub help: String,
    pub args: Vec<String>,
    pub flags: Vec<String>,
}

impl Hook {
    pub fn check_flags(&self, flags: Vec<UserArg>) -> bool {
        let user_flags: Vec<&String> = flags
            .iter()
            .filter_map(|f| match f {
                UserArg::Flag(f, _) => Some(f),
                _ => None,
            })
            .collect();

        if user_flags.is_empty() {
            return true;
        }

        for flag in user_flags {
            if !self.flags.contains(flag) {
                return false;
            }
        }
        true
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct HookKit {
    pub name: String,
    pub description: String,
    pub hooks: HashMap<String, Hook>,
}

impl HookKit {
    pub fn from_string(string: String) -> Result<Self, String> {
        match toml::from_str::<Self>(&string) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn from_file(path: &str) -> Result<Self, String> {
        let content = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        Self::from_string(content)
    }
}

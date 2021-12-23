use super::*;

pub use std::env::set_var as set_env;
pub use std::env::var as env;
pub use std::env::VarError as EnvVarError;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    #[serde(rename = "development")]
    Development,

    #[serde(rename = "production")]
    Production,
}

derive_display_from_serialize!(Environment);
derive_fromstr_from_deserialize!(Environment);

#[cfg(debug_assertions)]
impl Default for Environment {
    fn default() -> Self {
        Environment::Development
    }
}

#[cfg(not(debug_assertions))]
impl Default for Environment {
    fn default() -> Self {
        Environment::Production
    }
}

impl Environment {
    pub fn from_env() -> Result<Self> {
        let name = env_opt("ENV")?;
        let name = match name {
            Some(name) => name,
            None => return Ok(default()),
        };
        let env = Self::from_str(&name)?;
        Ok(env)
    }
}

pub fn env_opt(key: &str) -> Result<Option<String>, EnvVarError> {
    match env(key) {
        Ok(value) => Ok(Some(value)),
        Err(EnvVarError::NotPresent) => Ok(None),
        Err(error) => Err(error),
    }
}

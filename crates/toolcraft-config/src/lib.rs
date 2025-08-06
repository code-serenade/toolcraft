pub mod error;

use config::{Config, File};
use serde::de::DeserializeOwned;

use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub fn load_settings<T>(config_path: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let config = Config::builder()
        .add_source(File::with_name(config_path))
        .build()
        .map_err(|e| Error::ErrorMessage(e.to_string().into()))?;

    let r = config
        .try_deserialize()
        .map_err(|e| Error::ErrorMessage(e.to_string().into()))?;
    Ok(r)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct Settings {
        pub test: TestConfig,
    }

    #[derive(Debug, Deserialize)]
    pub struct TestConfig {
        pub test_key: String,
    }

    #[test]
    fn test_load_settings() {
        let config_path = "tests/test_config.toml"; // Adjust the path as needed
        let result: Result<Settings> = load_settings(config_path);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.test.test_key, "test_value");
    }
}

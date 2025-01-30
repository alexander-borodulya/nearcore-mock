use serde::Deserialize;

const DEFAULT_CONFIG_FILE: &str = "config.json";

#[derive(Debug, Deserialize)]
struct Config {
    version: String,
}

pub fn info() -> String {
    // Read configuration file
    let config_file = std::env::var("CONFIG_FILE").unwrap_or_else(|_| DEFAULT_CONFIG_FILE.to_string());
    let config_str = std::fs::read_to_string(config_file).expect("Failed to read config file");

    // Parse JSON configuration
    let config: Config = serde_json::from_str(&config_str).expect("Failed to parse config JSON");

    config.version
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info() {
        let version = info();
        assert!(!version.is_empty());
    }

    #[test]
    fn test_version_is_0_0_0() {
        let version = info();
        assert_eq!(version, "0.0.0");
    }
}

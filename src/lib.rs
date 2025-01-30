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

pub fn git_tag() -> String {
    // Assuming the current git tag is stored in a file named ".git/HEAD"
    let git_tag_file = std::path::Path::new(".git").join("HEAD");
    let git_tag_str = std::fs::read_to_string(git_tag_file).expect("Failed to read.git/HEAD");

    // Extract the git tag from the file content
    let git_tag = git_tag_str.trim_start_matches("ref: refs/heads/");

    git_tag.to_string()
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
    fn test_version_is_0_1_0() {
        let version = info();
        assert_eq!(version, "0.1.0");
    }

    #[test]
    fn test_git_tag_matches_config_version() {
        let version = info();
        let git_tag = git_tag();
        assert_eq!(version, git_tag);
    }
}

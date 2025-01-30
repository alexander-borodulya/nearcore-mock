use std::fs::DirEntry;

use serde::Deserialize;

const DEFAULT_CONFIG_FILE: &str = "config.json";

// Embed the contents of the config.json file at compile time
const CONFIG_JSON: &str = include_str!("../config.json");

#[derive(Debug, Deserialize)]
struct Config {
    version: String,
}

pub fn info() -> String {
    // Read configuration file
    let config_file = std::env::var("CONFIG_FILE").unwrap_or_else(|_| DEFAULT_CONFIG_FILE.to_string());
    let config_str = std::fs::read_to_string(config_file).unwrap_or_else(|err| {
        println!("Using default precompile config JSON because of error: {err}");
        CONFIG_JSON.to_string()
    });

    // Parse JSON configuration
    let config: Config = serde_json::from_str(&config_str).expect("Failed to parse config JSON");

    config.version
}

pub fn git_tag() -> String {
    // Assuming the current git tag is a filename of the file stored in a directory named ".git/refs/tags"
    let git_tag_dir = std::path::Path::new(".git").join("refs/tags");
    let tags_dir = std::fs::read_dir(git_tag_dir).expect("Failed to read.git/refs/tags");
    let all_tags = tags_dir.filter_map(|e| e.ok()).collect::<Vec<DirEntry>>();
    let git_tag = all_tags.last().map(|e| e.file_name()).map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "unknown".to_string());
    git_tag
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

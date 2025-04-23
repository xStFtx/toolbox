use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Default)]
pub struct ToolboxConfig {
    pub organize: Option<OrganizeConfig>,
}

#[derive(Deserialize, Debug, Default)]
pub struct OrganizeConfig {
    pub default_mode: Option<String>,
    pub custom_rules: Option<Vec<OrganizeRule>>,
}

#[derive(Deserialize, Debug)]
pub struct OrganizeRule {
    pub pattern: String, // e.g., "*.pdf"
    pub target_dir: String, // e.g., "Docs/"
}

impl ToolboxConfig {
    pub fn load() -> Self {
        let mut config_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        config_path.push("toolbox/config.toml");
        if let Ok(content) = fs::read_to_string(&config_path) {
            toml::from_str(&content).unwrap_or_default()
        } else {
            ToolboxConfig::default()
        }
    }
}

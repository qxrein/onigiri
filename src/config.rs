use anyhow::{anyhow, Result};  
use serde::Deserialize;
use std::path::Path;  
use std::process::Command;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct ThemeConfig {
    pub background: String,
    pub foreground: String,
    pub error_color: String,
    pub font: String,
}

impl fmt::Display for ThemeConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Theme(bg: {}, fg: {}, error: {}, font: {})",
            self.background, self.foreground, self.error_color, self.font
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct SessionConfig {
    pub name: String,
    pub command: String,
}

#[derive(Debug, Deserialize)]
pub struct OnigiriConfig {
    pub theme: ThemeConfig,
    pub sessions: Vec<SessionConfig>,
}

#[cfg(feature = "nix_config")]
pub fn load_nix_config(path: &Path) -> Result<OnigiriConfig> {
    let output = Command::new("nix")
        .args(["eval", "--json", "--file", path.to_str().unwrap()])
        .output()?;
    
    if !output.status.success() {
        return Err(anyhow!("Nix evaluation failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    
    let config: OnigiriConfig = serde_json::from_slice(&output.stdout)?;
    Ok(config)
}

#[cfg(not(feature = "nix_config"))]
pub fn load_nix_config(_path: &Path) -> Result<OnigiriConfig> {
    Err(anyhow!("Nix config support not compiled in"))
}

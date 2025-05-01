use crate::config::ThemeConfig;
use anyhow::Result;

pub mod prompt;
pub mod theme;

pub struct LoginUI {
    theme: ThemeConfig,
}

impl LoginUI {
    pub fn new(theme: ThemeConfig) -> Result<Self> {
        println!("Using theme: {}", theme);
        Ok(Self { theme })
    }

    pub async fn display_login_prompt(&mut self) -> Result<(String, String)> {
        Ok(("user".to_string(), "pass".to_string()))
    }

    pub async fn display_error(&mut self, message: &str) -> Result<()> {
        eprintln!("Error: {}", message);
        Ok(())
    }
}

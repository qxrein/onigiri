mod config;
mod session;
mod ui;

use anyhow::Result;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = PathBuf::from("./config.nix");
    let config = config::load_nix_config(&config_path)?;
    
    let mut ui = ui::LoginUI::new(config.theme)?;
    
    loop {
        let credentials = ui.display_login_prompt().await?;
        if let Some(session) = session::authenticate(&credentials).await? {
            session.start().await?;
            break;
        }
        ui.display_error("Invalid credentials").await?;
    }
    
    Ok(())
}

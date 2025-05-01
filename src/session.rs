use anyhow::Result;

pub struct Session {
    pub command: String,
}

impl Session {
    pub async fn start(&self) -> Result<()> {
        println!("Starting session with command: {}", self.command);
        Ok(())
    }
}

pub async fn authenticate(credentials: &(String, String)) -> Result<Option<Session>> {
    let (username, _password) = credentials; // Prefix with underscore to silence warning
    println!("Authenticating user: {}", username);
    Ok(Some(Session {
        command: "exec i3".to_string(),
    }))
}

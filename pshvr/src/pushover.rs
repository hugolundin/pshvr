use std::error::Error;

extern crate dirs;

extern crate envfile;
use envfile::EnvFile;

pub struct Pushover {
    user: String,
    token: String
}

impl Pushover {
    pub fn new(user: String, token: String) -> Self {
        Pushover {
            user,
            token
        }
    }

    pub fn from_config() -> Self {
        let path = dirs::home_dir()
            .expect("Unable to open home directory.")
            .join("pshvrc");

        let config = EnvFile::new(&path)
            .expect("~/.pshvrc does not exist.");

        let user = config.get("user")
            .expect("'user' does not exist in ~/.pshvrc");

        let token = config.get("token")
            .expect("'token' does not exist in ~/.pshvrc");
    
        Pushover::new(user.to_string(), token.to_string())
    }

    pub fn push(&self, message: &str) -> Result<(), Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();

        let result = client.post("https://api.pushover.net/1/messages.json")
            .query(&[("token", &self.token[..]), ("user", &self.user[..]), ("message", message)])
            .send()?;



        Ok(())
    }
}
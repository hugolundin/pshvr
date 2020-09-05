use std::error::Error;

extern crate dirs;
use dirs::home_dir;

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
        let home = match home_dir() {
            Some(d) => d,
            _ => panic!("Unable to open home directory.")
        };
    
        let config = match EnvFile::new(&home.join(".pshvr")) {
            Ok(c) => c, 
            _ => panic!("~/.pshvr does not exist.")
        };
    
        let user = match config.get("user") {
            Some(u) => u,
            _ => panic!("'user' does not exist in ~/.pshvr")
        };
    
        let token = match config.get("token") {
            Some(t) => t,
            _ => panic!("'token' does not exist in ~/.pshvr")
        };
    
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
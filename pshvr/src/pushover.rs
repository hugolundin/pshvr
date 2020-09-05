use std::error::Error;

pub struct Pushover<'a> {
    token: &'a str,
    user: &'a str
}

impl<'a> Pushover<'a> {
    pub fn new(token: &'a str, user: &'a str) -> Self {
        Pushover {
            token,
            user
        }
    }

    pub async fn push(&self, message: &str) -> Result<(), Box<dyn Error>>{
        let client = reqwest::Client::new();
        let result = client.post("https://api.pushover.net/1/messages.json")
            .query(&[("token", self.token), ("user", self.user), ("message", message)])
            .send()
            .await?;

        println!("{:#?}", result);

        Ok(())
    }
}
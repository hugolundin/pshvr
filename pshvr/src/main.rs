mod pushover;

use pushover::Pushover;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let pushover = Pushover::new("", "");
    pushover.push("hello from rust").await;
    Ok(())
}

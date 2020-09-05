extern crate log;
mod pushover;
use pushover::Pushover;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pushover = Pushover::from_config();
    pushover.push("hello from rust")?;

    Ok(())
}

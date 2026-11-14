use anyhow::{Result, bail};

use clap::Parser;
use weather::OpenMeteo;

#[derive(Parser)]
/// Shows the current weather for a given location.
struct Args {
    #[arg(required = true)]
    /// Example: "Tokyo"
    location: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let location = args.location.join(" ");

    if location.is_empty() {
        bail!("Usage: weather <LOCATION>")
    }

    let weather_client = OpenMeteo::new();
    let weather = weather_client.get_weather_by_location(&location)?;

    println!("{weather:#?}");
    Ok(())
}

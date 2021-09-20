#[path = "use-cases/solar.rs"]
mod use_cases;
mod types;

use serde_json::Result;

use futures::executor::block_on;

#[tokio::main]
async fn main() -> Result<()> {
    println!("starting program");
    let future = use_cases::solar::get_solar_times();
    let solar = block_on(future).unwrap();

    println!("today the sun rose at {}", solar.forecast.forecastday[0].astro.sunrise);
    println!("today the sun will set at {}", solar.forecast.forecastday[0].astro.sunset);

    println!("tomorrow the sun will rise at {}", solar.forecast.forecastday[1].astro.sunrise);
    println!("tomorrow the sun will set at {}", solar.forecast.forecastday[1].astro.sunset);

    Ok(())
}
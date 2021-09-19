mod types;

use futures::executor::block_on;

use serde_json::Result;

use hyper_tls::HttpsConnector;
use hyper::Client;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    println!("starting program");
    let future = get_solar_times();
    let solar = block_on(future).unwrap();

    println!("today the sun rose at {}", solar.forecast.forecastday[0].astro.sunrise);
    println!("today the sun will set at {}", solar.forecast.forecastday[0].astro.sunset);

    println!("tomorrow the sun will rise at {}", solar.forecast.forecastday[1].astro.sunrise);
    println!("tomorrow the sun will set at {}", solar.forecast.forecastday[1].astro.sunset);

    Ok(())
}

async fn get_solar_times() -> std::result::Result<types::ResponseBody, hyper::Error> {
    println!("initializing client");
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let uri = String::from(format!("https://api.weatherapi.com/v1/forecast.json?key={}&q=84095&days=2&aqi=no&alerts=no", dotenv::var("WEATHER_API_KEY").unwrap()));

    println!("making request");
    let resp = client.get(uri.parse().unwrap()).await;

    match resp {
        Ok(data) => {
            println!("request received");
            let body = hyper::body::to_bytes(data.into_body()).await.unwrap();

            let parsed = String::from_utf8(body.to_vec()).unwrap();

            let json: types::ResponseBody = serde_json::from_str(&parsed).unwrap();

            Ok(json)
        },
        Err(err) => {
            println!("something broke {}", err);
            Err(err)
        }
    }
}
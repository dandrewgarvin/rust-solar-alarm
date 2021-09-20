pub mod types {
  use serde::Deserialize;
  
  #[derive(Deserialize)]
  pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f32,
    pub lon: f32,
    pub localtime: String
  }

  #[derive(Deserialize)]
  pub struct Astro {
    pub sunrise: String,
    pub sunset: String,
    pub moonrise: String,
    pub moonset: String
  }

  #[derive(Deserialize)]
  pub struct ForecastDay {
    pub date: String,
    pub astro: Astro
  }

  #[derive(Deserialize)]
  pub struct Forecast {
    pub forecastday: Vec<ForecastDay>
  }

  #[derive(Deserialize)]
  pub struct ResponseBody {
    pub location: Location,
    pub forecast: Forecast
  }
}
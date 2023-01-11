use crate::models::assistant::{WeatherEntities,WeatherApiResponse};
use super::reqwest;
use std::env;

use crate::error::AppError;

pub async fn call_weather_api(input: crate::web::Json<WeatherEntities>) -> Result<WeatherApiResponse,AppError> {

    let entities = WeatherEntities {
        city : input.city.clone(),
        units : input.units.clone(),
    };

    let endpointish = "https://api.openweathermap.org/data/2.5/weather?".to_string();
    let api_token = "appid=".to_string() + &env::var("OPENWEATHERAPITOKEN").
        expect("OPENWEATHERAPITOKEN incorrect");
    let par_q = "&q=".to_string() + &entities.city.to_string();
    let par_units = "&units=".to_string() + &entities.units.to_string();
    let url = endpointish + &api_token + &par_q + &par_units;

    let client = reqwest::Client::new();
    let response : WeatherApiResponse = client
        .get(url)
        .send()
        .await
        .expect("Failed to get response")
        .json()
        .await
        .expect("Failed to get data");
    
    Ok(response)
}

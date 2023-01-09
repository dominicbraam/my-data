use crate::models::assistant::{WeatherEntities,WeatherApiResponse};
use super::reqwest;
use actix_web::Error;
use std::env;

pub async fn call_weather_api(input: crate::web::Json<WeatherEntities>) -> Result<String,Error> {

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
    
    let response = "In ".to_string()
        + &response.name.to_string()
        + ", it is currently "
        + &response.main.temp.to_string()
        + " degress Celcius"
        + " and feels like "
        + &response.main.feels_like.to_string()
        + ". "
        + &response.weather.weather.description.to_string()
        + " is to be expected.";

    Ok(response)
}

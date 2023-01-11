use crate::models::assistant::{WeatherEntities,WeatherApiResponse};
use super::reqwest;
use std::env;

use crate::error::AppError;

pub async fn call_weather_api(input: crate::web::Json<WeatherEntities>) -> Result<WeatherApiResponse,AppError> {

    let entities = WeatherEntities {
        city : input.city.clone(),
        units : input.units.clone(),
    };

    let api_token = match env::var("OPENWEATHERAPITOKEN"){
        Ok(v) => v.to_string(),
        Err(e) => {
            log::error!("Open Weather API token: {}", e);
            e.to_string()
        }
    };

    let endpointish = "https://api.openweathermap.org/data/2.5/weather?".to_string();
    let url_api_token = "appid=".to_string() + &api_token;
    let par_q = "&q=".to_string() + &entities.city.to_string();
    let par_units = "&units=".to_string() + &entities.units.to_string();
    let url = endpointish + &url_api_token + &par_q + &par_units;

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await;

    match response {
        Ok(res) => {
            let response : WeatherApiResponse = res.json().await?;
            log::info!("{:?}",response);
            Ok(response)
        },
        Err(e) => {
            log::error!("{}",e);
            panic!("{}",e)
        }
    }
}

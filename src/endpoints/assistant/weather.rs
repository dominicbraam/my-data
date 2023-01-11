use crate::actix_web::{web,get,HttpResponse};

use crate::models::assistant::{WeatherEntities};
use crate::external::weather;
use crate::error::AppError;

#[get("/assistant/weather")]
pub async fn get_weather(data: web::Json<WeatherEntities>) -> Result<HttpResponse, AppError> {

    let weather_res = weather::call_weather_api(data).await;

    weather_res.map(|tags| HttpResponse::Ok().json(tags))
}

use crate::actix_web::{web,get,HttpResponse,Error};

use crate::models::assistant::{WeatherEntities,AssistantResponse};
use crate::external::weather;

#[get("/assistant/weather")]
pub async fn get_weather(data: web::Json<WeatherEntities>) -> Result<HttpResponse, Error> {

    let weather_res = AssistantResponse{
        message: weather::call_weather_api(data)
            .await
            .expect("Did not get data from API"),
    };

    println!("{}",weather_res);

    if !weather_res.message.is_empty() {
        Ok(HttpResponse::Ok().json(weather_res))
    } else {
        let res = HttpResponse::NotFound().body(format!("Did not get data from API"));
        Ok(res)
    }
}

use serde::{Deserialize,Serialize};

//#[derive(Serialize,Deserialize,Queryable)]
//pub struct Person {
//    pub id: i32,
//    pub username: String,
//    pub first_name: String,
//    pub last_name: String,
//    pub dob: chrono::NaiveDate,
//}
//
//#[derive(Serialize,Deserialize,Insertable)]
//#[diesel(table_name = person)]
//pub struct NewPerson<'a> {
//    pub username: &'a str,
//    pub first_name: &'a str,
//    pub last_name: &'a str,
//    pub dob: chrono::NaiveDate,
//}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherEntities {
    pub city: String,
    pub units: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherApiResponse {
    pub weather: WeatherApiResponseWeatherList,
    pub base: String,
    pub main: WeatherApiResponseMain,
    pub visibility: i32,
    pub timezone: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherApiResponseWeatherList {
    pub weather: WeatherApiResponseWeather,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherApiResponseWeather {
    pub main: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherApiResponseMain {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GreetEntities {
    pub greeting: String,
}

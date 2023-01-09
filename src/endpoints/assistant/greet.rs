use crate::actix_web::{web,get,HttpResponse,Error};

use crate::models::assistant::GreetEntities;
use crate::models::assistant::AssistantResponse;

#[get("/assistant/greet")]
pub async fn greet(data: web::Json<GreetEntities>) -> Result<HttpResponse, Error> {

    let greet_res = AssistantResponse{
        //message: data.greeting.clone(),
        message: "Hello".to_string(),
    };

    if !greet_res.message.is_empty() {
        Ok(HttpResponse::Ok().json(greet_res))
    } else {
        let res = HttpResponse::NotFound().body(format!("Error called in greet endpoint"));
        Ok(res)
    }
}

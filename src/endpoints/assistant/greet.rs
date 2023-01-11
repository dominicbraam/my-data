use crate::actix_web::{web,get,HttpResponse};
use crate::models::assistant::{GreetEntities,AssistantResponse};
use crate::error::AppError;

#[get("/assistant/greet")]
pub async fn greet(data: web::Json<GreetEntities>) -> Result<HttpResponse, AppError> {

    let greet_res = AssistantResponse{
        message: data.greeting.clone(),
    };

    Ok(HttpResponse::Ok().json(greet_res))
}

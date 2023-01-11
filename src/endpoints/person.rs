use crate::actix_web::{web,get,post,HttpResponse};
use crate::database::ops;
use crate::models;
use super::DbPool;

use crate::error::AppError;

#[get("/person")]
pub async fn get_persons(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {

    let persons = web::block(move || {
        let mut conn = pool.get()?;
        ops::person::pull_persons(&mut conn)
    })
    .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(persons))
}

#[post("/person")]
pub async fn create_person(pool: web::Data<DbPool>, body: web::Json<models::person::InputPersonHandler>) -> Result<HttpResponse, AppError> {

    let size = web::block(move || {
        let mut conn = pool.get()?;
        ops::person::push_person(&mut conn, body)
    })
    .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(size))
}

use crate::actix_web::{web,get,post,HttpResponse,Error};
use crate::ops;
use crate::models;
use super::DbPool;

#[get("/person")]
pub async fn get_persons(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {

    let persons = web::block(move || {
        let mut conn = pool.get()?;
        ops::person::pull_persons(&mut conn)
    })
    .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if persons.len() > 0 {
        Ok(HttpResponse::Ok().json(persons))
    } else {
        let res = HttpResponse::NotFound().body(format!("No persons found"));
        Ok(res)
    }
}

#[post("/person")]
pub async fn create_person(pool: web::Data<DbPool>, body: web::Json<models::person::InputPersonHandler>) -> Result<HttpResponse, Error> {

    let size = web::block(move || {
        let mut conn = pool.get()?;
        ops::person::push_person(&mut conn, body)
    })
    .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
        //.map(|person| HttpResponse::Created().json(person))
        //.map_err(|_| HttpResponse::InternalServerError())?;

    Ok(HttpResponse::Ok().json(size))
}

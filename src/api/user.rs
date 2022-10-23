use crate::actix_web::{web,get,post,HttpResponse,Error};
use crate::ops;
use super::DbPool;

#[get("/user")]
pub async fn grab_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {

    let users = web::block(move || {
        let mut conn = pool.get()?;
        ops::user::pull_users(&mut conn)
    })
    .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if users.len() > 0 {
        Ok(HttpResponse::Ok().json(users))
    } else {
        let res = HttpResponse::NotFound().body(format!("No users found"));
        Ok(res)
    }
}

#[post("/user")]
pub async fn create_user(pool: web::Data<DbPool>, body: web::Json<crate::models::InputUserHandler>) -> Result<HttpResponse, Error> {

    let size = web::block(move || {
        let mut conn = pool.get()?;
        ops::user::push_user(&mut conn, body)
    })
    .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
        //.map(|user| HttpResponse::Created().json(user))
        //.map_err(|_| HttpResponse::InternalServerError())?;

    Ok(HttpResponse::Ok().json(size))
}

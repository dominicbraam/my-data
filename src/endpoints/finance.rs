use crate::actix_web::{web,get,post,HttpResponse,Error};
use crate::ops;
use crate::models;
use super::DbPool;

#[get("/finance/incexp")]
pub async fn get_incexps(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {

    let incexps = web::block(move || {
        let mut conn = pool.get()?;
        ops::finance::pull_incexps(&mut conn)
    })
    .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if incexps.len() > 0 {
        Ok(HttpResponse::Ok().json(incexps))
    } else {
        let res = HttpResponse::NotFound().body(format!("No data found"));
        Ok(res)
    }
}

#[post("/finance/incexp/add")]
pub async fn create_finance_incexp(pool: web::Data<DbPool>, body: web::Json<models::finance::InputFinanceIncExpHandler>) -> Result<HttpResponse, Error> {

    let size = web::block(move || {
        let mut conn = pool.get()?;
        ops::finance::push_incexp(&mut conn, body)
    })
    .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
        //.map(|person| HttpResponse::Created().json(person))
        //.map_err(|_| HttpResponse::InternalServerError())?;

    Ok(HttpResponse::Ok().json(size))
}

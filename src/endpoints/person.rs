use crate::actix_web::{web,get,post,delete,HttpResponse};
use super::DbPool;
use crate::database::ops;
use crate::models::person::{Person,NewPerson};
use crate::schema::persons::dsl::*;

use crate::error::AppError;

#[get("/person/{uid}")]
pub async fn get_person_by_id(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    ) -> Result<HttpResponse, AppError>
{
    let uid = path.into_inner();

    let results: Result<Person, AppError> = web::block(move || {
        let mut conn = pool.get()?;
        ops::fetch_item_by_pk(&mut conn, persons, uid)
    })
    .await
    .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(results))
}

#[post("/person")]
pub async fn create_person(
    pool: web::Data<DbPool>,
    body: web::Json<NewPerson>
    ) -> Result<HttpResponse, AppError>
{
    let new_person: Result<Person, AppError> = web::block(move || {
        let mut conn = pool.get()?;
        ops::insert_into_table(&mut conn, persons, body.into_inner())
    })
    .await
    .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(new_person))
}

// #[delete("/person/{uid}")]
// pub async fn delete_person(
//     pool: web::Data<DbPool>,
//     path: web::Path<i32>
//     ) -> Result<HttpResponse, AppError>
// {
//     let uid = path.into_inner();
//
//     let new_person: Result<usize, AppError> = web::block(move || {
//         let mut conn = pool.get()?;
//         ops::delete_row_by_pk(&mut conn, persons, uid)
//     })
//     .await
//     .map_err(AppError::from)?;
//
//     Ok(HttpResponse::Ok().json(new_person))
// }

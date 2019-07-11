use actix_web::{HttpResponse, Result, web};
use crate::handlers::pg_pool_handler;
use crate::db_connection::PgPool;
use crate::models::ques_ans::{QA, NewQA, QAlist};
use crate::handlers::LoggedUser;

pub fn index(_user: LoggedUser, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(QAlist::list(&pg_pool)))
}

pub fn create(user: LoggedUser, new_qa: web::Json<NewQA> ,pool: web::Data<PgPool>) ->Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;

    new_qa
        .create(user.id, &pg_pool)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}


pub fn show(_user: LoggedUser, id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse>{
    let pg_pool = pg_pool_handler(pool)?;
    QA::find(&id, &pg_pool)
        .map(|qa| HttpResponse::Ok().json(qa))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })

}


pub fn destroy(_user: LoggedUser, id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    QA::destroy(&id, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}


pub fn update(_user: LoggedUser, id: web::Path<i32>, new_qa: web::Json<NewQA>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    QA::update(&id, &new_qa, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}


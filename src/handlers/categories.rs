use actix_web::{HttpResponse, Result, web};

use crate::models::categorie::{Categorie, CategorieList, NewCategorie};
use crate::db_connection::PgPool;
use crate::handlers::pg_pool_handler;
use crate::handlers::LoggedUser;




pub fn index(_user: LoggedUser, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(CategorieList::list(&pg_pool)))
}


pub fn create(user: LoggedUser, new_comment: web::Json<NewCategorie> ,pool: web::Data<PgPool>) ->Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;

    new_comment
        .create(user.id, &pg_pool)
        .map(|comment| HttpResponse::Ok().json(comment))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}

pub fn show(_user: LoggedUser, id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Categorie::find(&id, &pg_pool)
        .map(|categories| HttpResponse::Ok().json(categories))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}


pub fn update(_user: LoggedUser, id: web::Path<i32>,  new_categorie: web::Json<NewCategorie>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Categorie::update(&id, &new_categorie, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}

pub fn destroy(_user: LoggedUser, id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Categorie::destroy(&id, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}
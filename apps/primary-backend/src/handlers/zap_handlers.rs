use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use database::DbPool;
use crate::types::zap_types::ZapCreateSchema;

pub async fn create_zap(pool: web::Data<DbPool>, form: web::Json<ZapCreateSchema>) -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "Zap created successfully" }))
}

pub async fn get_zap(pool: web::Data<DbPool>) -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "Zap fetched successfully" }))
}

pub async fn get_zap_by_id(pool: web::Data<DbPool>, zap_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "Zap by id fetched successfully" }))
}
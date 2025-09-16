use actix_web::{web, HttpResponse, Responder, web::ReqData};
use serde_json::json;
use database::schema::users::dsl::*;
use database::{DbPool, models::user::User};
use diesel::prelude::*;
use uuid::Uuid;



pub async fn get_user(
    pool: web::Data<DbPool>,
    user_id: ReqData<Uuid>,
) -> impl Responder {
    
    let conn = &mut pool.get().expect("couldn't get db connection from pool");
    match users.filter(id.eq(user_id.into_inner())).first::<User>(conn) {
        Ok(user) => {
            HttpResponse::Ok().json(json!({ "message": "Get successful", "user": {"id": user.id, "username": user.username, "email": user.email} }))
        }
        Err(_) => {
            HttpResponse::NotFound().json(json!({ "message": "User not found" }))
        }
    }
}
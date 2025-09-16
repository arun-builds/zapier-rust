use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use database::schema::users::dsl::*;
use database::{DbPool, models::user::User};
use diesel::prelude::*;use uuid::Uuid;



pub async fn get_user(
    path: web::Path<String>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let user_email = path.into_inner();
    let conn = &mut pool.get().expect("couldn't get db connection from pool");
    match users.filter(email.eq(user_email)).first::<User>(conn) {
        Ok(user) => {
            HttpResponse::Ok().json(json!({ "message": "Get successful", "user": user }))
        }
        Err(_) => {
            HttpResponse::NotFound().json(json!({ "message": "User not found" }))
        }
    }
}
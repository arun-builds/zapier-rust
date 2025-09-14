use log::info;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;






pub async fn signup_user (
   
) -> impl Responder  {
    log::info!("I'am user signup handler!");
    HttpResponse::Ok()
    .json(json!({ "message": "Signup successful" }))
    

   
}

pub async fn login_user() -> impl Responder  {
    log::info!("I'am user login handler!");
    HttpResponse::Ok()
    .json(json!({ "message": "Login successful" }))
}

pub async fn get_user() -> impl Responder  {
    log::info!("I'am user get handler!");
    HttpResponse::Ok()
    .json(json!({ "message": "Get successful" }))
}
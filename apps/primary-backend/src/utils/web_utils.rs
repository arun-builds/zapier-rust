use actix_web::{web, dev::ServiceRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use database::{DbPool, models::user::User};
use diesel::prelude::*;
use database::schema::user::dsl::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::env;
use log::info;

pub fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set in environment")
}

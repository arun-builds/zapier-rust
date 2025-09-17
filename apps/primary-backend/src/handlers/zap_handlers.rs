use crate::types::zap_types::ZapCreateSchema;
use actix_web::{HttpResponse, Responder, web, web::ReqData};
use database::DbPool;
use database::schema::zap::dsl::*;
use database::{
    models::zap::{Trigger, Zap},
    schema::{ trigger::dsl::*},
};
use diesel::prelude::*;
use serde_json:: json;
use uuid::Uuid;


pub async fn create_zap(
    pool: web::Data<DbPool>,
    body: web::Json<ZapCreateSchema>,
    user_id_from_req: ReqData<Uuid>,
) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    let pre_generated_trigger_id = Uuid::new_v4();

    // TODO: change trigger_id to available_trigger_id in db

    conn.transaction(|conn| {
        let new_zap = Zap {
            id: Uuid::new_v4(),
            user_id: user_id_from_req.into_inner(),
            trigger_id: pre_generated_trigger_id,
        };

        diesel::insert_into(zap).values(&new_zap).execute(conn)?;

        let new_trigger = Trigger {
            id: pre_generated_trigger_id,
            zap_id: new_zap.id,
            trigger_id: body.available_trigger_id.clone(),
            metadata: body.trigger_metadata.clone().unwrap_or_else(|| json!({})),
        };
        diesel::insert_into(trigger)
            .values(&new_trigger)
            .execute(conn)?;

        diesel::result::QueryResult::Ok(())
    })
    .map_err(|_| {
        // TODO: log error
        HttpResponse::InternalServerError().json(json!({ "error": "Failed to create zap" }))
    })
    .expect("Failed to create zap");

    HttpResponse::Ok().json(json!({ "message": "Zap created successfully", "zap": body }))
}

pub async fn get_zap(pool: web::Data<DbPool>) -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "Zap fetched successfully" }))
}

pub async fn get_zap_by_id(
    pool: web::Data<DbPool>,
    zap_id_from_path: web::Path<Uuid>,
) -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "Zap by id fetched successfully" }))
}

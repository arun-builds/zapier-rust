use axum:: {
    extract::Path,
    routing::post,
    Router,
};

use serde_json::Value as JsonValue;

use database::db::Database;

#[tokio::main]
async fn main() {
 
   
    let app = Router::new().route("/hooks/catch/{user_id}/{zap_id}", post(zap_handler));

    // run  app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn zap_handler(Path((user_id, zap_id)): Path<(String, String)>){

    // TODO: Store the trigger in the database
    println!("user_id: {}, zap_id: {}", user_id, zap_id);

    let mut db = Database::default().unwrap();
    let create_zap = db.create_zap(zap_id, JsonValue::Null);
    println!("create_zap: {:?}", create_zap);

    // TODO: Push the trigger to the queue(Kafka/Redis)

    
}
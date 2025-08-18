use axum:: {
    extract::Path,
    routing::post,
    Router,
};

use database::Database;

#[tokio::main]
async fn main() {
    // build our application with a single route

   
    let app = Router::new().route("/hooks/catch/{user_id}/{zap_id}", post(zap_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn zap_handler(Path((user_id, zap_id)): Path<(String, String)>){

    // TODO: Store the trigger in the database
    println!("user_id: {}, zap_id: {}", user_id, zap_id);

    // TODO: Push the trigger to the queue(Kafka/Redis)

    // TODO: Store the trigger in the database
    let db = Database{};
    let result = db.random_fn();
    println!("result: {}", result);
}
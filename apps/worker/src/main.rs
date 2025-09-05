use kafka::consumer::start_consumer;

#[tokio::main]
async fn main() {
    start_consumer().await;
    
}

 
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use std::time::Duration;

pub fn create_producer() -> FutureProducer {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", "localhost:9092");

    let producer: FutureProducer = config.create().expect("Failed to create producer");

    producer
}

pub async fn produce(future_producer: &FutureProducer, message: String) {
    let record = FutureRecord::to("zap-run-outbox")
    .payload(&message)
    .key("zap-run-outbox");
  let status_deliever = future_producer
  .send(record, Timeout::After(Duration::from_secs(2)))
  .await;

  match status_deliever {
    Ok(delivery_status) => println!("Message sent successfully: {:?}", delivery_status),
    Err(e) => println!("Error sending message: {:?}", e),
  }
}
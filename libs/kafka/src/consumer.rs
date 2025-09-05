// use rdkafka::{consumer::{stream_consumer::StreamConsumer, Consumer, CommitMode}, ClientConfig, Message, config::RDKafkaLogLevel};
// use log::{info, warn};

// use crate::consumer;

use rdkafka::{ consumer, ClientConfig, Message};
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};

pub async fn start_consumer() {
    // let consumer: StreamConsumer = ClientConfig::new()
    //     .set("group.id", &group_id)
    //     .set("bootstrap.servers", &brokers)
    //     .set("enable.partition.eof", "false")
    //     .set("session.timeout.ms", "6000")
    //     .set("enable.auto.commit", "false")
    //     .create()
    //     .expect("Consumer creation failed");

    // consumer
    //     .subscribe(&[&topic])
    //     .expect("Can't subscribe to specified topic");

    // match consumer.recv().await {
    //     Err(e) => warn!("Kafka error: {}", e),
    //     Ok(m) => {
    //         let payload = match m.payload_view::<str>() {
    //             None => "",
    //             Some(Ok(s)) => s,
    //             Some(Err(e)) => {
    //                 warn!("Error while deserializing message payload: {:?}", e);
    //                 ""
    //             }
    //         };
    //         info!(
    //             "key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
    //             m.key(),
    //             payload,
    //             m.topic(),
    //             m.partition(),
    //             m.offset(),
    //             m.timestamp()
    //         );
    //         consumer.commit_message(&m, CommitMode::Async).unwrap();
    //         payload
    //     }
    // }

    let consumer: StreamConsumer = create_consumer();
    consume(consumer).await;
}

fn create_consumer() -> StreamConsumer {
    let mut config = ClientConfig::new();
    config
        .set("bootstrap.servers", "localhost:9092")
        .set("auto.offset.reset", "earliest")
        .set("group.id", "zapier-consumer")
        .set("socker-timeout.ms", "4000");

    let consumer: StreamConsumer = config.create().expect("Failed to create consumer");

    consumer
    
}
async fn consume(consumer: StreamConsumer){
    consumer.subscribe(&["zap-run-outbox"]).expect("Failed to subscribe to topic");

    loop {
        match consumer.recv().await {
            Err(e) => println!("Kafka error: {}", e),
            Ok(message) => {
                match message.payload_view::<str>() {
                    None => println!("No payload"),
                    Some(Ok(payload)) => println!("Payload: {}", payload),
                    Some(Err(e)) => println!("Error: {}", e),
                }
                consumer.commit_message(&message, CommitMode::Async).unwrap();
            }
        }
        
    }
}
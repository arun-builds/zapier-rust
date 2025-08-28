use database::{db::Database, models::processor::ZapRunOutbox};
use kafka::producer::{produce, create_producer};


#[tokio::main]
async fn main() {
    let producer = create_producer();
    let mut db = Database::default().unwrap();
    
    loop {
        let pending_rows = db.get_pending_zap_run_outbox().unwrap();
        if pending_rows.is_empty() {
            continue;
        }
        for row in pending_rows {
            produce(&producer, row.id.to_string()).await;
            println!("Processing zap run outbox: {:?}", row.zap_run_id);
        }
        
    }
}

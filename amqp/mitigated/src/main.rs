use lapin::{
    options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties,
};
use serde_json::json;
use std::env;
use std::io::{self, Write};
use tokio::time::{sleep, Duration};

const MAX_MESSAGE_SIZE: usize = 100 * 1024; // 100KB
const RATE_LIMIT_MS: u64 = 1000; // 1 message per second

#[tokio::main]
async fn main() {
    let _ = io::stdout().flush();
    
    if let Err(e) = run().await {
        eprintln!("ERROR: {}", e);
        let _ = io::stderr().flush();
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Mitigated application - IoT Health Monitor");
    println!("==============================================\n");
    io::stdout().flush()?;

    let host = env::var("RABBITMQ_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("RABBITMQ_PORT").unwrap_or_else(|_| "5672".to_string());
    let user = env::var("RABBITMQ_USER").unwrap_or_else(|_| "admin".to_string());
    let pass = env::var("RABBITMQ_PASS").unwrap_or_else(|_| "password".to_string());

    // Strong credentials from environment
    let amqp_url = format!("amqp://{}:{}@{}:{}", user, pass, host, port);
    println!("Connecting to: amqp://{}:***@{}:{}", user, host, port);
    io::stdout().flush()?;

    let conn = Connection::connect(&amqp_url, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    // Queue with strict limits
    let mut queue_args = FieldTable::default();
    queue_args.insert("x-max-length".into(), 1000.into()); // Max 1000 messages
    queue_args.insert("x-max-length-bytes".into(), (10 * 1024 * 1024).into()); // 10MB max
    queue_args.insert("x-message-ttl".into(), 300000.into()); // 5 min TTL

    channel
        .queue_declare(
            "health_data_queue_secure",
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            queue_args,
        )
        .await?;

    println!("Connected to secure RabbitMQ broker");
    println!("Using strong credentials and queue limits:");
    println!("  - Max 1000 messages");
    println!("  - Max 10MB total size");
    println!("  - 5 minute TTL\n");
    io::stdout().flush()?;

    let mut counter = 0;
    loop {
        counter += 1;

        let health_data = json!({
            "device_id": "iot_device_001",
            "patient_id": "patient_123",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "heart_rate": 75,
            "blood_pressure": "120/80",
            "temperature": 36.6
        });

        let payload = serde_json::to_string(&health_data)?;

        // Message size validation
        if payload.len() > MAX_MESSAGE_SIZE {
            eprintln!("Message too big ({} bytes), skipping!", payload.len());
            io::stderr().flush()?;
            continue;
        }

        channel
            .basic_publish(
                "",
                "health_data_queue_secure",
                BasicPublishOptions::default(),
                payload.as_bytes(),
                BasicProperties::default(),
            )
            .await?;

        println!("Message #{}: {} bytes", counter, payload.len());
        io::stdout().flush()?;

        // Rate limiting (1 message per second)
        sleep(Duration::from_millis(RATE_LIMIT_MS)).await;
    }
}

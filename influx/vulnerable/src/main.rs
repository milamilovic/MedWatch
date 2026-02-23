use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use rand::Rng;
use chrono::Utc;

const INFLUX_URL: &str = "http://influxdb-vulnerable:8086";
const DB_NAME: &str = "health_monitoring";

#[derive(Debug)]
struct PatientData {
    patient_id: String,
    heart_rate: f64,
    blood_pressure_systolic: f64,
    blood_pressure_diastolic: f64,
    temperature: f64,
    oxygen_saturation: f64,
}

impl PatientData {
    fn random(patient_id: &str) -> Self {
        let mut rng = rand::thread_rng();
        PatientData {
            patient_id: patient_id.to_string(),
            heart_rate: rng.gen_range(60.0..100.0),
            blood_pressure_systolic: rng.gen_range(110.0..140.0),
            blood_pressure_diastolic: rng.gen_range(70.0..90.0),
            temperature: rng.gen_range(36.5..37.5),
            oxygen_saturation: rng.gen_range(95.0..100.0),
        }
    }

    fn to_line_protocol(&self) -> String {
        let timestamp = Utc::now().timestamp_nanos_opt().unwrap_or(0);
        format!(
            "vitals,patient_id={} heart_rate={},bp_systolic={},bp_diastolic={},temperature={},spo2={} {}",
            self.patient_id,
            self.heart_rate,
            self.blood_pressure_systolic,
            self.blood_pressure_diastolic,
            self.temperature,
            self.oxygen_saturation,
            timestamp
        )
    }
}

async fn create_database(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    // baza se kreira bez autentifikacije
    // podrazumevano nema autentifikaciju
    let url = format!("{}/query", INFLUX_URL);
    let response = client
        .post(&url)
        .query(&[("q", format!("CREATE DATABASE {}", DB_NAME))])
        .send()
        .await?;
    
    println!("[INFO] Kreiranje baze podataka '{}': HTTP {}", DB_NAME, response.status());
    Ok(())
}

async fn write_patient_data(client: &Client, data: &PatientData) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/write", INFLUX_URL);
    let line = data.to_line_protocol();
    
    // influxDB sa auth-enabled=false prihvata sve zahteve
    let response = client
        .post(&url)
        .query(&[("db", DB_NAME)])
        .body(line.clone())
        .send()
        .await?;
    
    if response.status().is_success() {
        println!("[WRITE] Pacijent {}: HR={:.1}, BP={:.0}/{:.0}, Temp={:.1}°C, SpO2={:.1}%",
            data.patient_id,
            data.heart_rate,
            data.blood_pressure_systolic,
            data.blood_pressure_diastolic,
            data.temperature,
            data.oxygen_saturation
        );
    } else {
        println!("[ERROR] Greška pri pisanju: HTTP {}", response.status());
    }
    
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("--- RANJIVA IoT Monitoring Aplikacija MedWatch ---");
    println!("InfluxDB radi bez autentifikacije!");
    println!("Podaci pacijenata su izloženi bez ikakve zaštite!");
    println!();

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Nije moguće kreirati HTTP klijent");

    // Čekanje da InfluxDB bude spreman
    println!("Čekanje na InfluxDB...");
    sleep(Duration::from_secs(5)).await;

    // Kreiranje baze bez autentifikacije
    if let Err(e) = create_database(&client).await {
        eprintln!("Greška pri kreiranju baze: {}", e);
    }

    let patients = vec!["patient_001", "patient_002", "patient_003"];
    let mut iteration = 0u64;

    loop {
        iteration += 1;
        println!("\n--- Iteracija {} ({}UTC) ---", iteration, Utc::now().format("%H:%M:%S"));

        for patient_id in &patients {
            let data = PatientData::random(patient_id);
            if let Err(e) = write_patient_data(&client, &data).await {
                eprintln!("GREŠKA {}: {}", patient_id, e);
            }
            sleep(Duration::from_millis(200)).await;
        }

        sleep(Duration::from_secs(3)).await;
    }
}

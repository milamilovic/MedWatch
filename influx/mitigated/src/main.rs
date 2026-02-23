use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;
use rand::Rng;
use chrono::Utc;

// Kredencijali se sada čitaju iz environment varijabli
const INFLUX_URL: &str = "http://influxdb-secure:8086";
const INFLUX_ORG: &str = "medwatch";
const INFLUX_BUCKET: &str = "health_monitoring";

fn get_token() -> String {
    std::env::var("INFLUX_TOKEN").unwrap_or_else(|_| {
        panic!(" INFLUX_TOKEN environment varijabla nije postavljena! Aplikacija se ne može pokrenuti bez tokena.")
    })
}

struct PatientData {
    patient_id: String,
    heart_rate: f64,
    bp_systolic: f64,
    bp_diastolic: f64,
    temperature: f64,
    spo2: f64,
}

impl PatientData {
    fn random(patient_id: &str) -> Self {
        let mut rng = rand::thread_rng();
        PatientData {
            patient_id: patient_id.to_string(),
            heart_rate: rng.gen_range(60.0..100.0),
            bp_systolic: rng.gen_range(110.0..140.0),
            bp_diastolic: rng.gen_range(70.0..90.0),
            temperature: rng.gen_range(36.5..37.5),
            spo2: rng.gen_range(95.0..100.0),
        }
    }

    fn to_line_protocol(&self) -> String {
        let timestamp = Utc::now().timestamp_nanos_opt().unwrap_or(0);
        format!(
            "vitals,patient_id={} heart_rate={:.1},bp_systolic={:.1},bp_diastolic={:.1},temperature={:.2},spo2={:.1} {}",
            self.patient_id,
            self.heart_rate,
            self.bp_systolic,
            self.bp_diastolic,
            self.temperature,
            self.spo2,
            timestamp
        )
    }
}

async fn setup_influxdb(client: &Client, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Provera konekcije sa InfluxDB...");
    
    // influxDB zahteva token autentifikaciju za sve operacije
    let resp = client
        .get(&format!("{}/ping", INFLUX_URL))
        .send()
        .await?;
    
    println!(" InfluxDB ping: HTTP {}", resp.status());
    
    // Provera da li bucket postoji
    let resp = client
        .get(&format!("{}/api/v2/buckets", INFLUX_URL))
        .query(&[("name", INFLUX_BUCKET), ("org", INFLUX_ORG)])
        .header("Authorization", format!("Token {}", token))
        .send()
        .await?;
    
    println!(" Bucket provera: HTTP {}", resp.status());
    Ok(())
}

async fn write_patient_data(
    client: &Client,
    token: &str,
    data: &PatientData,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/api/v2/write", INFLUX_URL);
    let line = data.to_line_protocol();

    // MITIGACIJE
    // Svaki zahtev koristi Bearer token autentifikaciju
    // Koristi se InfluxDB 2.x API koji ne podržava JWT "none" algoritam
    // Precizno definisana prava pristupa tokena (write-only za ovaj servis)
    let response = client
        .post(&url)
        .query(&[("org", INFLUX_ORG), ("bucket", INFLUX_BUCKET), ("precision", "ns")])
        .header("Authorization", format!("Token {}", token))
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(line)
        .send()
        .await?;

    if response.status().as_u16() == 204 {
        println!("Pacijent {}: HR={:.1}, BP={:.0}/{:.0}, Temp={:.1}°C, SpO2={:.1}%",
            data.patient_id,
            data.heart_rate,
            data.bp_systolic,
            data.bp_diastolic,
            data.temperature,
            data.spo2
        );
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        println!("Greška: HTTP {} - {}", status, body);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("--- MITIGOVANA IoT Monitoring Aplikacija MedWatch ---");
    println!("Koristi InfluxDB 2.x sa obaveznom token autentifikacijom");
    println!("JWT 'none' algoritam nije podržan (CVE-2019-20933)");
    println!("Pristup bez autentifikacije je onemogućen");
    println!();

    // Token se čita iz env varijable, nikad nije hardkodovan
    let token = get_token();
    println!("Token autentifikacija: aktivna ({}...)", &token[..8]);

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Nije moguće kreirati HTTP klijent");

    println!("Čekanje na InfluxDB...");
    sleep(Duration::from_secs(10)).await;

    if let Err(e) = setup_influxdb(&client, &token).await {
        eprintln!("Setup greška: {}", e);
    }

    let patients = vec!["patient_001", "patient_002", "patient_003"];
    let mut iteration = 0u64;

    loop {
        iteration += 1;
        println!("\n--- Iteracija {} ({}UTC) ---", iteration, Utc::now().format("%H:%M:%S"));

        for patient_id in &patients {
            let data = PatientData::random(patient_id);
            if let Err(e) = write_patient_data(&client, &token, &data).await {
                eprintln!("GREŠKA {}: {}", patient_id, e);
            }
            sleep(Duration::from_millis(200)).await;
        }

        sleep(Duration::from_secs(3)).await;
    }
}

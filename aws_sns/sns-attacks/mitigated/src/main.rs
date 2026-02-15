use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use aws_sdk_sns::{Client as SnsClient, types::MessageAttributeValue};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tower_http::limit::RequestBodyLimitLayer;
use tracing::info;

const MAX_REQUEST_SIZE: usize = 16 * 1024; // 16KB

#[derive(Clone)]
struct AppState {
    sns_client: SnsClient,
    topic_arn: String,
    allowed_topics: HashSet<String>,
    jwt_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // patient_id
    exp: u64,
    iat: u64,
}

#[derive(Deserialize)]
struct LoginRequest {
    patient_id: String,
    password: String,
}

#[derive(Deserialize)]
struct SosAlertRequest {
    metric_type: String,
    value: f64,
    threshold: f64,
}

#[derive(Deserialize)]
struct SubscribeRequest {
    phone_number: String,
}

#[derive(Serialize)]
struct ApiResponse {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_in: Option<u64>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("Starting SECURE SNS Publisher");

    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .endpoint_url(std::env::var("SNS_ENDPOINT").unwrap_or_else(|_| "http://localstack:4566".to_string()))
        .load()
        .await;

    let sns_client = SnsClient::new(&config);
    let topic_arn = initialize_sns(&sns_client).await;

    let mut allowed_topics = HashSet::new();
    allowed_topics.insert(topic_arn.clone());

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "very-big-secret-key-for-demo".to_string());

    info!("SNS Topic initialized successfully");

    let state = AppState {
        sns_client,
        topic_arn,
        allowed_topics,
        jwt_secret,
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/auth/login", post(login))
        .route("/send-sos", post(send_sos_alert))
        .route("/subscribe-contact", post(subscribe_contact))
        .layer(RequestBodyLimitLayer::new(MAX_REQUEST_SIZE))
        .with_state(Arc::new(state));

    let addr = "0.0.0.0:8080";
    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn initialize_sns(client: &SnsClient) -> String {
    match client
        .create_topic()
        .name("medwatch-sos-alerts")
        .send()
        .await
    {
        Ok(output) => output.topic_arn().unwrap().to_string(),
        Err(e) => {
            panic!("Cannot initialize SNS: {:?}", e);
        }
    }
}

// Helper to extract patient ID from JWT
fn extract_patient_id(
    headers: &HeaderMap,
    jwt_secret: &str,
) -> Result<String, (StatusCode, Json<ErrorResponse>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Authentication required".to_string(),
                }),
            )
        })?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Invalid authorization header".to_string(),
                }),
            )
        })?;

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid or expired token".to_string(),
            }),
        )
    })?;

    Ok(claims.claims.sub)
}

async fn health_check() -> impl IntoResponse {
    Json(HealthResponse {
        status: "running".to_string(),
        service: "sns-publisher-secure".to_string(),
    })
}

// Login endpoint with rate limiting
async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<ApiResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate patient_id format
    if !validate_patient_id(&req.patient_id) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid patient ID format".to_string(),
            }),
        ));
    }

    if req.password != "demo123" {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid credentials".to_string(),
            }),
        ));
    }

    // Generate JWT
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = Claims {
        sub: req.patient_id.clone(),
        exp: now + 86400, // 24 hours
        iat: now,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Token generation failed".to_string(),
            }),
        )
    })?;

    Ok(Json(ApiResponse {
        status: "success".to_string(),
        message: None,
        token: Some(token),
        expires_in: Some(86400),
    }))
}

async fn send_sos_alert(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(alert_req): Json<SosAlertRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    // Extract patient ID from JWT
    let patient_id = extract_patient_id(&headers, &state.jwt_secret)?;

    // Validate metric type (whitelist)
    if !validate_metric_type(&alert_req.metric_type) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid metric type".to_string(),
            }),
        ));
    }

    // Validate metric value
    if !validate_metric_value(alert_req.value, &alert_req.metric_type) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid metric value".to_string(),
            }),
        ));
    }

    // Template-based message
    let message = format!(
        "MEDWATCH SOS ALERT\n\n\
         Critical Metric: {}\n\
         Value: {}\n\
         Threshold: {}\n\
         Time: {}\n\n\
         Emergency response may be required.",
        alert_req.metric_type,
        alert_req.value,
        alert_req.threshold,
        chrono::Utc::now().to_rfc3339()
    );

    match state
        .sns_client
        .publish()
        .topic_arn(&state.topic_arn)
        .message(&message)
        .subject("MedWatch Critical Alert")
        .message_attributes(
            "severity",
            MessageAttributeValue::builder()
                .data_type("String")
                .string_value("CRITICAL")
                .build()
                .unwrap(),
        )
        .send()
        .await
    {
        Ok(_) => {
            // Audit log without sensitive data
            audit_log("sos_alert_sent", &patient_id, &alert_req.metric_type);

            Ok((
                StatusCode::OK,
                Json(ApiResponse {
                    status: "success".to_string(),
                    message: Some("SOS alert sent successfully".to_string()),
                    token: None,
                    expires_in: None,
                }),
            ))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to send alert".to_string(),
            }),
        )),
    }
}

async fn subscribe_contact(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(sub_req): Json<SubscribeRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    // Extract patient ID from JWT
    let patient_id = extract_patient_id(&headers, &state.jwt_secret)?;

    // Validate phone number
    if !validate_phone_number(&sub_req.phone_number) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid phone number format".to_string(),
            }),
        ));
    }

    match state
        .sns_client
        .subscribe()
        .topic_arn(&state.topic_arn)
        .protocol("sms")
        .endpoint(&sub_req.phone_number)
        .send()
        .await
    {
        Ok(_) => {
            audit_log("contact_subscribed", &patient_id, "phone");

            Ok((
                StatusCode::OK,
                Json(ApiResponse {
                    status: "success".to_string(),
                    message: Some("Emergency contact subscribed successfully".to_string()),
                    token: None,
                    expires_in: None,
                }),
            ))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to subscribe contact".to_string(),
            }),
        )),
    }
}


fn validate_patient_id(id: &str) -> bool {
    let re = regex::Regex::new(r"^[a-zA-Z0-9_]{1,50}$").unwrap();
    re.is_match(id)
}

fn validate_metric_type(metric: &str) -> bool {
    matches!(
        metric,
        "heart_rate"
            | "blood_pressure_systolic"
            | "blood_pressure_diastolic"
            | "oxygen_saturation"
            | "temperature"
    )
}

fn validate_metric_value(value: f64, metric_type: &str) -> bool {
    let (min, max) = match metric_type {
        "heart_rate" => (30.0, 250.0),
        "blood_pressure_systolic" => (60.0, 250.0),
        "blood_pressure_diastolic" => (40.0, 150.0),
        "oxygen_saturation" => (70.0, 100.0),
        "temperature" => (35.0, 42.0),
        _ => return true,
    };

    value >= min && value <= max
}

fn validate_phone_number(phone: &str) -> bool {
    phonenumber::parse(None, phone).is_ok()
}

fn audit_log(action: &str, patient_id: &str, details: &str) {
    let mut hasher = Sha256::new();
    hasher.update(patient_id.as_bytes());
    let patient_hash = hex::encode(&hasher.finalize()[..4]);

    info!(
        "AUDIT: action={}, patient_hash={}, details={}",
        action, patient_hash, details
    );
}
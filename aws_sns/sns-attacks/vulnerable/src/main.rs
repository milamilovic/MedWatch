use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use aws_sdk_sns::{Client as SnsClient, types::MessageAttributeValue};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, error, info};

#[derive(Clone)]
struct AppState {
    sns_client: SnsClient,
    topic_arn: String,
}

#[derive(Deserialize)]
struct SosAlertRequest {
    patient_id: String,
    metric_type: String,
    value: f64,
    threshold: f64,
    #[serde(default)]
    custom_message: Option<String>,
    #[serde(default)]
    topic_arn: Option<String>,
}

#[derive(Deserialize)]
struct SubscribeRequest {
    phone_number: String,
    #[serde(default)]
    patient_id: String,
}

#[derive(Deserialize)]
struct BulkAlertRequest {
    count: usize,
    message: String,
}

#[derive(Serialize)]
struct ApiResponse {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic_arn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<usize>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    topic_arn: String,
}

#[tokio::main]
async fn main() {
    // VULNERABILITY: Verbose logging including secrets
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .init();

    info!("Starting VULNERABLE SNS Publisher");

    // AWS SDK configuration
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .endpoint_url(std::env::var("SNS_ENDPOINT").unwrap_or_else(|_| "http://localstack:4566".to_string()))
        .load()
        .await;

    let sns_client = SnsClient::new(&config);

    // Initialize SNS topic
    let topic_arn = initialize_sns(&sns_client).await;
    
    // VULNERABILITY: Logging sensitive information
    debug!("Topic ARN: {}", topic_arn);

    let state = AppState {
        sns_client,
        topic_arn,
    };

    // VULNERABILITY: No middleware for rate limiting or auth
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/send-sos", post(send_sos_alert))
        .route("/subscribe-contact", post(subscribe_contact))
        .route("/send-bulk", post(send_bulk_alerts))
        .route("/admin/topics", get(list_topics))
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
        Ok(output) => {
            let topic_arn = output.topic_arn().unwrap().to_string();
            info!("SNS Topic created: {}", topic_arn);

            // Subscribe test phone number
            let _ = client
                .subscribe()
                .topic_arn(&topic_arn)
                .protocol("sms")
                .endpoint("+381641234567")
                .send()
                .await;

            topic_arn
        }
        Err(e) => {
            error!("Failed to create topic: {:?}", e);
            panic!("Cannot initialize SNS");
        }
    }
}

async fn health_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(HealthResponse {
        status: "running".to_string(),
        service: "sns-publisher-vulnerable".to_string(),
        topic_arn: state.topic_arn.clone(),
    })
}

// VULNERABILITY: No authentication, no rate limiting, no input validation
async fn send_sos_alert(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SosAlertRequest>,
) -> impl IntoResponse {
    // VULNERABILITY: No authentication check - anyone can call this
    
    // VULNERABILITY: No input validation - directly using user input
    let patient_id = req.patient_id;
    let metric_type = req.metric_type;
    let value = req.value;
    let threshold = req.threshold;

    // VULNERABILITY: Custom message allows injection
    let message = if let Some(custom_msg) = req.custom_message {
        // VULNERABILITY: No sanitization!
        custom_msg
    } else {
        format!(
            "MEDWATCH SOS ALERT \n\n\
             Patient ID: {}\n\
             Critical Metric: {}\n\
             Current Value: {}\n\
             Threshold: {}\n\
             Timestamp: {}\n\n\
             Immediate medical attention may be required!",
            patient_id,
            metric_type,
            value,
            threshold,
            chrono::Utc::now().to_rfc3339()
        )
    };

    // VULNERABILITY: No topic ARN validation
    let topic_arn = req.topic_arn.unwrap_or_else(|| state.topic_arn.clone());

    // VULNERABILITY: No rate limiting before expensive SMS operation
    match state
        .sns_client
        .publish()
        .topic_arn(&topic_arn)
        .message(&message)
        .subject("MedWatch Critical Alert")
        .message_attributes(
            "patient_id",
            MessageAttributeValue::builder()
                .data_type("String")
                .string_value(&patient_id)
                .build()
                .unwrap(),
        )
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
        Ok(output) => {
            let message_id = output.message_id().unwrap_or("unknown");
            // VULNERABILITY: Logging sensitive information
            info!("SOS Alert sent - Patient: {}, MessageId: {}", patient_id, message_id);

            (
                StatusCode::OK,
                Json(ApiResponse {
                    status: "success".to_string(),
                    message: Some("SOS alert sent successfully".to_string()),
                    message_id: Some(message_id.to_string()),
                    topic_arn: Some(topic_arn),
                    error: None,
                    count: None,
                }),
            )
        }
        Err(e) => {
            // VULNERABILITY: Detailed error exposure
            error!("Error sending SOS: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    status: "error".to_string(),
                    message: None,
                    message_id: None,
                    topic_arn: None,
                    error: Some(format!("{:?}", e)),
                    count: None,
                }),
            )
        }
    }
}

// VULNERABILITY: No phone validation, no authentication, no rate limiting
async fn subscribe_contact(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SubscribeRequest>,
) -> impl IntoResponse {
    // VULNERABILITY: No input validation on phone number
    let phone_number = req.phone_number;
    let patient_id = req.patient_id;

    // VULNERABILITY: No verification - can subscribe others without consent
    match state
        .sns_client
        .subscribe()
        .topic_arn(&state.topic_arn)
        .protocol("sms")
        .endpoint(&phone_number)
        .return_subscription_arn(true)
        .send()
        .await
    {
        Ok(output) => {
            let sub_arn = output.subscription_arn().unwrap_or("pending");
            info!("Subscribed {} for patient {}", phone_number, patient_id);

            (
                StatusCode::OK,
                Json(ApiResponse {
                    status: "success".to_string(),
                    message: Some(format!("Emergency contact {} subscribed", phone_number)),
                    message_id: Some(sub_arn.to_string()),
                    topic_arn: None,
                    error: None,
                    count: None,
                }),
            )
        }
        Err(e) => {
            error!("Subscription error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    status: "error".to_string(),
                    message: None,
                    message_id: None,
                    topic_arn: None,
                    error: Some(format!("{:?}", e)),
                    count: None,
                }),
            )
        }
    }
}

// VULNERABILITY:  no limit on bulk sends
async fn send_bulk_alerts(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BulkAlertRequest>,
) -> impl IntoResponse {
    // VULNERABILITY: No authentication, no count limit
    let count = req.count;
    let message = req.message;

    let mut message_ids = Vec::new();

    // VULNERABILITY: Can send 10000+ messages at once
    for i in 0..count {
        let msg = format!("{} #{}", message, i + 1);

        match state
            .sns_client
            .publish()
            .topic_arn(&state.topic_arn)
            .message(&msg)
            .subject("Bulk Alert")
            .send()
            .await
        {
            Ok(output) => {
                if let Some(msg_id) = output.message_id() {
                    message_ids.push(msg_id.to_string());
                }
            }
            Err(e) => {
                error!("Bulk send error: {:?}", e);
            }
        }
    }

    (
        StatusCode::OK,
        Json(ApiResponse {
            status: "success".to_string(),
            message: Some(format!("Sent {} messages", message_ids.len())),
            message_id: None,
            topic_arn: None,
            error: None,
            count: Some(message_ids.len()),
        }),
    )
}

// VULNERABILITY: Admin endpoint with no authentication
async fn list_topics(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.sns_client.list_topics().send().await {
        Ok(output) => {
            let topics: Vec<String> = output
                .topics()
                .iter()
                .filter_map(|t| t.topic_arn().map(|s| s.to_string()))
                .collect();

            Json(serde_json::json!({
                "topics": topics
            }))
        }
        Err(e) => {
            error!("Error listing topics: {:?}", e);
            Json(serde_json::json!({
                "error": format!("{:?}", e)
            }))
        }
    }
}
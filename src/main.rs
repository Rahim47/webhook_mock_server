use actix_web::middleware::Logger;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WebhookPayload {
    timestamp: String,
    headers: Vec<(String, String)>,
    body: serde_json::Value,
}

struct AppState {
    webhooks: Mutex<Vec<WebhookPayload>>,
}

async fn handle_webhook(
    data: web::Data<AppState>,
    payload: web::Json<serde_json::Value>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    let headers = req
        .headers()
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_str().unwrap_or("").to_string()))
        .collect();

    let webhook = WebhookPayload {
        timestamp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_secs(0))
            .as_secs()
            .to_string(),
        headers,
        body: payload.into_inner(),
    };

    println!("Received webhook: {:#?}", webhook);

    match data.webhooks.lock() {
        Ok(mut webhooks) => webhooks.push(webhook.clone()),
        Err(err) => {
            eprintln!("Failed to acquire lock on webhooks: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    }

    HttpResponse::Ok().json(serde_json::json!({"status": "success"}))
}

async fn list_webhooks(data: web::Data<AppState>) -> impl Responder {
    match data.webhooks.lock() {
        Ok(webhooks) => HttpResponse::Ok().json(&*webhooks),
        Err(err) => {
            eprintln!("Failed to acquire lock on webhooks: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::new()
        .parse_filters("actix_web=info")
        .init();

    let app_state = web::Data::new(AppState {
        webhooks: Mutex::new(Vec::new()),
    });

    println!("Starting mock Jira webhook server on http://localhost:9998");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .route("/jira-webhook", web::post().to(handle_webhook))
            .route("/webhooks", web::get().to(list_webhooks))
    })
    .bind("127.0.0.1:9998")?
    .run()
    .await
}

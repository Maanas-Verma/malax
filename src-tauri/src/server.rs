use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[derive(Serialize, Clone)]
struct ModelDetails {
    format: String,
    family: String,
    families: Option<Vec<String>>,
    parameter_size: String,
    quantization_level: String,
}

#[derive(Serialize, Clone)]
struct Model {
    name: String,
    model: String,
    modified_at: String,
    size: u64,
    digest: String,
    details: ModelDetails,
}

#[derive(Serialize)]
struct ListModelsResponse {
    models: Vec<Model>,
}

#[derive(Deserialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
}

#[derive(Serialize)]
struct GenerateResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Serialize)]
struct ChatResponse {
    model: String,
    created_at: String,
    message: ChatMessage,
    done: bool,
}

#[derive(Serialize)]
struct VersionResponse {
    version: String,
}

async fn root() -> &'static str {
    "Malax is running"
}

async fn version() -> Json<VersionResponse> {
    Json(VersionResponse {
        version: "0.1.30".to_string(), // Mock version
    })
}

async fn list_models() -> Json<ListModelsResponse> {
    let mock_model = Model {
        name: "llama3:latest".to_string(),
        model: "llama3:latest".to_string(),
        modified_at: "2023-11-04T14:56:49.277302595-07:00".to_string(),
        size: 3826793677,
        digest: "fe938a131f40e6f6d40083c9f0f430a515233eb2edaa6d72eb85c50d64f2300e".to_string(),
        details: ModelDetails {
            format: "gguf".to_string(),
            family: "llama".to_string(),
            families: None,
            parameter_size: "8B".to_string(),
            quantization_level: "Q4_0".to_string(),
        },
    };
    Json(ListModelsResponse {
        models: vec![mock_model],
    })
}

async fn generate(Json(payload): Json<GenerateRequest>) -> Json<GenerateResponse> {
    Json(GenerateResponse {
        model: payload.model,
        created_at: "2023-08-04T08:52:19.385406455-07:00".to_string(),
        response: format!("Mock response to: {}", payload.prompt),
        done: true,
    })
}

async fn chat(Json(payload): Json<ChatRequest>) -> Json<ChatResponse> {
    let last_message = payload.messages.last().map(|m| m.content.clone()).unwrap_or_default();
    Json(ChatResponse {
        model: payload.model,
        created_at: "2023-08-04T08:52:19.385406455-07:00".to_string(),
        message: ChatMessage {
            role: "assistant".to_string(),
            content: format!("Mock chat response to: {}", last_message),
        },
        done: true,
    })
}

pub async fn run() {
    let app = Router::new()
        .route("/", get(root))
        .route("/api/version", get(version))
        .route("/api/tags", get(list_models))
        .route("/api/generate", post(generate))
        .route("/api/chat", post(chat))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 11435));
    println!("Mock Malax listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

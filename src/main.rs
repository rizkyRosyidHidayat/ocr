use axum::{
    extract::{DefaultBodyLimit, Multipart, Query, State},
    routing::post,
    Json, Router,
};
use ocrs::{OcrEngine, OcrEngineParams};
use serde::{Deserialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

mod error;
mod models;
mod ocr;
mod output;

use error::AppError;
use models::{load_model, resolve_model_source};
use output::{format_json_output, format_text_output, FormatJsonArgs};

/// Default text detection model URL.
const DETECTION_MODEL_URL: &str =
    "https://ocrs-models.s3-accelerate.amazonaws.com/text-detection.rten";

/// Default text recognition model URL.
const RECOGNITION_MODEL_URL: &str =
    "https://ocrs-models.s3-accelerate.amazonaws.com/text-recognition.rten";

/// Shared application state holding the initialized OCR engine.
struct AppState {
    engine: OcrEngine,
}

/// Query parameters for the OCR endpoint.
#[derive(Deserialize)]
struct OcrQuery {
    /// Output format: `"text"` (default) or `"json"`.
    format: Option<String>,
}




#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file (if present).
    dotenvy::dotenv().ok();

    // Read configuration from environment.
    let host = std::env::var("OCR_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("OCR_PORT").unwrap_or_else(|_| "8000".to_string());
    let detection_model_path =
        std::env::var("OCR_DETECTION_MODEL").unwrap_or_else(|_| "models/text-detection.rten".to_string());
    let recognition_model_path =
        std::env::var("OCR_RECOGNITION_MODEL").unwrap_or_else(|_| "models/text-recognition.rten".to_string());
    let max_body_size: usize = std::env::var("OCR_MAX_BODY_SIZE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10 * 1024 * 1024); // 10MB default

    let bind_addr = format!("{}:{}", host, port);

    // Resolve model sources: use local files if available, otherwise
    // auto-download from S3.
    let detection_source = resolve_model_source(&detection_model_path, DETECTION_MODEL_URL);
    let recognition_source = resolve_model_source(&recognition_model_path, RECOGNITION_MODEL_URL);
    // let detection_source = resolve_model_source("", DETECTION_MODEL_URL);
    // let recognition_source = resolve_model_source("", RECOGNITION_MODEL_URL);

    eprintln!("Loading detection model from {}...", detection_source);
    let detection_model = load_model(detection_source)?;

    eprintln!("Loading recognition model from {}...", recognition_source);
    let recognition_model = load_model(recognition_source)?;

    // Initialize OCR engine.
    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })?;

    let shared_state = Arc::new(AppState { engine });

    // Configure CORS.
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router.
    let app = Router::new()
        .route("/ocr", post(handle_ocr))
        .layer(DefaultBodyLimit::max(max_body_size))
        .layer(cors)
        .with_state(shared_state);

    // Start server.
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    eprintln!("🚀 OCR server running at http://{}", bind_addr);
    axum::serve(listener, app).await?;

    Ok(())
}

/// Handle OCR requests.
///
/// Accepts a multipart form with an `image` field. Optionally accepts a
/// `format` query parameter:
/// - `text` (default): Returns `{ "text": "..." }`
/// - `json`: Returns structured JSON with paragraphs, lines, words, and
///   bounding box vertices (HierText format)
async fn handle_ocr(
    State(state): State<Arc<AppState>>,
    query: Query<OcrQuery>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, AppError> {
    // Extract image bytes from multipart form.
    let mut image_bytes: Option<Vec<u8>> = None;

    while let Some(field) = multipart.next_field().await? {
        if field.name() == Some("image") {
            image_bytes = Some(field.bytes().await?.to_vec());
        }
    }

    let image_bytes = image_bytes.ok_or_else(|| {
        anyhow::anyhow!("Missing 'image' field in multipart form data")
    })?;

    // Run OCR pipeline.
    let result = ocr::perform_ocr(&state.engine, &image_bytes)?;

    // Format output based on requested format.
    let output_format = query.format.as_deref().unwrap_or("text");

    let response = match output_format {
        "json" => format_json_output(FormatJsonArgs {
            image_hw: result.image_hw,
            text_lines: &result.line_texts,
        }),
        _ => {
            let text = format_text_output(&result.line_texts);
            serde_json::json!({ "text": text })
        }
    };

    Ok(Json(response))
}
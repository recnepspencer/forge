//! Axum REST API server for hierarchical trace inspection.
//!
//! Endpoints:
//! - `GET /api/traces` — list all traces with summary stats
//! - `GET /api/traces/:id` — trace overview (spans, no decisions)
//! - `GET /api/traces/:id/spans/:span_id` — decisions within a span
//! - `GET /api/traces/:id/decisions/:idx` — single decision detail
//! - `GET /api/traces/:id/summary` — display_interesting text
//! - `POST /api/reload` — re-scan the traces directory
//! - `GET /` — serve the web UI

use std::sync::{Arc, Mutex};

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
};
use tower_http::cors::CorsLayer;

use crate::trace_store::TraceStore;

/// Shared application state.
pub type AppState = Arc<Mutex<TraceStore>>;

/// Build the axum router with all trace viewer routes.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(serve_ui))
        .route("/api/traces", get(list_traces))
        .route("/api/traces/{id}", get(get_trace))
        .route("/api/traces/{id}/spans/{span_id}", get(get_span_decisions))
        .route("/api/traces/{id}/decisions/{idx}", get(get_decision))
        .route("/api/traces/{id}/summary", get(get_summary))
        .route("/api/reload", post(reload_traces))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// `GET /api/traces` — list all traces with summary stats.
async fn list_traces(State(store): State<AppState>) -> impl IntoResponse {
    let store = store.lock().unwrap();
    let traces = store.list_traces();
    Json(serde_json::to_value(traces).unwrap())
}

/// `GET /api/traces/:id` — trace overview with spans (no decisions).
async fn get_trace(
    State(store): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let store = store.lock().unwrap();
    match store.get_trace_overview(&id) {
        Some(overview) => Json(serde_json::to_value(overview).unwrap()).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// `GET /api/traces/:id/spans/:span_id` — decisions within a span.
async fn get_span_decisions(
    State(store): State<AppState>,
    Path((id, span_id)): Path<(String, u64)>,
) -> impl IntoResponse {
    let store = store.lock().unwrap();
    match store.get_span_decisions(&id, span_id) {
        Some(decisions) => Json(serde_json::to_value(decisions).unwrap()).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// `GET /api/traces/:id/decisions/:idx` — single decision detail.
async fn get_decision(
    State(store): State<AppState>,
    Path((id, idx)): Path<(String, usize)>,
) -> impl IntoResponse {
    let store = store.lock().unwrap();
    match store.get_decision(&id, idx) {
        Some(decision) => Json(serde_json::to_value(decision).unwrap()).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// `GET /api/traces/:id/summary` — display_interesting text.
async fn get_summary(
    State(store): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let store = store.lock().unwrap();
    match store.get_raw_log(&id) {
        Some(log) => log.display_interesting().into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// `POST /api/reload` — re-scan the traces directory.
async fn reload_traces(State(store): State<AppState>) -> impl IntoResponse {
    let mut store = store.lock().unwrap();
    let count = store.reload();
    Json(serde_json::json!({ "loaded": count }))
}

/// `GET /` — serve the embedded web UI.
async fn serve_ui() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

//! Forge Trace Server — binary entry point.
//!
//! Usage: `cargo run -p forge-view --bin forge-trace-server [traces_dir]`
//!
//! Starts an HTTP server on port 9091 serving the trace viewer API and UI.
//! Reads trace files from `traces_dir` (defaults to `./traces/`).

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use forge_view::trace::server::{build_router, AppState};
use forge_view::trace::store::TraceStore;

#[tokio::main]
async fn main() {
    let trace_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("traces"));

    eprintln!("Forge Trace Server");
    eprintln!("  Trace directory: {}", trace_dir.display());

    let mut store = TraceStore::new(trace_dir);
    let count = store.reload();
    eprintln!("  Loaded {} trace(s)", count);

    let state: AppState = Arc::new(Mutex::new(store));
    let app = build_router(state);

    let addr = "0.0.0.0:9091";
    eprintln!("  Listening on http://localhost:9091");
    eprintln!("  POST /api/reload to re-scan after new test runs");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

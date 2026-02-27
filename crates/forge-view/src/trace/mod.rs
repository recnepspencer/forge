//! Trace inspection infrastructure.
//!
//! DOMAIN: Trace storage, HTTP API, native GUI, and CLI inspector.
//! DEPENDENCIES: `forge-core` (DecisionLog, TraceEvent)
//!
//! ## Sub-modules
//!
//! - `store` — In-memory trace index (`TraceStore`, `TraceFile`, `TraceMeta`)
//! - `server` — Axum REST API for hierarchical drill-down
//! - `viewer` — Native egui trace viewer app

#[cfg(feature = "server")]
pub mod server;
pub mod store;
#[cfg(feature = "gui")]
pub mod viewer;

#[cfg(feature = "server")]
pub use server::{build_router, AppState};
pub use store::{DecisionView, SpanView, TraceFile, TraceMeta, TraceOverview, TraceStore};
#[cfg(feature = "gui")]
pub use viewer::TraceViewerApp;

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

pub mod store;
#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "gui")]
pub mod viewer;

pub use store::{TraceStore, TraceFile, TraceMeta, TraceOverview, SpanView, DecisionView};
#[cfg(feature = "server")]
pub use server::{build_router, AppState};
#[cfg(feature = "gui")]
pub use viewer::TraceViewerApp;

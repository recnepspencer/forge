//! # forge-view
//!
//! Trace inspection infrastructure for the Forge geometry kernel.
//!
//! ## Domains
//!
//! - **trace** — In-memory trace store, REST API server, native egui viewer
//!
//! ## Binaries
//!
//! - `forge-trace-server` — HTTP server on port 9091
//! - `forge-trace-viewer` — Native egui desktop app
//! - `forge-trace-cli` — CLI inspector for AI drill-down

#![forbid(unsafe_code)]

pub mod trace;

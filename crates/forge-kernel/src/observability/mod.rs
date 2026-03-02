//! Kernel observability domain component.
//!
//! DOMAIN: Span-based decision and operation telemetry.
//!
//! ## Structure
//!
//! ```text
//! observability/
//! ├── data/
//! │   └── output.rs     ← SpanOutput
//! ├── logic/
//! │   ├── collector.rs  ← SpanCollector + CURRENT_SPAN thread-local
//! │   ├── span.rs       ← KernelSpan (public recording API)
//! │   └── guard.rs      ← KernelSpanGuard, KernelSpanHandle
//! └── facade.rs
//! ```

mod data;
mod logic;

pub mod facade;

#[cfg(test)]
mod tests;

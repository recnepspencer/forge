//! Tracing and span-based operation collection.
//!
//! DOMAIN: The KernelSpan thread-local collector.

pub mod span;

pub use span::{KernelSpan, KernelSpanGuard, KernelSpanHandle, SpanOutput};

#[cfg(test)]
mod tests;

//! Kernel observability domain component.
//!
//! DOMAIN: Span-based decision and operation telemetry.

pub mod facade;
pub mod span;

#[cfg(test)]
mod tests;

//! Main boolean execution logic (split, classify, assemble).
//!
//! DOMAIN: Orchestrate the full boolean pipeline and wrap results.

mod eval;
mod assemble;

pub use eval::{execute_boolean, execute_boolean_with_overrides};

//! Main boolean execution logic (split, classify, assemble).
//!
//! DOMAIN: Orchestrate the full boolean pipeline and wrap results.

mod assemble;
mod eval;

pub(crate) use assemble::assemble_result;
pub use eval::{
    execute_boolean_direct, execute_boolean_with_engine, execute_boolean_with_overrides,
};

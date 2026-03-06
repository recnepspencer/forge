//! Public API for operation pipeline infrastructure.
//!
//! External modules should import pipeline primitives from this facade.
//!
//! CONSUMERS: operations/boolean, integration_tests, engine/pipeline

pub use super::builder::{OperationPipeline, PipelineBuilder};
pub use super::step_contract::{OperationAuditRecord, StepAuditEntry, StepContract};

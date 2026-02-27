//! Invariant checkpoint system for the proof pipeline.
//!
//! DOMAIN: Automatic topology validation wired into the operation pipeline.
//!
//! - `schema`: ValidationConfig, ValidationCheckpoint, ValidationResult, run_checkpoint
//! - `diagnose`: PipelineStage, PipelineDiagnostic, diagnose_arena (non-fatal mid-pipeline)
//!
//! DEPENDENCIES: `forge-topo` (arena, validate), `forge-core` (KernelError)

pub mod diagnose;
pub mod schema;

//! P0.5 Invariant Checkpoint System
//!
//! DOMAIN: Automatic topology validation wired into the operation pipeline.
//!
//! INVARIANTS:
//! - Every checkpoint evaluation is logged via `ValidationResult`.
//! - Geometric validation skips entities beyond `entity_limit` but logs the skip.
//! - Non-geometric validation adds < 5% overhead to operations.
//!
//! DEPENDENCIES: `forge-topo` (validate, arena), `forge-core` (KernelError)

mod config;
mod eval;
mod result;

pub use config::{ValidationCheckpoint, ValidationConfig};
pub use eval::{run_checkpoint, run_spec_checkpoint, run_spec_envelope_checkpoint};
pub use result::ValidationResult;

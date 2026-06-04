#[path = "phase_chain/admitted_scaffold/mod.rs"]
mod admitted_scaffold;
#[path = "result_surface/artifact.rs"]
mod artifact;
pub(crate) mod authoring;
mod authoring_authority;
pub(crate) mod authoring_entry;
pub(crate) mod authoring_input;
#[path = "certification/mod.rs"]
pub(crate) mod certification;
#[path = "runtime_proof/continuity_branch_runtime.rs"]
#[cfg(test)]
pub(crate) mod continuity_branch_runtime;
#[path = "runtime_proof/continuity_replay.rs"]
#[cfg(test)]
pub(crate) mod continuity_replay;
#[path = "runtime_proof/diagnostics.rs"]
pub(crate) mod diagnostics;
mod digest;
#[path = "result_surface/evidence.rs"]
mod evidence;
#[path = "runtime_proof/family_coverage.rs"]
pub(crate) mod family_coverage;
#[path = "phase_chain/intent.rs"]
pub(crate) mod intent;
#[path = "runtime_proof/arbitration/replay.rs"]
#[cfg(test)]
pub(crate) mod intent_arbitration_replay;
#[path = "runtime_proof/motion/branch_runtime.rs"]
pub(crate) mod motion_branch_runtime;
#[path = "runtime_proof/motion/replay.rs"]
pub(crate) mod motion_replay;
#[path = "result_surface/outcome.rs"]
pub(crate) mod outcome;
#[path = "runtime_proof/parity.rs"]
#[cfg(test)]
pub(crate) mod parity;
#[path = "runtime_proof/preview_branch_runtime.rs"]
#[cfg(test)]
pub(crate) mod preview_branch_runtime;
#[path = "runtime_proof/preview_replay.rs"]
#[cfg(test)]
pub(crate) mod preview_replay;
#[path = "runtime_proof/profile_branch_runtime.rs"]
#[cfg(test)]
pub(crate) mod profile_branch_runtime;
#[path = "runtime_proof/profile_replay.rs"]
#[cfg(test)]
pub(crate) mod profile_replay;
#[path = "proof/mod.rs"]
mod proof;
#[path = "runtime_proof/query/mod.rs"]
mod query;
#[path = "runtime_proof/realization_truth.rs"]
mod realization_truth;
#[path = "phase_chain/request.rs"]
pub(crate) mod request;
#[path = "result_surface/result.rs"]
pub(crate) mod result;
#[path = "runtime_proof/runtime_basis.rs"]
pub(crate) mod runtime_basis;
#[path = "phase_chain/specs.rs"]
pub(crate) mod specs;

pub use crate::spatial_intent::PrimitiveConstructionSpatialIntentError;
pub use intent::PrimitiveConstructionIntent;
pub use request::PrimitiveConstructionFamily;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;

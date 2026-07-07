// --- Certification-only harness vocabulary and execution surface ---
#[doc(hidden)]
pub use crate::handoffs::{
    BlobHarnessAccessMode, BlobHarnessActorMix, BlobHarnessChunkSizeClass,
    BlobHarnessChunkTopology, BlobHarnessFailurePoint, BlobHarnessPlacementClass,
    BlobHarnessSecurityScopeClass, BlobHarnessSizeClass, BlobHarnessTopologyDenial,
};
pub use crate::harness_execution::{
    execute_s7_blob_harness, BlobHarnessExecutedWitness, BlobHarnessExecutionInput,
    BlobHarnessObservedYieldpoint,
};
pub use crate::S7ExecutedLifecycleEvidenceBundle;

pub fn materialize_s7_executed_lifecycle_evidence(
    witness: BlobHarnessExecutedWitness,
) -> S7ExecutedLifecycleEvidenceBundle {
    witness.into_closeout_evidence()
}

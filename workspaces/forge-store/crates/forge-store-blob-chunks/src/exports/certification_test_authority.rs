// --- Certification-only harness vocabulary and execution surface ---
#[doc(hidden)]
pub use crate::handoffs::{
    BlobHarnessAccessMode, BlobHarnessActorMix, BlobHarnessChunkSizeClass,
    BlobHarnessChunkTopology, BlobHarnessFailurePoint, BlobHarnessPlacementClass,
    BlobHarnessSecurityScopeClass, BlobHarnessSizeClass, BlobHarnessTopologyDenial,
};
pub use crate::harness_execution::Phase28OperationsWitnesses;
pub use crate::harness_execution::{
    execute_blob_harness, BlobHarnessExecutedWitness, BlobHarnessExecutionInput,
    BlobHarnessObservedYieldpoint,
};
use crate::BoundaryBridgedCanonicalExportArtifact;
pub use crate::ExecutedBlobLifecycleEvidenceBundle;

pub fn materialize_blob_executed_lifecycle_evidence(
    witness: BlobHarnessExecutedWitness,
) -> ExecutedBlobLifecycleEvidenceBundle {
    witness.into_closeout_evidence()
}

pub fn bridge_blob_export_trust_boundary(
    witness: &BlobHarnessExecutedWitness,
) -> BoundaryBridgedCanonicalExportArtifact {
    witness.bridged_export_artifact().clone()
}

pub fn phase28_operations_witnesses(
    case: &str,
    bytes: &'static [u8],
    chunk_size: u64,
) -> Phase28OperationsWitnesses {
    crate::harness_execution::phase28_operations_witnesses(case, bytes, chunk_size)
}

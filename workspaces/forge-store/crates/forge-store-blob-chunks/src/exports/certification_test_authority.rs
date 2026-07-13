// --- Certification-only harness vocabulary and execution surface ---
#[doc(hidden)]
pub use crate::handoffs::{
    BlobHarnessAccessMode, BlobHarnessActorMix, BlobHarnessChunkSizeClass,
    BlobHarnessChunkTopology, BlobHarnessFailurePoint, BlobHarnessPlacementClass,
    BlobHarnessSecurityScopeClass, BlobHarnessSizeClass, BlobHarnessTopologyDenial,
};
pub use crate::harness_execution::{
    execute_blob_harness, BlobHarnessExecutedWitness, BlobHarnessExecutionInput,
    BlobHarnessObservedYieldpoint,
};
use crate::BoundaryBridgedCanonicalExportArtifact;
pub use crate::ExecutedBlobLifecycleEvidenceBundle;
use forge_store_security::StoreTrustBoundaryCrossing;

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

pub fn execute_readmitted_blob_import(case: &str) -> crate::ImportedBlobWitness {
    execute_readmitted_blob_import_for_store(case, "store.new.strategy")
}

pub fn execute_readmitted_blob_import_for_store(
    case: &str,
    store_authority_key: &str,
) -> crate::ImportedBlobWitness {
    use crate::import_readmission::test_support::{
        collect_current_chunks, import_lane, readmission_trigger,
    };

    let lane = import_lane(case, b"layout-import-authority", 23);
    let authority = crate::BlobImportReadmissionAuthority::from_current_store_authority(
        crate::test_support::current_authority(store_authority_key, "layout-import"),
    );
    let bridged = crate::bridge_canonical_export_trust_boundary(&lane.bundle);
    let current_chunks = collect_current_chunks(&authority, &lane);
    let trigger = readmission_trigger(
        StoreTrustBoundaryCrossing::OfflineExportImport,
        bridged.declaration().chunk_scope(),
        case,
    );
    let readmitted = authority
        .readmit_import_declaration_after_boundary(&bridged, trigger, &current_chunks)
        .expect("certification import must readmit through the production boundary");
    let placement = readmitted
        .plan_placement_admission()
        .expect("certification import placement must admit");
    readmitted
        .admit_imported_blob(&placement)
        .expect("certification import witness must follow production readmission")
}

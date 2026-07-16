use crate::handoffs::BlobHarnessSecurityScopeClass;
use crate::{
    AuthenticatedFrameDigest, BlobAuthorityClassification, BlobChunkReachabilityProofSet,
    BlobChunkRootPublication, BlobGeneration, BlobLifecycleAdmission, BlobLifecycleDeclaration,
    BlobLifecycleReadinessAuthority, BlobLifecycleStoreAuthority, BlobObjectId, LifecycleReceipt,
};

use super::backend::{current_authority, stable_digest};
use super::certification_test_authority::lifecycle_multichunk_reachability;
use super::chunk_sequence::GeneratedBlobSequence;
use super::placement_admission::admit_placement;
use super::transition_success::TransitionSuccess;

pub(super) struct ExecutedBlobLane {
    pub(super) lifecycle: LifecycleReceipt,
    pub(super) reachability: BlobChunkReachabilityProofSet,
    pub(super) placement: crate::AdmittedBlobPlacement,
}

pub(super) fn execute_lifecycle(
    case: &str,
    _request_scope: BlobHarnessSecurityScopeClass,
    publication: &BlobChunkRootPublication,
    generated: &GeneratedBlobSequence,
) -> ExecutedBlobLane {
    let lifecycle_leaf = generated.sequence.proof_frontier().first_leaf();
    let declaration = BlobLifecycleDeclaration::new(
        crate::lifecycle::BlobLifecycleIdentityBasis::new(
            BlobObjectId::from_declared_digest(stable_digest(&format!("sha256:{case}.object"))),
            BlobGeneration::published(1),
            publication.chunk_tree_root().clone(),
            publication.logical_content_digest().clone(),
        ),
        lifecycle_leaf.security_metadata(),
        lifecycle_leaf.stored_digest().clone(),
        AuthenticatedFrameDigest::from_declared_digest(stable_digest(&format!(
            "sha256:{case}.frame"
        ))),
        BlobAuthorityClassification::StoreOwnedPhysicalBlob,
    );
    let reachability = lifecycle_multichunk_reachability(&declaration, generated);
    let placement = admit_placement(
        case,
        &reachability,
        crate::BlobHarnessPlacementClass::StoreLocal,
    );
    let store_authority = BlobLifecycleStoreAuthority::from_current_store_authority(
        current_authority(case, "blob-harness-lifecycle"),
    );
    let lowering = store_authority.lowering_capability();
    let ready = BlobLifecycleAdmission::start(declaration)
        .resolve_store_authority(store_authority)
        .lower_lifecycle_plan(lowering)
        .admit_reachability(reachability.clone())
        .success("reachability")
        .admit_placement(placement.clone())
        .success("placement")
        .ready_for_execution(BlobLifecycleReadinessAuthority::from_admitted_placement(
            placement.clone(),
        ))
        .success("readiness");
    let replay = ready.admitted_replay_input();
    let lifecycle = ready
        .execute_lifecycle_replay(replay)
        .success("execution")
        .into_lifecycle_receipt();
    ExecutedBlobLane {
        lifecycle,
        reachability,
        placement,
    }
}

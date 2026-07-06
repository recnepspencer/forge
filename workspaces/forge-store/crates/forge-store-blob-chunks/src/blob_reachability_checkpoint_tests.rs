use forge_store_physical_isolation::read_during_checkpoint_verdict_for_certification_test;

use crate::blob_chunk_test_support::{admitted_sequence_for_scope, blob_scope};
use crate::blob_publication_commit_test_support::publish_generation_with_bytes_and_chunk_size;
use crate::{
    BlobChunkReachabilityRegistry, BlobReachabilityDenial, BlobReachabilityEdge,
    BlobReachabilityReclaimDecision,
};

#[test]
fn checkpoint_hold_cannot_seed_empty_registry_authority() {
    let verdict = read_during_checkpoint_verdict_for_certification_test();
    let mut registry = BlobChunkReachabilityRegistry::new_store_owned();

    assert!(matches!(
        registry.admit_checkpoint_hold(&verdict),
        Err(BlobReachabilityDenial::InvalidProtectedHold { .. })
    ));
}

#[test]
fn checkpoint_hold_admits_through_registry_authority_and_blocks_reclaim() {
    let (published, sequence) = published_with_sequence("phase14-checkpoint-hold");
    let leaf = sequence.proof_frontier().first_leaf();
    let edge = BlobReachabilityEdge::primary_blob_reference(&published, leaf)
        .expect("primary edge should admit");
    let verdict = read_during_checkpoint_verdict_for_certification_test();
    let mut registry = BlobChunkReachabilityRegistry::new_store_owned();

    registry.admit_edge(edge).expect("edge should admit");
    registry
        .admit_checkpoint_hold(&verdict)
        .expect("checkpoint hold should admit through registry authority");

    let proof = registry
        .prove_reachable_chunks()
        .expect("reachable chunk proof set should admit");
    assert_eq!(proof.protected_holds().len(), 1);
    assert!(matches!(
        registry.reclaim_decision_for(leaf.identity()),
        BlobReachabilityReclaimDecision::ReclaimDenied(_)
    ));
}

fn published_with_sequence(
    case: &str,
) -> (
    crate::BlobGenerationPublished,
    crate::AdmittedBlobChunkSequence,
) {
    let bytes = b"phase14 checkpoint reachability";
    let scope = blob_scope(
        case,
        forge_store_security::StoreTenantScope::TenantPhysicalBoundary,
    );
    let sequence = admitted_sequence_for_scope(scope, bytes);
    let (published, _) =
        publish_generation_with_bytes_and_chunk_size(case, bytes, bytes.len() as u64);
    (published, sequence)
}

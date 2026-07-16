use worth_store_budgets::CounterEvidenceStrength;
use worth_store_security::StoreTenantScope;

use crate::publication::test_support::publish_generation_with_bytes_and_chunk_size;
use crate::test_support::{admitted_blob_custody, admitted_sequence_for_scope, blob_scope};
use crate::{
    AdmittedBlobCustody, BlobChunkReachabilityRegistry, BlobCustodyPurpose, BlobReachabilityDenial,
    BlobReachabilityEdge, BlobReachabilityReclaimDecision,
};

#[test]
fn export_hold_cannot_seed_empty_registry_and_admits_through_reachability_authority() {
    let (published, sequence) = published_with_sequence("phase14-export-hold-authority");
    let export =
        backup_export_readiness("phase14-export-hold-authority", BlobCustodyPurpose::Export);
    let leaf = sequence.proof_frontier().first_leaf();
    let edge = BlobReachabilityEdge::primary_blob_reference(&published, leaf)
        .expect("primary edge should admit");
    let mut empty_registry = BlobChunkReachabilityRegistry::new_store_owned();

    assert!(matches!(
        empty_registry.admit_export_hold(&export),
        Err(BlobReachabilityDenial::InvalidProtectedHold { .. })
    ));

    let mut registry = BlobChunkReachabilityRegistry::new_store_owned();
    registry.admit_edge(edge).expect("edge should admit");
    registry
        .admit_export_hold(&export)
        .expect("export hold should admit through registry authority");
    let proof = registry
        .prove_reachable_chunks()
        .expect("export hold should remain visible");

    assert_eq!(proof.protected_holds().len(), 1);
    assert_eq!(proof.counters().protected_holds(), 1);
    assert!(matches!(
        registry.reclaim_decision_for(leaf.identity()),
        BlobReachabilityReclaimDecision::ReclaimDenied(_)
    ));
}

#[test]
fn release_churn_converges_to_same_reachable_set_edges_and_exact_counters() {
    let (published, sequence) = published_with_sequence("phase14-release-converge");
    let leaf = sequence.proof_frontier().first_leaf();
    let edge = BlobReachabilityEdge::primary_blob_reference(&published, leaf)
        .expect("primary edge should admit");

    let mut churned = BlobChunkReachabilityRegistry::new_store_owned();
    churned.admit_edge(edge.clone()).expect("edge should admit");
    churned
        .release_edge(&edge)
        .expect("edge release should admit");
    churned
        .admit_edge(edge.clone())
        .expect("edge re-admission should admit");

    let mut direct = BlobChunkReachabilityRegistry::new_store_owned();
    direct.admit_edge(edge).expect("edge should admit");

    let churned_snapshot = churned
        .canonical_snapshot()
        .expect("churned snapshot should prove");
    let direct_snapshot = direct
        .canonical_snapshot()
        .expect("direct snapshot should prove");

    assert_eq!(churned_snapshot, direct_snapshot);
    assert_eq!(
        churned_snapshot.counters().strength(),
        CounterEvidenceStrength::Exact
    );
    assert_eq!(churned_snapshot.counters().reference_edges(), 1);
    assert_eq!(churned_snapshot.counters().dedupe_reference_edges(), 0);
}

fn published_with_sequence(
    case: &str,
) -> (
    crate::BlobGenerationPublished,
    crate::AdmittedBlobChunkSequence,
) {
    let bytes = b"phase14 authority repair";
    let scope = blob_scope(case, StoreTenantScope::TenantPhysicalBoundary);
    let sequence = admitted_sequence_for_scope(scope, bytes);
    let (published, _) =
        publish_generation_with_bytes_and_chunk_size(case, bytes, bytes.len() as u64);
    (published, sequence)
}

fn backup_export_readiness(case: &str, purpose: BlobCustodyPurpose) -> AdmittedBlobCustody {
    admitted_blob_custody(case, purpose)
}

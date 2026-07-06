use forge_proof::TransitionOutcome;
use forge_store_security::StoreTenantScope;

use crate::test_support::{
    blob_scope, candidate_for_bytes_and_scope, canonical_equivalence,
};
use crate::publication::test_support::publish_generation_with_bytes_and_chunk_size;
use crate::{
    BlobChunkDedupeAdmission, BlobChunkDedupeReceipt, BlobChunkDedupeReferenceRegistry,
    BlobChunkReachabilityRegistry, BlobChunkReferenceAccountingRegistry, BlobChunkSecurityScope,
    BlobReachabilityEdge, BlobReachabilityReclaimDecision,
};

#[test]
fn copied_replayed_dedupe_release_cannot_clear_reachability() {
    let receipt = dedupe_receipt_for_case("phase14-dedupe-release-copy", b"shared release bytes");
    let mut dedupe_registry = BlobChunkDedupeReferenceRegistry::new_store_owned();
    let registered = receipt
        .admit_into_reference_registry(&mut dedupe_registry)
        .expect("registered dedupe reference should mint");

    let (published, sequence) =
        published_with_sequence("phase14-dedupe-release-copy", b"shared release bytes");
    let leaf = sequence.proof_frontier().first_leaf();
    let edge = BlobReachabilityEdge::dedupe_shared_reference(&registered, &published, leaf)
        .expect("dedupe edge should admit");

    let mut reachability = BlobChunkReachabilityRegistry::new_store_owned();
    reachability
        .admit_edge(edge)
        .expect("reachability edge should admit");
    assert_eq!(reachability.counters().dedupe_reference_edges(), 1);

    let _copied_release =
        copied_release_for_case("phase14-dedupe-release-copy", b"shared release bytes");

    let proof = reachability
        .prove_reachable_chunks()
        .expect("copied replayed release must not clear live reachability");
    assert_eq!(proof.counters().dedupe_reference_edges(), 1);
    assert!(matches!(
        reachability.reclaim_decision_for(leaf.identity()),
        BlobReachabilityReclaimDecision::ReclaimDenied(_)
    ));
}

#[test]
fn owning_dedupe_release_clears_its_reachability_edge() {
    let receipt = dedupe_receipt_for_case("phase14-dedupe-release-owner", b"owned release bytes");
    let (published, sequence) =
        published_with_sequence("phase14-dedupe-release-owner", b"owned release bytes");
    let leaf = sequence.proof_frontier().first_leaf();
    let shared_identity = receipt.existing_identity().clone();
    let security_metadata = receipt.security_metadata();
    let mut accounting = BlobChunkReferenceAccountingRegistry::new_store_owned();
    accounting
        .admit_dedupe_reference(receipt, &published, leaf)
        .expect("owning accounting registry should admit dedupe reachability");
    accounting
        .deny_all_dedupe_edges_for(&shared_identity, security_metadata)
        .expect("owning release should mint and apply");

    assert!(matches!(
        accounting.reclaim_decision_for(leaf.identity()),
        BlobReachabilityReclaimDecision::ReclaimPermitted(_)
    ));
}

fn copied_release_for_case(case: &str, bytes: &[u8]) -> crate::BlobChunkDedupeReferenceRelease {
    let mut registry = BlobChunkDedupeReferenceRegistry::new_store_owned();
    let registered = dedupe_receipt_for_case(case, bytes)
        .admit_into_reference_registry(&mut registry)
        .expect("copied reference should mint");
    registry
        .deny_all_edges_for(registered.shared_identity(), registered.security_metadata())
        .expect("copied release should mint")
}

fn dedupe_receipt_for_case(case: &str, bytes: &[u8]) -> BlobChunkDedupeReceipt {
    dedupe_receipt_for_scopes(
        bytes,
        blob_scope(
            &format!("{case}-existing"),
            StoreTenantScope::TenantPhysicalBoundary,
        ),
        blob_scope(
            &format!("{case}-candidate"),
            StoreTenantScope::TenantPhysicalBoundary,
        ),
    )
}

fn dedupe_receipt_for_scopes(
    bytes: &[u8],
    existing_scope: BlobChunkSecurityScope,
    candidate_scope: BlobChunkSecurityScope,
) -> BlobChunkDedupeReceipt {
    let existing = candidate_for_bytes_and_scope(bytes, existing_scope);
    let candidate = candidate_for_bytes_and_scope(bytes, candidate_scope);
    let equivalence = canonical_equivalence(&existing, &candidate);
    match BlobChunkDedupeAdmission::compare_candidates(existing, candidate)
        .with_foundational_canonical_equivalence(equivalence)
        .admit()
    {
        TransitionOutcome::Success(receipt) => receipt,
        outcome => panic!("dedupe should admit: {outcome:?}"),
    }
}

fn published_with_sequence(
    case: &str,
    bytes: &[u8],
) -> (
    crate::BlobGenerationPublished,
    crate::AdmittedBlobChunkSequence,
) {
    let scope = blob_scope(case, StoreTenantScope::TenantPhysicalBoundary);
    let sequence = crate::test_support::admitted_sequence_for_scope(scope, bytes);
    let (published, _) =
        publish_generation_with_bytes_and_chunk_size(case, bytes, bytes.len() as u64);
    (published, sequence)
}

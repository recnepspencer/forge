use forge_proof::TransitionOutcome;
use forge_store_contracts::StableDigest;
use forge_store_security::StoreTenantScope;

use crate::blob_chunk_test_support::{
    admitted_multichunk_sequence_for_scope, admitted_sequence_for_scope, blob_scope,
    candidate_for_bytes_and_scope, forced_collision_equivalence, physical_payload_for_bytes,
};
use crate::{
    reject_checksum_only_evidence_as_chunk_root_publication,
    reject_digest_only_evidence_as_chunk_root_publication, BlobChunkDedupeAdmission,
    BlobChunkDedupeAdmissionDenial, BlobChunkDedupeByteComparison, BlobChunkDedupeCandidate,
    BlobChunkIntegrityDenial, BlobChunkRootCanonicalComparison, BlobChunkRootPublication,
    BlobChunkRootPublicationDenial, BlobChunkSequenceAdmission, BlobChunkSize,
    BlobChunkingRuleAdmission,
};

#[test]
fn same_ordered_scoped_chunks_publish_same_root_digest_basis_and_comparison() {
    let left = admitted_multichunk_sequence_for_scope(
        blob_scope(
            "phase4-equivalence",
            StoreTenantScope::TenantPhysicalBoundary,
        ),
        b"aaaabbbbcccc",
        4,
    );
    let right = admitted_multichunk_sequence_for_scope(
        blob_scope(
            "phase4-equivalence",
            StoreTenantScope::TenantPhysicalBoundary,
        ),
        b"aaaabbbbcccc",
        4,
    );

    let left_publication = BlobChunkRootPublication::publish(left).expect("left publishes");
    let right_publication = BlobChunkRootPublication::publish(right).expect("right publishes");
    let comparison =
        BlobChunkRootCanonicalComparison::compare(&left_publication, &right_publication)
            .expect("comparison prepares");

    assert_eq!(
        left_publication.chunk_tree_root(),
        right_publication.chunk_tree_root()
    );
    assert_eq!(
        left_publication.logical_content_digest(),
        right_publication.logical_content_digest()
    );
    assert_eq!(
        left_publication.canonical_basis().canonical_digest(),
        right_publication.canonical_basis().canonical_digest()
    );
    assert!(comparison.is_equivalent());
    assert_eq!(left_publication.canonical_basis().chunk_count(), 3);
    assert_eq!(
        left_publication
            .canonical_basis()
            .counters()
            .canonical_basis_entries(),
        4
    );
    assert_eq!(comparison.counters().canonical_comparisons(), 1);
}

#[test]
fn middle_chunk_difference_changes_root_basis_even_when_edges_match() {
    let left = admitted_multichunk_sequence_for_scope(
        blob_scope(
            "phase4-middle-collision",
            StoreTenantScope::TenantPhysicalBoundary,
        ),
        b"aaaabbbbcccc",
        4,
    );
    let right = admitted_multichunk_sequence_for_scope(
        blob_scope(
            "phase4-middle-collision",
            StoreTenantScope::TenantPhysicalBoundary,
        ),
        b"aaaaxxxxcccc",
        4,
    );
    assert_eq!(
        left.proof_frontier().first_chunk().identity(),
        right.proof_frontier().first_chunk().identity()
    );
    assert_eq!(
        left.proof_frontier().latest_chunk().identity(),
        right.proof_frontier().latest_chunk().identity()
    );

    let left_publication = BlobChunkRootPublication::publish(left).expect("left publishes");
    let right_publication = BlobChunkRootPublication::publish(right).expect("right publishes");
    let comparison =
        BlobChunkRootCanonicalComparison::compare(&left_publication, &right_publication)
            .expect("comparison prepares");

    assert_ne!(
        left_publication.chunk_tree_root(),
        right_publication.chunk_tree_root()
    );
    assert!(!comparison.is_equivalent());
}

#[test]
fn ordering_tail_and_empty_denials_happen_before_root_publication() {
    assert_eq!(
        BlobChunkSequenceAdmission::start(denial_scope(), denial_rule(), 0),
        Err(BlobChunkIntegrityDenial::EmptyByteWindow)
    );

    let missing_tail = BlobChunkSequenceAdmission::start(denial_scope(), denial_rule(), 8)
        .unwrap()
        .push_payload(0, physical_payload_for_bytes(b"aaaa"))
        .unwrap()
        .finish();
    assert!(matches!(
        missing_tail,
        Err(BlobChunkIntegrityDenial::MissingTailChunk { .. })
    ));

    let reordered = BlobChunkSequenceAdmission::start(denial_scope(), denial_rule(), 8)
        .unwrap()
        .push_payload(4, physical_payload_for_bytes(b"bbbb"));
    assert!(matches!(
        reordered,
        Err(BlobChunkIntegrityDenial::UnexpectedWindowOffset { .. })
    ));

    let duplicated_middle = BlobChunkSequenceAdmission::start(denial_scope(), denial_rule(), 12)
        .unwrap()
        .push_payload(0, physical_payload_for_bytes(b"aaaa"))
        .unwrap()
        .push_payload(4, physical_payload_for_bytes(b"bbbb"))
        .unwrap()
        .push_payload(4, physical_payload_for_bytes(b"bbbb"));
    assert!(matches!(
        duplicated_middle,
        Err(BlobChunkIntegrityDenial::UnexpectedWindowOffset { .. })
    ));
}

#[test]
fn checksum_only_and_digest_only_evidence_cannot_publish_root() {
    let scope = blob_scope(
        "phase4-weak-evidence",
        StoreTenantScope::TenantPhysicalBoundary,
    );
    let sequence = admitted_sequence_for_scope(scope, b"weak-evidence");
    let checksum = sequence.first_chunk().checksum().clone();
    let digest = StableDigest::new("sha256:copied-root").expect("digest");

    assert!(matches!(
        reject_checksum_only_evidence_as_chunk_root_publication(checksum),
        BlobChunkRootPublicationDenial::ChecksumOnlyEvidenceRejected { .. }
    ));
    assert!(matches!(
        reject_digest_only_evidence_as_chunk_root_publication(digest),
        BlobChunkRootPublicationDenial::DigestOnlyEvidenceRejected { .. }
    ));
}

#[test]
fn forced_digest_collision_requires_root_comparison_before_dedupe_denial() {
    let existing_sequence = admitted_sequence_for_scope(
        blob_scope("phase4-collision", StoreTenantScope::TenantPhysicalBoundary),
        b"same-digest-left",
    );
    let candidate_sequence = admitted_sequence_for_scope(
        blob_scope("phase4-collision", StoreTenantScope::TenantPhysicalBoundary),
        b"same-digest-right",
    );
    let existing_publication = BlobChunkRootPublication::publish(existing_sequence.clone())
        .expect("existing root publishes");
    let candidate_publication = BlobChunkRootPublication::publish(candidate_sequence.clone())
        .expect("candidate root publishes");
    let root_comparison =
        BlobChunkRootCanonicalComparison::compare(&existing_publication, &candidate_publication)
            .expect("root comparison prepares");
    assert!(!root_comparison.is_equivalent());

    let missing_existing =
        BlobChunkDedupeCandidate::from_integrity_proof(existing_sequence.first_chunk().clone());
    let missing_candidate = candidate_for_bytes_and_scope(
        b"same-digest-right",
        blob_scope("phase4-collision", StoreTenantScope::TenantPhysicalBoundary),
    )
    .with_forced_content_digest_for_collision_fixture(missing_existing.content_digest().clone());
    let missing_equivalence = forced_collision_equivalence(&missing_existing, &missing_candidate);

    let missing_root =
        BlobChunkDedupeAdmission::compare_candidates(missing_existing, missing_candidate)
            .with_foundational_canonical_equivalence(missing_equivalence)
            .admit();
    assert!(matches!(
        missing_root,
        TransitionOutcome::Denied(
            BlobChunkDedupeAdmissionDenial::CanonicalRootComparisonRequired { .. }
        )
    ));

    let existing =
        BlobChunkDedupeCandidate::from_integrity_proof(existing_sequence.first_chunk().clone());
    let candidate = candidate_for_bytes_and_scope(
        b"same-digest-right",
        blob_scope("phase4-collision", StoreTenantScope::TenantPhysicalBoundary),
    )
    .with_forced_content_digest_for_collision_fixture(existing.content_digest().clone());
    let forced_equivalence = forced_collision_equivalence(&existing, &candidate);
    let byte_comparison = BlobChunkDedupeByteComparison::compare_chunk_payloads(
        &existing,
        &candidate,
        &physical_payload_for_bytes(b"same-digest-left"),
        &physical_payload_for_bytes(b"same-digest-right"),
    )
    .expect("collision byte comparison is bounded to the chunk payloads");
    let with_root = BlobChunkDedupeAdmission::compare_candidates(existing, candidate)
        .with_foundational_canonical_equivalence(forced_equivalence)
        .with_root_canonical_comparison(root_comparison)
        .with_byte_comparison(byte_comparison)
        .admit();
    let TransitionOutcome::Denied(BlobChunkDedupeAdmissionDenial::DigestCollisionDenied {
        counters,
        posture,
        ..
    }) = with_root
    else {
        panic!("collision must deny after byte verification");
    };
    assert_eq!(
        posture,
        crate::BlobChunkDedupeCollisionPosture::DigestCollisionDenied
    );
    assert_eq!(counters.byte_verify_probes(), 1);
    assert_eq!(counters.collision_denials(), 1);
}

fn denial_scope() -> crate::BlobChunkSecurityScope {
    blob_scope("phase4-denials", StoreTenantScope::TenantPhysicalBoundary)
}

fn denial_rule() -> BlobChunkingRuleAdmission {
    BlobChunkingRuleAdmission::fixed_size(BlobChunkSize::from_bytes(4).unwrap())
        .expect("rule admits")
}

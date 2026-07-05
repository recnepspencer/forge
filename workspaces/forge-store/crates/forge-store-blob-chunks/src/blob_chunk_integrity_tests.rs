use forge_proof::TransitionOutcome;
use forge_store_contracts::StableDigest;
use forge_store_physical_format::{
    PhysicalBinaryEncodingWitness, PhysicalChunkChecksumAuthority, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalPageId, PhysicalPageKind,
    PhysicalPageRecordAuthority, PhysicalPublicationState, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalSegmentId, SlotAppendRequest,
    StorePhysicalChunkWriteReceipt, PHYSICAL_HEADER_LENGTH,
};
use forge_store_security::StoreTenantScope;

use crate::blob_chunk_test_support::{
    admitted_sequence_for_scope, blob_scope, candidate_for_scope, canonical_equivalence,
    forced_collision_equivalence,
};
use crate::{
    reject_checksum_only_evidence_as_blob_chunk_integrity,
    reject_digest_only_evidence_as_blob_chunk_integrity, BlobChunkDedupeAdmission,
    BlobChunkDedupeAdmissionDenial, BlobChunkDedupeCandidate, BlobChunkIntegrityDenial,
    BlobChunkRootCanonicalComparison, BlobChunkRootPublication, BlobChunkSequenceAdmission,
    BlobChunkSize, BlobChunkingRuleAdmission,
};

#[test]
fn same_stream_and_rule_replay_produces_same_chunk_identity_and_digests() {
    let first = admitted_sequence("phase3.equivalence.one", &[b"abcd", b"efgh"], 8);
    let second = admitted_sequence("phase3.equivalence.one", &[b"abcd", b"efgh"], 8);

    assert_eq!(
        first.chunk_identity_summary(),
        second.chunk_identity_summary()
    );
    assert_eq!(first.chunk_tree_root(), second.chunk_tree_root());
    assert_eq!(
        first.logical_content_digest(),
        second.logical_content_digest()
    );
    assert_eq!(
        first.proof_frontier().first_chunk().checksum(),
        second.proof_frontier().first_chunk().checksum()
    );
    assert_eq!(first.proof_frontier().total_bytes(), 8);
    assert_eq!(first.proof_frontier().chunk_count(), 2);
    assert_eq!(first.counters().bytes_chunked(), 8);
    assert_eq!(first.counters().chunks_emitted(), 2);
    assert_eq!(first.counters().checksums_computed(), 2);
    assert_eq!(first.counters().chunk_tree_nodes_materialized(), 2);
}

#[test]
fn reordered_duplicate_and_missing_tail_chunks_deny_before_sequence_publication() {
    let rule = rule(4);
    let scope = blob_scope("phase3.denial", StoreTenantScope::TenantPhysicalBoundary);
    let reordered = BlobChunkSequenceAdmission::start(scope, rule.clone(), 8)
        .unwrap()
        .push_payload(4, payload(b"efgh"));
    assert!(matches!(
        reordered,
        Err(BlobChunkIntegrityDenial::UnexpectedWindowOffset { counters, .. })
            if counters.order_denials() == 1
    ));

    let duplicate = BlobChunkSequenceAdmission::start(
        blob_scope("phase3.duplicate", StoreTenantScope::TenantPhysicalBoundary),
        rule.clone(),
        8,
    )
    .unwrap()
    .push_payload(0, payload(b"abcd"))
    .unwrap()
    .push_payload(0, payload(b"abcd"));
    assert!(matches!(
        duplicate,
        Err(BlobChunkIntegrityDenial::UnexpectedWindowOffset { counters, .. })
            if counters.order_denials() == 1
    ));

    let missing_tail = BlobChunkSequenceAdmission::start(
        blob_scope(
            "phase3.missing-tail",
            StoreTenantScope::TenantPhysicalBoundary,
        ),
        rule,
        8,
    )
    .unwrap()
    .push_payload(0, payload(b"abcd"))
    .unwrap()
    .finish();
    assert!(matches!(
        missing_tail,
        Err(BlobChunkIntegrityDenial::MissingTailChunk { counters, .. })
            if counters.order_denials() == 1
    ));
}

#[test]
fn noncanonical_short_interior_chunk_denies_before_identity_construction() {
    let denial = BlobChunkSequenceAdmission::start(
        blob_scope(
            "phase3.noncanonical-interior",
            StoreTenantScope::TenantPhysicalBoundary,
        ),
        rule(4),
        8,
    )
    .unwrap()
    .push_payload(0, payload(b"ab"));

    assert!(matches!(
        denial,
        Err(BlobChunkIntegrityDenial::NonCanonicalInteriorChunk { counters })
            if counters.order_denials() == 1
    ));
}

#[test]
fn checksum_only_and_digest_only_evidence_are_denied_before_chunk_authority() {
    assert!(matches!(
        reject_checksum_only_evidence_as_blob_chunk_integrity(payload(b"abcd").checksum().clone()),
        BlobChunkIntegrityDenial::ChecksumOnlyEvidenceRejected { counters }
            if counters.checksum_only_denials() == 1
    ));
    assert!(matches!(
        reject_digest_only_evidence_as_blob_chunk_integrity(
            StableDigest::new("sha256:copied").unwrap()
        ),
        BlobChunkIntegrityDenial::DigestOnlyEvidenceRejected { counters }
            if counters.digest_only_denials() == 1
    ));
}

#[test]
fn digest_equivalent_dedupe_still_requires_chunk_proof_and_scope_admission() {
    let existing = candidate_for_scope(blob_scope(
        "phase3.collision.same",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    let candidate = candidate_for_scope(blob_scope(
        "phase3.collision.same",
        StoreTenantScope::TenantPhysicalBoundary,
    ));

    assert!(matches!(
        BlobChunkDedupeAdmission::compare_candidates(existing, candidate).admit(),
        TransitionOutcome::Denied(
            BlobChunkDedupeAdmissionDenial::MissingFoundationalCanonicalEquivalence { counters }
        ) if counters.digest_only_denials() == 1
    ));

    let existing = candidate_for_scope(blob_scope(
        "phase3.collision.verified",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    let candidate = candidate_for_scope(blob_scope(
        "phase3.collision.verified",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    let equivalence = canonical_equivalence(&existing, &candidate);
    assert!(matches!(
        BlobChunkDedupeAdmission::compare_candidates(existing, candidate)
            .with_foundational_canonical_equivalence(equivalence)
            .admit(),
        TransitionOutcome::Success(_)
    ));
}

#[test]
fn forced_digest_equivalence_collision_requires_chunk_byte_verification() {
    let existing_sequence = admitted_sequence_for_scope(
        blob_scope(
            "phase3.collision.forced.left",
            StoreTenantScope::TenantPhysicalBoundary,
        ),
        b"same-collision-bytes",
    );
    let candidate_sequence = admitted_sequence_for_scope(
        blob_scope(
            "phase3.collision.forced.left",
            StoreTenantScope::TenantPhysicalBoundary,
        ),
        b"different-collision-bytes",
    );
    let comparison = BlobChunkRootCanonicalComparison::compare(
        &BlobChunkRootPublication::publish(existing_sequence.clone()).unwrap(),
        &BlobChunkRootPublication::publish(candidate_sequence.clone()).unwrap(),
    )
    .unwrap();
    let existing =
        BlobChunkDedupeCandidate::from_integrity_proof(existing_sequence.first_chunk().clone());
    let forced_digest = existing.content_digest().clone();
    let candidate =
        BlobChunkDedupeCandidate::from_integrity_proof(candidate_sequence.first_chunk().clone())
            .with_forced_content_digest_for_collision_fixture(forced_digest);
    let equivalence = forced_collision_equivalence(&existing, &candidate);

    let outcome = BlobChunkDedupeAdmission::compare_candidates(existing, candidate)
        .with_foundational_canonical_equivalence(equivalence)
        .with_root_canonical_comparison(comparison)
        .admit();
    let TransitionOutcome::Denied(BlobChunkDedupeAdmissionDenial::ChunkByteVerificationRequired {
        receipt,
        counters,
    }) = outcome
    else {
        panic!("forced digest collision must deny through byte verification: {outcome:?}");
    };
    assert_eq!(receipt.counters(), counters);
    assert_ne!(receipt.existing_identity(), receipt.candidate_identity());
    assert_eq!(receipt.existing_proof().byte_range().start(), 0);
    assert_eq!(receipt.candidate_proof().byte_range().start(), 0);
    assert_eq!(counters.collision_probes(), 1);
    assert_eq!(counters.byte_verify_probes(), 1);
    assert_eq!(counters.collision_denials(), 1);
}

fn admitted_sequence(
    scope_key: &str,
    windows: &[&[u8]],
    declared_total_bytes: u64,
) -> crate::AdmittedBlobChunkSequence {
    let mut admission = BlobChunkSequenceAdmission::start(
        blob_scope(scope_key, StoreTenantScope::TenantPhysicalBoundary),
        rule(4),
        declared_total_bytes,
    )
    .unwrap();
    let mut offset = 0;
    for window in windows {
        admission = admission.push_payload(offset, payload(window)).unwrap();
        offset += window.len() as u64;
    }
    admission.finish().unwrap()
}

fn rule(bytes: u64) -> BlobChunkingRuleAdmission {
    BlobChunkingRuleAdmission::fixed_size(BlobChunkSize::from_bytes(bytes).unwrap()).unwrap()
}

fn payload(bytes: &[u8]) -> forge_store_physical_format::PhysicalChunkPayloadIntegrityWitness {
    let receipt = record_receipt(bytes);
    PhysicalChunkChecksumAuthority::s7_canonical()
        .admit_store_payload(receipt)
        .unwrap()
}

fn record_receipt(bytes: &[u8]) -> StorePhysicalChunkWriteReceipt {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let page_cell = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(5));
    let slot_cell = generations
        .slot_cell(segment(7), page(11), slot(1))
        .with_slot_generation(generation(9));
    let empty_page = page_bytes(generation(5), &[]);
    let append = records
        .append_record(
            admitted_page(&records, page_cell, &empty_page),
            SlotAppendRequest::ordinary(slot_cell, bytes),
        )
        .unwrap();
    let reopened_page = page_bytes(generation(5), append.page_payload());
    let validation = references
        .validate_page_slot(append.reference_admission(), slot_cell)
        .unwrap();
    let located = records
        .locate_record(
            admitted_page(&records, page_cell, &reopened_page),
            validation,
        )
        .unwrap();
    StorePhysicalChunkWriteReceipt::from_page_record_view(located.record_view()).unwrap()
}

fn admitted_page<'a>(
    records: &PhysicalPageRecordAuthority,
    page_cell: forge_store_physical_format::PageGenerationCell,
    bytes: &'a [u8],
) -> forge_store_physical_format::RecordPagePayload<'a> {
    let header = records
        .decode_record_page_header(page_cell, bytes, PhysicalPageKind::DataPage)
        .unwrap();
    records
        .admit_record_page_payload(bytes, header.witness())
        .unwrap()
}

fn record_authority() -> PhysicalPageRecordAuthority {
    PhysicalPageRecordAuthority::s1(PhysicalHeaderAuthority::s1(
        PhysicalBinaryEncodingWitness::s1_canonical().unwrap(),
    ))
}

fn page_bytes(generation: PhysicalGeneration, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalPageKind::DataPage.tag());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation.get().to_le_bytes());
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(payload);
    bytes
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}

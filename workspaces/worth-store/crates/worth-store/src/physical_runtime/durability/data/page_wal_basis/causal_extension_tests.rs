use worth_proof::CanonicalVec;
use worth_store_physical_format::{
    encode_data_frame_page_lsn, encode_inline_page, DurableFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalPageId, PhysicalPageLsn, PhysicalRecordFormatDeclaration,
    PhysicalSegmentId, RecordArtifactFile,
};
use worth_store_wal::LogSequenceNumber;

use super::{PageWalBasis, PhysicalRedoLsn};
use crate::physical_runtime::durability::{
    CertifiedPriorPageBasis, PhysicalDataFrameIdentity, PhysicalDataFrameSubject,
};

#[test]
fn bounded_delta_is_a_strict_causal_extension_of_its_certified_prior() {
    let (target, prior) = materialized_prior(10);
    let bytes = encoded_target(target, 12);
    let basis = PageWalBasis::from_encoded_frame(target, prior, delta(&[(0, 11), (1, 12)]), &bytes)
        .expect("a strictly advancing new delta must be admitted");

    assert_eq!(basis.resulting_page_lsn(), PhysicalPageLsn::new(12));
    assert_eq!(basis.delta().len(), 2);
}

#[test]
fn empty_duplicate_regressing_and_lifetime_deltas_are_rejected() {
    let (target, prior) = materialized_prior(10);
    let bytes = encoded_target(target, 11);

    assert!(PageWalBasis::from_encoded_frame(target, prior, delta(&[]), &bytes).is_none());
    assert!(
        PageWalBasis::from_encoded_frame(target, prior, delta(&[(0, 11), (1, 11)]), &bytes,)
            .is_none()
    );
    assert!(
        PageWalBasis::from_encoded_frame(target, prior, delta(&[(0, 12), (1, 11)]), &bytes,)
            .is_none()
    );
    assert!(
        PageWalBasis::from_encoded_frame(target, prior, delta(&[(0, 4), (1, 11)]), &bytes,)
            .is_none()
    );
}

#[test]
fn encoded_page_lsn_and_prior_identity_cannot_be_substituted() {
    let (target, prior) = materialized_prior(10);
    let ahead = encoded_target(target, 13);
    assert!(
        PageWalBasis::from_encoded_frame(target, prior, delta(&[(0, 11), (1, 12)]), &ahead,)
            .is_none()
    );

    let (_, foreign_prior) = materialized_prior_for_page(2, 10);
    let exact = encoded_target(target, 12);
    assert!(PageWalBasis::from_encoded_frame(
        target,
        foreign_prior,
        delta(&[(0, 11), (1, 12)]),
        &exact,
    )
    .is_none());
}

fn materialized_prior(page_lsn: u64) -> (PhysicalDataFrameIdentity, CertifiedPriorPageBasis) {
    materialized_prior_for_page(1, page_lsn)
}

fn materialized_prior_for_page(
    page_id: u64,
    page_lsn: u64,
) -> (PhysicalDataFrameIdentity, CertifiedPriorPageBasis) {
    let format = PhysicalRecordFormatDeclaration::builder()
        .admit()
        .expect("test format");
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let segment = PhysicalSegmentId::from_raw(1).expect("segment");
    let page = PhysicalPageId::from_raw(page_id).expect("page");
    let source_page = generations
        .page_cell(segment, page)
        .with_page_generation(PhysicalGeneration::from_raw(1).expect("generation"));
    let target_page = generations
        .page_cell(segment, page)
        .with_page_generation(PhysicalGeneration::from_raw(2).expect("generation"));
    let mut source_bytes = encode_inline_page(format, source_page, &[]).expect("source page");
    encode_data_frame_page_lsn(
        &mut source_bytes,
        DurableFrameKind::InlinePage,
        PhysicalPageLsn::new(page_lsn),
    )
    .expect("source pageLSN");
    let source = PhysicalDataFrameIdentity::inline_page(
        source_page,
        RecordArtifactFile::Segment {
            segment: 1,
            generation: 1,
        },
        0,
        u32::try_from(source_bytes.len()).expect("bounded source page"),
    )
    .expect("source identity");
    let target = PhysicalDataFrameIdentity::inline_page(
        target_page,
        RecordArtifactFile::Segment {
            segment: 1,
            generation: 2,
        },
        0,
        u32::try_from(source_bytes.len()).expect("bounded target page"),
    )
    .expect("target identity");
    let prior = CertifiedPriorPageBasis::for_materialized_source(source, format, &source_bytes)
        .expect("exact source basis");
    (target, prior)
}

fn encoded_target(target: PhysicalDataFrameIdentity, page_lsn: u64) -> Vec<u8> {
    let PhysicalDataFrameSubject::InlinePage(page) = target.subject() else {
        panic!("causal extension tests use inline pages");
    };
    let format = PhysicalRecordFormatDeclaration::builder()
        .admit()
        .expect("test format");
    let mut bytes = encode_inline_page(format, page, &[]).expect("target page");
    encode_data_frame_page_lsn(
        &mut bytes,
        DurableFrameKind::InlinePage,
        PhysicalPageLsn::new(page_lsn),
    )
    .expect("target pageLSN");
    bytes
}

fn delta(entries: &[(u32, u64)]) -> CanonicalVec<PhysicalRedoLsn> {
    CanonicalVec::try_from_sorted(
        entries
            .iter()
            .map(|(ordinal, lsn)| PhysicalRedoLsn::new(*ordinal, LogSequenceNumber::new(*lsn)))
            .collect(),
    )
    .expect("test entries are in canonical ordinal order")
}

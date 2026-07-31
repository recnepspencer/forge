use worth_proof::CanonicalVec;
use worth_store_physical_format::{
    encode_data_frame_page_lsn, encode_inline_page, DurableFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalPageId, PhysicalPageLsn, PhysicalRecordFormatDeclaration,
    PhysicalSegmentId, RecordArtifactFile,
};
use worth_store_wal::LogSequenceNumber;

use super::{PageWalBasis, PhysicalRedoLsn};
use crate::physical_runtime::durability::{CertifiedPriorPageBasis, PhysicalDataFrameIdentity};

#[test]
fn bounded_delta_is_a_strict_causal_extension_of_its_certified_prior() {
    let (target, prior) = materialized_prior(10);
    let basis = PageWalBasis::new(target, prior, delta(&[(0, 11), (1, 12)]), [7; 32])
        .expect("a strictly advancing new delta must be admitted");

    assert_eq!(basis.resulting_page_lsn(), PhysicalPageLsn::new(12));
    assert_eq!(basis.delta().len(), 2);
}

#[test]
fn empty_duplicate_regressing_and_lifetime_deltas_are_rejected() {
    let (target, prior) = materialized_prior(10);

    assert!(PageWalBasis::new(target, prior, delta(&[]), [7; 32]).is_none());
    assert!(PageWalBasis::new(target, prior, delta(&[(0, 11), (1, 11)]), [7; 32],).is_none());
    assert!(PageWalBasis::new(target, prior, delta(&[(0, 12), (1, 11)]), [7; 32],).is_none());
    assert!(PageWalBasis::new(target, prior, delta(&[(0, 4), (1, 11)]), [7; 32],).is_none());
}

fn materialized_prior(page_lsn: u64) -> (PhysicalDataFrameIdentity, CertifiedPriorPageBasis) {
    let format = PhysicalRecordFormatDeclaration::builder()
        .admit()
        .expect("test format");
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let segment = PhysicalSegmentId::from_raw(1).expect("segment");
    let page = PhysicalPageId::from_raw(1).expect("page");
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

fn delta(entries: &[(u32, u64)]) -> CanonicalVec<PhysicalRedoLsn> {
    CanonicalVec::try_from_sorted(
        entries
            .iter()
            .map(|(ordinal, lsn)| PhysicalRedoLsn::new(*ordinal, LogSequenceNumber::new(*lsn)))
            .collect(),
    )
    .expect("test entries are in canonical ordinal order")
}

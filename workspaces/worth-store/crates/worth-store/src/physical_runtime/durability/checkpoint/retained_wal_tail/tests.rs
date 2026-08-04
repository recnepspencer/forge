use std::num::{NonZeroU64, NonZeroUsize};

use worth_proof::NonEmpty;
use worth_store_physical_format::store_namespace::{
    ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
};
use worth_store_physical_format::{
    CheckpointRootBasis, CheckpointWalSourceRange, PhysicalCheckpointIdentity,
    PhysicalCheckpointSource,
};
use worth_store_wal::{
    LogSequenceNumber, WalLsnRange, WalSegmentArtifactIdentity, WalSegmentGeneration, WalSegmentId,
};

use super::{ContiguousRetainedWalTail, RetainedWalSegment, RetainedWalTailAdmissionDenial};
use crate::physical_runtime::RetainedWalTailLimit;

#[test]
fn canonical_tail_carries_exact_checkpoint_and_original_segment_facts() {
    let source = source(3);
    let tail = ContiguousRetainedWalTail::admit(
        source,
        LogSequenceNumber::new(8),
        nonempty(vec![segment(2, 1, 3, 5, 90), segment(3, 1, 5, 8, 110)]),
        limit(200),
    )
    .unwrap();

    assert_eq!(tail.checkpoint_source(), source);
    assert_eq!(tail.checkpoint_identity(), source.identity());
    assert_eq!(tail.checkpoint_boundary_lsn(), LogSequenceNumber::new(3));
    assert_eq!(
        tail.durable_tail_end_lsn_exclusive(),
        LogSequenceNumber::new(8)
    );
    assert_eq!(tail.retained_physical_bytes(), 200);
    assert_eq!(tail.segment_count(), NonZeroUsize::new(2).unwrap());
    assert_eq!(tail.segments()[0].artifact(), identity(2, 1));
    assert_eq!(tail.segments()[1].observed_lsn_range(), range(5, 8));
    assert_eq!(tail.segments()[1].physical_bytes(), 110);
}

#[test]
fn zero_new_record_tail_still_retains_the_nonempty_boundary_artifact() {
    let tail = ContiguousRetainedWalTail::admit(
        source(3),
        LogSequenceNumber::new(3),
        nonempty(vec![segment(1, 1, 1, 3, 90)]),
        limit(90),
    )
    .unwrap();

    assert_eq!(tail.segment_count(), NonZeroUsize::new(1).unwrap());
    assert_eq!(tail.checkpoint_boundary_lsn(), LogSequenceNumber::new(3));
    assert_eq!(
        tail.durable_tail_end_lsn_exclusive(),
        LogSequenceNumber::new(3)
    );
}

#[test]
fn adversarial_topology_and_boundary_substitutions_are_rejected() {
    let cases = [
        (
            nonempty(vec![segment(3, 1, 5, 8, 10), segment(2, 1, 3, 5, 10)]),
            RetainedWalTailAdmissionDenial::NonCanonicalOrder,
        ),
        (
            nonempty(vec![segment(2, 1, 3, 5, 10), segment(4, 1, 5, 8, 10)]),
            RetainedWalTailAdmissionDenial::ArtifactIdentityDiscontinuity,
        ),
        (
            nonempty(vec![segment(2, 1, 3, 5, 10), segment(3, 2, 5, 8, 10)]),
            RetainedWalTailAdmissionDenial::ArtifactGenerationMismatch,
        ),
        (
            nonempty(vec![segment(2, 1, 3, 5, 10), segment(3, 1, 6, 8, 10)]),
            RetainedWalTailAdmissionDenial::LsnGap,
        ),
        (
            nonempty(vec![segment(2, 1, 3, 6, 10), segment(3, 1, 5, 8, 10)]),
            RetainedWalTailAdmissionDenial::LsnOverlap,
        ),
    ];

    for (segments, expected) in cases {
        assert_eq!(
            ContiguousRetainedWalTail::admit(
                source(3),
                LogSequenceNumber::new(8),
                segments,
                limit(100),
            ),
            Err(expected)
        );
    }
    assert_eq!(
        ContiguousRetainedWalTail::admit(
            source(2),
            LogSequenceNumber::new(8),
            nonempty(vec![segment(2, 1, 3, 5, 10), segment(3, 1, 5, 8, 10)]),
            limit(100),
        ),
        Err(RetainedWalTailAdmissionDenial::CheckpointBoundaryNotCovered)
    );
    assert_eq!(
        ContiguousRetainedWalTail::admit(
            source(3),
            LogSequenceNumber::new(9),
            nonempty(vec![segment(2, 1, 3, 5, 10), segment(3, 1, 5, 8, 10)]),
            limit(100),
        ),
        Err(RetainedWalTailAdmissionDenial::DurableFrontierNotCovered)
    );
}

#[test]
fn empty_artifacts_overflow_and_tail_limit_escape_are_rejected() {
    assert_eq!(
        ContiguousRetainedWalTail::admit(
            source(3),
            LogSequenceNumber::new(5),
            nonempty(vec![segment(2, 1, 3, 5, 0)]),
            limit(10),
        ),
        Err(RetainedWalTailAdmissionDenial::EmptyArtifact)
    );
    assert_eq!(
        ContiguousRetainedWalTail::admit(
            source(3),
            LogSequenceNumber::new(8),
            nonempty(vec![segment(2, 1, 3, 5, u64::MAX), segment(3, 1, 5, 8, 1),]),
            limit(u64::MAX),
        ),
        Err(RetainedWalTailAdmissionDenial::RetainedByteCountOverflow)
    );
    assert_eq!(
        ContiguousRetainedWalTail::admit(
            source(3),
            LogSequenceNumber::new(5),
            nonempty(vec![segment(2, 1, 3, 5, 11)]),
            limit(10),
        ),
        Err(RetainedWalTailAdmissionDenial::RetainedByteLimitExceeded)
    );
}

fn source(boundary: u64) -> PhysicalCheckpointSource {
    let proposed = ProposedStoreIdentity::from_nonzero_bytes([7; 16]).unwrap();
    let stable = StoreNamespaceIdentityRecord::new(StoreNamespaceVersion::CURRENT, proposed)
        .published_identity();
    let identity = PhysicalCheckpointIdentity::new(stable, NonZeroU64::new(1).unwrap());
    PhysicalCheckpointSource::concurrent(
        identity,
        CheckpointWalSourceRange::new(1, boundary).unwrap(),
        CheckpointRootBasis::new(1, 1),
        1,
    )
}

fn nonempty(segments: Vec<RetainedWalSegment>) -> NonEmpty<RetainedWalSegment> {
    NonEmpty::try_from_vec(segments).unwrap()
}

fn segment(
    segment: u64,
    generation: u64,
    start: u64,
    end: u64,
    physical_bytes: u64,
) -> RetainedWalSegment {
    RetainedWalSegment {
        artifact: identity(segment, generation),
        observed_lsn_range: range(start, end),
        physical_bytes,
    }
}

fn identity(segment: u64, generation: u64) -> WalSegmentArtifactIdentity {
    WalSegmentArtifactIdentity::new(
        WalSegmentId::new(segment).unwrap(),
        WalSegmentGeneration::new(generation).unwrap(),
    )
}

fn range(start: u64, end: u64) -> WalLsnRange {
    WalLsnRange::new(LogSequenceNumber::new(start), LogSequenceNumber::new(end)).unwrap()
}

fn limit(bytes: u64) -> RetainedWalTailLimit {
    RetainedWalTailLimit::new(NonZeroU64::new(bytes).unwrap())
}

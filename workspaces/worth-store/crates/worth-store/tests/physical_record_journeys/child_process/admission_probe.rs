use std::{io::Write, path::Path};

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    ManifestEntryCapacity, PageFillPercent, PhysicalManifestCapacityTransition,
    PhysicalMutationIdempotencyMaterial, PhysicalMutationPreparationDenial,
    PhysicalRecordPlacementPolicy, RecordAppendBatch, RecordAppendDenial, RecordByteLimit,
    SegmentPageCount,
};
use worth_store_physical_backend::MediaOperationRole;

use super::super::{
    allocation_probe::allocations_during, configuration, durable_publication,
    serving_from_initialization,
};

pub(super) fn batch() {
    let builder = (0..u16::MAX).fold(RecordAppendBatch::builder(), |builder, _| {
        builder.push_owned(Vec::new())
    });
    let payload = vec![0x5a; 1024 * 1024];
    let (builder, allocations) = allocations_during(|| builder.push_bytes(&payload));
    let denied = matches!(
        builder.build(),
        Err(RecordAppendDenial::BatchRecordLimitExceeded)
    );
    println!(
        "C5_BATCH_ADMISSION {} {} {denied}",
        allocations.allocations, allocations.bytes_allocated,
    );
    std::io::stdout().flush().unwrap();
}

pub(super) fn geometry(root: &Path) {
    let (format, _, _) = configuration();
    let placement = PhysicalRecordPlacementPolicy::builder()
        .segment_pages(SegmentPageCount::new(4).unwrap())
        .extent_threshold(RecordByteLimit::new(8_000).unwrap())
        .page_fill(PageFillPercent::new(1).unwrap())
        .manifest_capacity(ManifestEntryCapacity::new(64).unwrap())
        .admit(format)
        .unwrap();
    let serving = serving_from_initialization(root);
    let batch = RecordAppendBatch::builder()
        .push_source(super::super::stream_fixture::PatternSource::exact(200))
        .build()
        .unwrap();
    let writes_before = serving
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedWrite);
    let (result, allocations) = allocations_during(|| {
        durable_publication::prepare_single(
            &serving.record_submission(),
            placement,
            PhysicalManifestCapacityTransition::PreserveCurrent,
            PhysicalMutationIdempotencyMaterial::new([211; 32]),
            batch,
        )
        .into_raw()
    });
    let writes_after = serving
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedWrite);
    println!(
        "C5_GEOMETRY_ADMISSION {} {} {} {}",
        allocations.allocations,
        allocations.bytes_allocated,
        writes_after - writes_before,
        matches!(
            result,
            TransitionOutcome::Denied(PhysicalMutationPreparationDenial::RecordAppend(
                RecordAppendDenial::InlinePageFull
            ))
        )
    );
    std::io::stdout().flush().unwrap();
    serving.close();
}

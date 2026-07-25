use worth_foundational::{
    compare_canonical_basis, prepare_canonical_basis_sequence, prepare_canonical_comparison,
    CanonicalBasisEntry, CanonicalBasisLocus, CanonicalBasisValue, CanonicalComparisonOutcome,
    CanonicalEquivalenceBasis, CanonicalIntegerWidth, CanonicalMismatchKind,
};
use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    lower_offline_record_publication_canonical_basis, lower_record_operation_performance_receipt,
    lower_record_publication_canonical_basis, PhysicalRecordAccessSummary,
    PhysicalRecordPerformanceContract, RecordAppendBatch, RecordAppendPerformanceExpectation,
    RecordByteLimit, RecordLocatePerformanceExpectation, RecordManifestPerformanceExpectation,
    RecordReadLimits, RecordScanOutcome, RecordScanPerformanceExpectation, RecordScanRequest,
    RecordTransferPerformanceExpectation,
};

use super::{configuration, serving_from_initialization};

#[test]
fn runtime_and_offline_topology_have_canonical_parity() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, _) = configuration();
    let serving = serving_from_initialization(&root);
    let inline = (0..70).map(|ordinal| vec![ordinal as u8; 257]);
    serving
        .record_submission()
        .append_batch(RecordAppendBatch::try_from_iter(inline).unwrap(), placement)
        .unwrap();
    serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([vec![0x91; 20_000]]).unwrap(),
            placement,
        )
        .unwrap();

    let runtime = serving.certification_publication_summary().unwrap();
    let offline = worth_store_offline_verifier::walk_current_durable_record_manifest(
        &root,
        format.declaration(),
    )
    .unwrap();
    let runtime = success(lower_record_publication_canonical_basis(&runtime));
    let offline = success(lower_offline_record_publication_canonical_basis(&offline));
    assert!(matches!(
        compare(runtime.clone(), offline),
        CanonicalComparisonOutcome::Equivalent(_)
    ));

    let mut changed = runtime.payload().entries().to_vec();
    let target = changed
        .iter_mut()
        .find(|entry| entry.locus() == &CanonicalBasisLocus::Named("root.generation".into()))
        .unwrap();
    *target = CanonicalBasisEntry::new(
        target.domain(),
        target.locus().clone(),
        target.kind(),
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: 99,
        },
    );
    let divergent = success(prepare_canonical_basis_sequence(
        runtime.payload().version().clone(),
        runtime.payload().domain(),
        changed,
    ));
    match compare(runtime, divergent) {
        CanonicalComparisonOutcome::Mismatched(mismatch) => {
            assert_eq!(mismatch.kind(), CanonicalMismatchKind::ValueMismatch);
            assert_eq!(
                mismatch.left_locus(),
                Some(&CanonicalBasisLocus::Named("root.generation".into()))
            );
        }
        other => panic!("one-field topology divergence escaped: {other:?}"),
    }
    serving.close();
}

#[test]
fn counter_receipt_rejects_missing_duplicate_and_mismatched_rows() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (_, placement, _) = configuration();
    let serving = serving_from_initialization(&root);
    let published = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"counter receipt".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    let append_summary = PhysicalRecordAccessSummary::from_published_batch(&published);
    assert!(matches!(
        lower_record_operation_performance_receipt(
            PhysicalRecordPerformanceContract::append(zero_append_expectation()),
            append_summary,
        ),
        Err(worth_store::physical_runtime::RecordPerformanceEvidenceDenial::Receipt(_))
    ));
    let append = lower_record_operation_performance_receipt(
        PhysicalRecordPerformanceContract::append(RecordAppendPerformanceExpectation {
            records: 1,
            payload_bytes: 15,
            manifest: manifest_expectation(1, 120, 0),
            allocated_segments: 1,
            allocated_extents: 0,
            transfer: transfer_expectation(8, 16_384, 1, 15, 16_384),
            file_barriers: 29,
            directory_barriers: 52,
            catalog_replacements: 1,
        }),
        append_summary,
    )
    .unwrap();
    assert_eq!(append.counter_rows().len(), 20);
    assert_rows_fail_closed(&append);

    let record = published.record_id(0).unwrap();
    serving.drain_clean_residency();
    let mut read = serving
        .records()
        .open(
            record,
            RecordReadLimits::new(RecordByteLimit::new(128).unwrap()),
        )
        .unwrap();
    let mut scratch = [0_u8; 64];
    while read.read_next(&mut scratch).unwrap() != 0 {}
    let read = PhysicalRecordAccessSummary::from_completed_read(read.observation()).unwrap();
    lower_record_operation_performance_receipt(
        PhysicalRecordPerformanceContract::locate(RecordLocatePerformanceExpectation {
            payload_bytes: 15,
            manifest: manifest_expectation(2, 288, 2),
            transfer: transfer_expectation(1, 16_384, 1, 15, 0),
            frames_traversed: 3,
        }),
        read,
    )
    .unwrap();

    let mut scan = serving
        .records()
        .scan(RecordScanRequest::from_start())
        .unwrap();
    let completed = loop {
        match scan.read_next_into(&mut scratch).unwrap() {
            RecordScanOutcome::Batch(_) => {}
            RecordScanOutcome::Completed(completed) => break completed,
        }
    };
    let scan = PhysicalRecordAccessSummary::from_completed_scan(completed);
    lower_record_operation_performance_receipt(
        PhysicalRecordPerformanceContract::scan(RecordScanPerformanceExpectation {
            records: 1,
            payload_bytes: 15,
            manifest: manifest_expectation(2, 288, 1),
            transfer: transfer_expectation(1, 16_384, 1, 15, 0),
            frames_traversed: 3,
        }),
        scan,
    )
    .unwrap();
    serving.close();
}

fn zero_append_expectation() -> RecordAppendPerformanceExpectation {
    RecordAppendPerformanceExpectation {
        records: 0,
        payload_bytes: 0,
        manifest: manifest_expectation(0, 0, 0),
        allocated_segments: 0,
        allocated_extents: 0,
        transfer: transfer_expectation(0, 0, 0, 0, 0),
        file_barriers: 0,
        directory_barriers: 0,
        catalog_replacements: 0,
    }
}

const fn manifest_expectation(
    blocks: u64,
    bytes: u64,
    comparisons: u64,
) -> RecordManifestPerformanceExpectation {
    RecordManifestPerformanceExpectation {
        blocks,
        bytes,
        comparisons,
    }
}

const fn transfer_expectation(
    transfers: u64,
    peak_transfer_bytes: u64,
    explicit_copies: u64,
    copied_bytes: u64,
    peak_scratch_bytes: u64,
) -> RecordTransferPerformanceExpectation {
    RecordTransferPerformanceExpectation {
        transfers,
        peak_transfer_bytes,
        explicit_copies,
        copied_bytes,
        peak_scratch_bytes,
    }
}

fn compare(
    left: worth_foundational::CanonicalBasisReadyArtifact,
    right: worth_foundational::CanonicalBasisReadyArtifact,
) -> CanonicalComparisonOutcome {
    let ready = success(prepare_canonical_comparison(
        CanonicalEquivalenceBasis::ExactCanonicalBasis,
        left,
        right,
    ));
    compare_canonical_basis(&ready)
}

fn success<T, D>(outcome: TransitionOutcome<T, D>) -> T {
    match outcome {
        TransitionOutcome::Success(value) => value,
        _ => panic!("evidence construction failed"),
    }
}

fn assert_rows_fail_closed(receipt: &worth_store::physical_runtime::StoreRecordPerformanceReceipt) {
    use worth_foundational::performance_api::lower_lane::receipts;
    use worth_foundational::{
        FoundationalCounterBackedPerformanceReceiptConstructionDenial as Denial,
        FoundationalPerformanceCounterRow,
    };

    assert!(matches!(
        receipts::counter_backed_performance_receipt(receipt.bundle().clone()).finish(),
        Err(Denial::MissingCounterRowForSpec)
    ));
    let first = receipt.counter_rows()[0].clone();
    assert!(matches!(
        receipts::counter_backed_performance_receipt(receipt.bundle().clone())
            .attach_counter_row(first.clone())
            .attach_counter_row(first)
            .finish(),
        Err(Denial::DuplicateCounterRow)
    ));
    let mut mismatched = receipts::counter_backed_performance_receipt(receipt.bundle().clone());
    for (index, row) in receipt.counter_rows().iter().enumerate() {
        mismatched = mismatched.attach_counter_row(FoundationalPerformanceCounterRow::new(
            row.name().clone(),
            row.observed_count() + u64::from(index == 0),
        ));
    }
    assert!(matches!(
        mismatched.finish(),
        Err(Denial::CounterValueMismatch)
    ));
}

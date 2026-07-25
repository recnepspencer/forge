use worth_store::physical_runtime::{
    ExternalPhysicalRecordLocator, PhysicalRecordInitialization, RecordAppendBatch,
};

use super::super::{
    media,
    scenario_configuration::dense_configuration,
    stream_fixture::{hex, pattern_digest, PatternSource},
    success,
};

#[test]
fn mixed_batch_streams_extent_and_a_fresh_process_reads_with_seventeen_widths() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(4);
    let serving = success(
        media(&root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let logical_bytes = 1_048_613_u64;
    let batch = RecordAppendBatch::builder()
        .push_bytes(b"inline")
        .push_source(PatternSource::exact(logical_bytes))
        .build()
        .unwrap();
    let published = serving
        .record_submission()
        .append_batch(batch, placement)
        .unwrap();
    let observation = published.observation();
    let chunks = logical_bytes.div_ceil(16_384 - 104);
    assert_eq!(observation.extent_artifacts(), 1);
    assert_eq!(observation.bytes_requested(), logical_bytes + 6);
    assert_eq!(observation.bytes_completed(), logical_bytes + 6);
    assert_eq!(observation.transfer_count(), chunks + 9);
    assert_eq!(observation.peak_transfer_width(), 16_384);
    assert_eq!(observation.peak_scratch_bytes(), 16_384);
    assert_eq!(observation.explicit_copy_count(), chunks + 1);
    assert_eq!(observation.copied_bytes(), logical_bytes + 6);
    let locator = ExternalPhysicalRecordLocator::new(
        serving.store_identity(),
        published.record_id(1).unwrap(),
    );
    serving.close();
    let extent =
        root.join("families/records/extents/extent-0000000000000001-0000000000000001.data");
    assert_eq!(
        std::fs::metadata(extent).unwrap().len(),
        logical_bytes + chunks * 104
    );
    let output = super::super::child_process::run_child(
        "extent_reader",
        &root,
        Some(&hex(&locator.encode())),
    );
    assert!(output.contains(&format!(
        "C5_EXTENT {logical_bytes} {}",
        pattern_digest(logical_bytes)
    )));
    assert!(output.contains(&format!(
        "C5_EXTENT_OBS {logical_bytes} {logical_bytes} {} 16384 {logical_bytes} {} 0",
        chunks + 1,
        chunks + 1,
    )));
}

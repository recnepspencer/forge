use std::io::{Seek, SeekFrom, Write};

use worth_store::physical_runtime::{
    PhysicalRecordInitialization, RecordAppendBatch, RecordAppendDenial, RecordAppendError,
    RecordByteLimit, RecordReadLimits, RecordServingTerminalPosture, RecordStreamFailureKind,
};

use super::super::{
    media, scenario_configuration::dense_configuration, stream_fixture::PatternSource, success,
};

#[test]
fn streamed_read_damage_retains_the_completed_logical_range() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(4);
    let serving = success(initialize_record_store!(media(&root), |durability| {
        PhysicalRecordInitialization::new(format, placement, access, durability)
    }));
    let published = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::builder()
                .push_source(PatternSource::exact(40_000))
                .build()
                .unwrap(),
            placement,
        )
        .unwrap();
    let path = root.join("families/records/extents/extent-0000000000000001-0000000000000001.data");
    let mut file = std::fs::OpenOptions::new().write(true).open(path).unwrap();
    file.seek(SeekFrom::Start((16_384 + 120) as u64)).unwrap();
    file.write_all(&[0xa5]).unwrap();
    file.sync_all().unwrap();
    assert!(
        serving
            .certification_physical_residency()
            .drain_unpinned_clean_frames()
            > 0
    );
    let mut session = serving
        .records()
        .open(
            published.record_id(0).unwrap(),
            RecordReadLimits::new(RecordByteLimit::new(40_000).unwrap()),
        )
        .unwrap();
    let mut first = vec![0_u8; 16_384 - 104];
    assert_eq!(session.read_next(&mut first).unwrap(), 16_384 - 104);
    let failure = session.read_next(&mut [0_u8; 1]).unwrap_err();
    assert_eq!(failure.kind(), RecordStreamFailureKind::ArtifactDamaged);
    assert_eq!(failure.completed_range(), 0..16_280);
    assert_eq!(session.observation().generation_checks(), 2);
    assert_eq!(session.observation().generation_rejections(), 0);
    drop(session);
    assert_eq!(
        serving
            .record_submission()
            .append_batch(
                RecordAppendBatch::try_from_iter([b"retry".as_slice()]).unwrap(),
                placement,
            )
            .unwrap_err(),
        RecordAppendError::Denied(RecordAppendDenial::ServingRequiresInspection)
    );
    assert_eq!(
        serving.abort().records().posture(),
        RecordServingTerminalPosture::InspectionRequired
    );
}

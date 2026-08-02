use std::io::Write;

use worth_store_lsm_authority::{LsmMembershipArtifactDeclaration, LsmMembershipKey};
use worth_store_wal::{
    observe_wal_frame_artifact, prepare_wal_frame_append, BlobWalRecordEnvelope,
    BlobWalRecordIdentity, BlobWalRecordKind, LogSequenceNumber, PublicationDeclaration,
    WalFrameArtifactObservation, WalFramePublicationScope, WalLsnRange, WalSegmentGeneration,
    WalSegmentId,
};

pub(crate) fn durable_record_binding(
    key: LsmMembershipKey,
    sequence: u64,
    kind: BlobWalRecordKind,
) -> (BlobWalRecordEnvelope, WalFrameArtifactObservation) {
    durable_record_binding_for_store(key, sequence, kind, 1, 1)
}

pub(crate) fn durable_record_binding_for_store(
    key: LsmMembershipKey,
    sequence: u64,
    kind: BlobWalRecordKind,
    segment_id: u64,
    generation: u64,
) -> (BlobWalRecordEnvelope, WalFrameArtifactObservation) {
    let payload_digest = format!("lsm-input:{sequence}:{kind:?}:lsn={sequence}");
    let scope = WalFramePublicationScope::new(
        WalSegmentId::new(segment_id).expect("nonzero fixture WAL segment"),
        WalSegmentGeneration::new(generation).expect("nonzero fixture WAL generation"),
        WalLsnRange::new(
            LogSequenceNumber::new(sequence),
            LogSequenceNumber::new(sequence.checked_add(1).expect("bounded fixture LSN")),
        )
        .expect("nonempty fixture WAL range"),
        payload_digest.clone(),
        4096,
    )
    .expect("WAL frame scope");
    let envelope = BlobWalRecordEnvelope::new(
        BlobWalRecordIdentity::new(sequence, kind).expect("record identity"),
        PublicationDeclaration::wal_frame(scope.clone()),
        payload_digest,
    )
    .expect("WAL envelope");
    let artifact = LsmMembershipArtifactDeclaration::record(&envelope, key);
    let observation = wal_artifact_observation(scope, artifact.bytes());
    (envelope, observation)
}

pub(super) fn wal_scope(
    sequence: u64,
    frame_digest: String,
    expected_bytes: u64,
) -> WalFramePublicationScope {
    WalFramePublicationScope::new(
        WalSegmentId::new(1).unwrap(),
        WalSegmentGeneration::new(1).unwrap(),
        WalLsnRange::new(
            LogSequenceNumber::new(sequence),
            LogSequenceNumber::new(sequence.checked_add(1).expect("bounded fixture sequence")),
        )
        .expect("nonempty fixture WAL range"),
        frame_digest,
        expected_bytes,
    )
    .expect("WAL frame scope")
}

pub(crate) fn wal_artifact_observation(
    scope: WalFramePublicationScope,
    artifact: &[u8],
) -> WalFrameArtifactObservation {
    execution_directory(|directory| {
        let append = prepare_wal_frame_append(
            directory,
            scope.segment_id(),
            scope.generation(),
            scope.lsn_start(),
            scope.lsn_end(),
            scope.frame_digest(),
            artifact,
        )
        .expect("valid WAL frame append");
        let path = directory.join(append.relative_path());
        std::fs::create_dir_all(path.parent().expect("WAL path parent"))
            .expect("create fixture WAL directory");
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .expect("open fixture WAL artifact");
        let frame_offset = append.observed_file_bytes();
        file.write_all(append.encoded_frame())
            .expect("write fixture WAL frame");
        drop(file);
        observe_wal_frame_artifact(
            &path,
            frame_offset,
            append.encoded_frame().len() as u64,
            &scope,
        )
        .expect("observe fixture WAL frame")
    })
}

thread_local! {
    static FIXTURE_DIRECTORY: std::cell::RefCell<Option<crate::TemporaryDirectory>> = const {
        std::cell::RefCell::new(None)
    };
}

pub(super) fn begin_durability_fixture() -> crate::TemporaryDirectory {
    let directory = crate::TemporaryDirectory::create("lsm-artifact-observation")
        .expect("cannot create LSM artifact observation directory");
    FIXTURE_DIRECTORY.with(|slot| {
        *slot.borrow_mut() = Some(directory.clone());
    });
    directory
}

fn execution_directory<T>(execute: impl FnOnce(&std::path::Path) -> T) -> T {
    FIXTURE_DIRECTORY.with(|slot| {
        if slot.borrow().is_none() {
            let _directory = begin_durability_fixture();
        }
        let borrowed = slot.borrow();
        execute(borrowed.as_ref().expect("fixture directory").path())
    })
}

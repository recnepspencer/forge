use std::sync::mpsc;
use std::time::Duration;

use worth_store::physical_runtime::certification::CertificationPhysicalExecutionCheckpoint;
use worth_store::physical_runtime::{PhysicalRecordInitialization, RecordAppendBatch};
use worth_store_physical_backend::MediaOperationRole;

use super::{configuration, media, success};

#[test]
fn catalog_replacement_checkpoint_blocks_publication_completion() {
    let root = tempfile::tempdir().unwrap();
    let (format, placement, access) = configuration();
    let serving = success(
        media(root.path())
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let gate = serving.certification_pause_physical_execution_at(
        CertificationPhysicalExecutionCheckpoint::AfterCatalogReplacementBeforeSchedulerSettlement,
    );
    let replacements_before = serving
        .media_counters()
        .attempts_for(MediaOperationRole::AtomicReplace);
    let (finished, completion) = mpsc::sync_channel(1);

    std::thread::scope(|scope| {
        scope.spawn(|| {
            let result = serving.record_submission().append_batch(
                RecordAppendBatch::try_from_iter([b"published".as_slice()]).unwrap(),
                placement,
            );
            finished.send(result).unwrap();
        });
        assert!(gate.await_arrival());
        assert_eq!(
            serving
                .media_counters()
                .attempts_for(MediaOperationRole::AtomicReplace),
            replacements_before + 1
        );
        assert!(completion.recv_timeout(Duration::from_millis(20)).is_err());
        gate.release();
        assert!(completion
            .recv_timeout(Duration::from_secs(2))
            .unwrap()
            .is_ok());
    });
    serving.close();
}

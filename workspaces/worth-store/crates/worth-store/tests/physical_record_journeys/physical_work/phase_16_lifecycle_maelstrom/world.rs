use std::path::PathBuf;

use worth_store::physical_runtime::{PhysicalWorkObservation, ServingPhysicalRuntime};
use worth_store_physical_backend::MediaOperationRole;

use super::fixture::{MaelstromFixture, MaelstromPauseGates};

pub(super) struct MaelstromWorld {
    _parent: tempfile::TempDir,
    pub root: PathBuf,
    pub fixture: MaelstromFixture,
    pub serving: ServingPhysicalRuntime,
    pub gates: MaelstromPauseGates,
    pub observer: PhysicalWorkObservation,
    pub catalog_path: PathBuf,
    pub original_writeback: Vec<u8>,
}

pub(super) fn open() -> MaelstromWorld {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let fixture = super::fixture::maelstrom_fixture();
    assert!(fixture.read_delta.is_partitioned());
    super::super::fixture::serving_from_initialization_with_work_profile(
        &root,
        fixture.profile.clone(),
    )
    .close();
    let calibration =
        super::super::fixture::serving_from_open_with_work_profile(&root, fixture.profile.clone());
    let calibration_counters = calibration.media_counters();
    let identified_reads_after_open =
        calibration_counters.identified_operation_attempts_for(MediaOperationRole::PositionedRead);
    let identified_writes_after_open =
        calibration_counters.identified_operation_attempts_for(MediaOperationRole::PositionedWrite);
    calibration.close();
    let catalog_path = root.join("families/records/bootstrap.catalog");
    let original_writeback = std::fs::read(&catalog_path).unwrap()[8..16].to_vec();
    let (serving, gates) = super::fixture::open_with_maelstrom_faults(
        &root,
        fixture.profile.clone(),
        identified_reads_after_open,
        identified_writes_after_open,
    );
    assert_open_counters(
        &serving,
        identified_reads_after_open,
        identified_writes_after_open,
    );
    let observer = serving.physical_work_observer();
    MaelstromWorld {
        _parent: parent,
        root,
        fixture,
        serving,
        gates,
        observer,
        catalog_path,
        original_writeback,
    }
}

fn assert_open_counters(
    serving: &ServingPhysicalRuntime,
    identified_reads: u64,
    identified_writes: u64,
) {
    assert_eq!(
        serving
            .media_counters()
            .identified_operation_attempts_for(MediaOperationRole::PositionedRead),
        identified_reads
    );
    assert_eq!(
        serving
            .media_counters()
            .identified_operation_attempts_for(MediaOperationRole::PositionedWrite),
        identified_writes
    );
}

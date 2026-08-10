use crate::{
    AuthoritativeExportBundle, Milestone13CertificationBundle, Milestone13CounterContract,
    WORTHStore, WORTHStoreBuilder,
};

use super::super::super::harness::fixtures::stores::{
    unique_test_sqlite_path, unique_test_store_path,
};
use super::super::counter_lane::interleaving_counter_lane;
use super::super::foreground_lanes::{
    continuation_interleaving_lane, foreground_read_interleaving_lane,
};
use super::super::tiering_lanes::{
    execute_tiering_batch, interleaved_tiering_lane, recalled_tiering_lane,
};
use super::super::world::build_store;

pub(super) struct Milestone13LaneEvidence {
    pub(super) baseline: BaselineEvidence,
    pub(super) restart: RestartEvidence,
    pub(super) interleaving: InterleavingEvidence,
}

pub(super) struct BaselineEvidence {
    pub(super) control_bundle: Milestone13CertificationBundle,
    pub(super) moved_bundle: Milestone13CertificationBundle,
    pub(super) expected_counter_contract: String,
}

pub(super) struct RestartEvidence {
    pub(super) sqlite: SqliteRestartEvidence,
    pub(super) local_file: LocalFileRestartEvidence,
}

pub(super) struct SqliteRestartEvidence {
    pub(super) moved_bundle: Milestone13CertificationBundle,
    pub(super) reopened_bundle: Milestone13CertificationBundle,
    pub(super) before_reopen_manifest: String,
    pub(super) after_reopen_manifest: String,
}

pub(super) struct LocalFileRestartEvidence {
    pub(super) moved_bundle: Milestone13CertificationBundle,
    pub(super) reopened_bundle: Milestone13CertificationBundle,
    pub(super) before_reopen_manifest: String,
    pub(super) after_reopen_manifest: String,
}

pub(super) struct InterleavingEvidence {
    pub(super) recalled_bundle: Milestone13CertificationBundle,
    pub(super) interleaved_bundle: Milestone13CertificationBundle,
    pub(super) foreground_interleaved_bundle: Milestone13CertificationBundle,
    pub(super) continuation_interleaved_bundle: Milestone13CertificationBundle,
    pub(super) interleaving_counter_contract: Milestone13CounterContract,
}

struct BaselineAndMovedEvidence {
    control_export: AuthoritativeExportBundle,
    control_bundle: Milestone13CertificationBundle,
    moved_bundle: Milestone13CertificationBundle,
    moved_store: WORTHStore,
}

pub(super) fn collect_milestone_13_lane_evidence() -> Milestone13LaneEvidence {
    let baseline_and_moved = collect_baseline_and_moved_evidence();
    let sqlite = collect_sqlite_moved_and_reopened_evidence(&baseline_and_moved.control_export);
    let local_file =
        collect_local_file_moved_and_reopened_evidence(&baseline_and_moved.control_export);
    let interleaving =
        collect_recalled_interleaving_and_counter_evidence(&baseline_and_moved.control_export);
    let expected_counter_contract =
        serialize_expected_moved_counter(&baseline_and_moved.moved_store);

    Milestone13LaneEvidence {
        baseline: BaselineEvidence {
            control_bundle: baseline_and_moved.control_bundle,
            moved_bundle: baseline_and_moved.moved_bundle,
            expected_counter_contract,
        },
        restart: RestartEvidence { sqlite, local_file },
        interleaving,
    }
}

fn collect_baseline_and_moved_evidence() -> BaselineAndMovedEvidence {
    let (control_store, _) = build_store(WORTHStoreBuilder::new().in_memory());
    let control_export = control_store.export_authoritative_records();
    let control_bundle = control_store
        .milestone_13_certification_bundle(&control_export)
        .unwrap();

    let (mut moved_store, moved_snapshot_id) = build_store(WORTHStoreBuilder::new().in_memory());
    execute_tiering_batch(&mut moved_store, moved_snapshot_id);
    let moved_bundle = moved_store
        .milestone_13_certification_bundle(&control_export)
        .unwrap();

    BaselineAndMovedEvidence {
        control_export,
        control_bundle,
        moved_bundle,
        moved_store,
    }
}

fn collect_sqlite_moved_and_reopened_evidence(
    control_export: &AuthoritativeExportBundle,
) -> SqliteRestartEvidence {
    let sqlite_path = unique_test_sqlite_path("worth-store-m13-certification");
    let (mut sqlite_store, sqlite_snapshot_id) =
        build_store(WORTHStoreBuilder::new().sqlite_file(sqlite_path.clone()));
    execute_tiering_batch(&mut sqlite_store, sqlite_snapshot_id);
    let moved_bundle = sqlite_store
        .milestone_13_certification_bundle(control_export)
        .unwrap();
    let before_reopen_manifest =
        serde_json::to_string(&sqlite_store.recover_tiering_state().unwrap()).unwrap();
    drop(sqlite_store);

    let reopened_store = WORTHStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let reopened_bundle = reopened_store
        .milestone_13_certification_bundle(control_export)
        .unwrap();
    let after_reopen_manifest =
        serde_json::to_string(&reopened_store.recover_tiering_state().unwrap()).unwrap();

    SqliteRestartEvidence {
        moved_bundle,
        reopened_bundle,
        before_reopen_manifest,
        after_reopen_manifest,
    }
}

fn collect_local_file_moved_and_reopened_evidence(
    control_export: &AuthoritativeExportBundle,
) -> LocalFileRestartEvidence {
    let local_path = unique_test_store_path("worth-store-m13-certification-local");
    let (mut local_store, local_snapshot_id) =
        build_store(WORTHStoreBuilder::new().local_file(local_path.clone()));
    execute_tiering_batch(&mut local_store, local_snapshot_id);
    let moved_bundle = local_store
        .milestone_13_certification_bundle(control_export)
        .unwrap();
    let before_reopen_manifest =
        serde_json::to_string(&local_store.recover_tiering_state().unwrap()).unwrap();
    drop(local_store);

    let reopened_store = WORTHStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let reopened_bundle = reopened_store
        .milestone_13_certification_bundle(control_export)
        .unwrap();
    let after_reopen_manifest =
        serde_json::to_string(&reopened_store.recover_tiering_state().unwrap()).unwrap();

    LocalFileRestartEvidence {
        moved_bundle,
        reopened_bundle,
        before_reopen_manifest,
        after_reopen_manifest,
    }
}

fn collect_recalled_interleaving_and_counter_evidence(
    control_export: &AuthoritativeExportBundle,
) -> InterleavingEvidence {
    let (recalled_store, _) = recalled_tiering_lane(WORTHStoreBuilder::new().in_memory());
    let recalled_bundle = recalled_store
        .milestone_13_certification_bundle(control_export)
        .unwrap();

    let (interleaved_store, _) = interleaved_tiering_lane(WORTHStoreBuilder::new().in_memory());
    let interleaved_bundle = interleaved_store
        .milestone_13_certification_bundle(control_export)
        .unwrap();

    let foreground_interleaved_store =
        foreground_read_interleaving_lane(WORTHStoreBuilder::new().in_memory());
    let foreground_interleaved_bundle = foreground_interleaved_store
        .milestone_13_certification_bundle(control_export)
        .unwrap();

    let continuation_interleaved_store =
        continuation_interleaving_lane(WORTHStoreBuilder::new().in_memory());
    let continuation_interleaved_bundle = continuation_interleaved_store
        .milestone_13_certification_bundle(control_export)
        .unwrap();

    let interleaving_counter_store =
        interleaving_counter_lane(WORTHStoreBuilder::new().in_memory());
    let interleaving_counter_contract = interleaving_counter_store.milestone_13_counter_contract();

    InterleavingEvidence {
        recalled_bundle,
        interleaved_bundle,
        foreground_interleaved_bundle,
        continuation_interleaved_bundle,
        interleaving_counter_contract,
    }
}

fn serialize_expected_moved_counter(store: &WORTHStore) -> String {
    serde_json::to_string(&store.milestone_13_counter_contract()).unwrap()
}

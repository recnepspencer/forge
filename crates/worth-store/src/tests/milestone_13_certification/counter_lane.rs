use crate::{
    ContinuationRetentionStatus, PlacementBoundArtifactRef, PlacementExecutionOrigin,
    PlacementRaceOutcome, SnapshotCaptureRequest, WORTHStore, WORTHStoreBuilder,
};

use super::super::live_query::helpers::stable_basis_request_for_store;
use super::foreground_lanes::continuation_interleaving_lane;

pub(super) fn interleaving_counter_lane(builder: WORTHStoreBuilder) -> WORTHStore {
    let mut store = continuation_interleaving_lane(builder);
    let export = store.export_authoritative_records().into_canonicalized();
    let envelope = export.commit_envelopes.first().unwrap().envelope.clone();
    let basis = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            envelope.branch_context.clone(),
            envelope.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    let foreground = store.observe_stable_basis_interleaving(&basis).unwrap();
    assert_eq!(
        foreground.observation().race_outcome(),
        PlacementRaceOutcome::TransferObserved
    );

    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            envelope.branch_context.clone(),
            envelope.commit.commit_id,
        ))
        .unwrap();
    store
        .admit_inflight_cold_recall(
            PlacementBoundArtifactRef::snapshot_family(snapshot.snapshot_id.0.to_string()),
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    let cold = store
        .plan_cold_recall_lease(
            PlacementBoundArtifactRef::snapshot_family(snapshot.snapshot_id.0.to_string()),
            PlacementExecutionOrigin::Foreground,
        )
        .unwrap();
    let handle = store.resolve_cold_recall_read_handle(cold.cold_recall_lease().unwrap());
    let report = store.observe_placement_read_interleaving(&handle).unwrap();
    assert_eq!(
        report.observation().race_outcome(),
        PlacementRaceOutcome::RecallObserved
    );
    store
}

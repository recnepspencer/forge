use crate::application::WorthQueryDomainOperatingContext;
use crate::continuation_pipeline::WorthQueryContinuationExecutionOutcome;
use crate::ordinary_outcome::{
    WorthQueryOrdinaryContinuationCheckedTopologyKind, WorthQueryOrdinaryOutcome,
};

use super::support::{
    admitted_handle, drifted_readmission_handle, historical_disabled_handle,
    historical_truth_view_request, preview_disabled_handle, preview_session_request,
    runtime_route_request, target_request, HistoricalFamily, PreviewFamily, ReadmissionDrift,
    RuntimeFamily,
};

#[test]
fn execution_stays_separate_from_preparation_and_produces_runtime_artifact() {
    let handle = admitted_handle("main");
    let prepared = match handle.prepare_continuation_from_target(target_request::<RuntimeFamily>(
        &handle,
        "face-a",
        runtime_route_request(),
    )) {
        crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::Prepared(prepared) => {
            prepared
        }
        _ => panic!("expected prepared continuation"),
    };
    let prepared_digest = prepared.prepared_digest().to_string();

    let executed = match handle.execute_prepared_continuation(prepared) {
        WorthQueryContinuationExecutionOutcome::Executed(executed) => executed,
        _ => panic!("expected executed continuation"),
    };

    assert_eq!(
        executed.family(),
        crate::continuation_pipeline::WorthQueryPreparedContinuationFamily::BridgeRuntimeRoute
    );
    assert_ne!(executed.execution_digest(), prepared_digest);
    assert_eq!(executed.prepared().prepared_digest(), prepared_digest);
}

#[test]
fn preparation_and_execution_both_preserve_wrong_world_when_world_changes() {
    let left = admitted_handle("left");
    let right = admitted_handle("right");

    let wrong_world = right.prepare_continuation_from_target(target_request::<RuntimeFamily>(
        &left,
        "face-a",
        runtime_route_request(),
    ));
    assert!(matches!(
        wrong_world,
        crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::WrongWorld(_)
    ));

    let prepared = match left.prepare_continuation_from_target(target_request::<RuntimeFamily>(
        &left,
        "face-a",
        runtime_route_request(),
    )) {
        crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::Prepared(prepared) => {
            prepared
        }
        _ => panic!("expected prepared continuation"),
    };
    let execution = right.execute_prepared_continuation(prepared);
    assert!(matches!(
        execution,
        WorthQueryContinuationExecutionOutcome::WrongWorld(_)
    ));
}

#[test]
fn continuation_ordinary_outcome_keeps_execution_topology_honest() {
    let handle = admitted_handle("main");
    let prepared =
        match handle.prepare_continuation_from_target_outcome(target_request::<RuntimeFamily>(
            &handle,
            "face-a",
            runtime_route_request(),
        )) {
            WorthQueryOrdinaryOutcome::Bound(prepared) => prepared,
            _ => panic!("expected bound continuation outcome"),
        };

    assert!(matches!(
        handle.execute_prepared_continuation_outcome(prepared),
        WorthQueryOrdinaryOutcome::Bound(_)
    ));

    match admitted_handle("right").execute_prepared_continuation_outcome(
        match handle.prepare_continuation_from_target(target_request::<RuntimeFamily>(
            &handle,
            "face-a",
            runtime_route_request(),
        )) {
            crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::Prepared(
                prepared,
            ) => prepared,
            _ => panic!("expected prepared continuation"),
        },
    ) {
        WorthQueryOrdinaryOutcome::WrongWorld(posture) => {
            assert_eq!(
                posture.checked_topology().continuation_kind(),
                Some(WorthQueryOrdinaryContinuationCheckedTopologyKind::WrongWorld)
            );
        }
        _ => panic!("expected wrong-world execution outcome"),
    }
}

#[test]
fn execution_rechecks_capability_support_for_preview_and_historical_paths() {
    let preview_source = admitted_handle("shared");
    let preview_prepared =
        match preview_source.prepare_continuation_from_target(target_request::<PreviewFamily>(
            &preview_source,
            "face-a",
            preview_session_request(),
        )) {
            crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::Prepared(
                prepared,
            ) => prepared,
            _ => panic!("expected prepared preview continuation"),
        };
    assert!(matches!(
        preview_disabled_handle("shared").execute_prepared_continuation(preview_prepared),
        WorthQueryContinuationExecutionOutcome::Unsupported(_)
    ));

    let historical_source = admitted_handle("shared");
    let historical_prepared = match historical_source.prepare_continuation_from_target(
        target_request::<HistoricalFamily>(
            &historical_source,
            "face-a",
            historical_truth_view_request(),
        ),
    ) {
        crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::Prepared(prepared) => {
            prepared
        }
        _ => panic!("expected prepared historical continuation"),
    };
    assert!(matches!(
        historical_disabled_handle("shared").execute_prepared_continuation(historical_prepared),
        WorthQueryContinuationExecutionOutcome::Unsupported(_)
    ));
}

#[test]
fn execution_stops_as_stale_when_retained_basis_evidence_is_stale() {
    let handle = admitted_handle("main");
    let prepared = match handle.prepare_continuation_from_target(target_request::<RuntimeFamily>(
        &handle,
        "face-a",
        runtime_route_request(),
    )) {
        crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::Prepared(prepared) => {
            prepared
        }
        _ => panic!("expected prepared continuation"),
    };

    assert!(matches!(
        drifted_readmission_handle("main", ReadmissionDrift::Stale)
            .execute_prepared_continuation(prepared),
        WorthQueryContinuationExecutionOutcome::Stale(_)
    ));
}

#[test]
fn execution_stops_on_basis_mismatch_before_handle_alignment() {
    let handle = admitted_handle("main");
    let prepared = match handle.prepare_continuation_from_target(
        target_request::<HistoricalFamily>(&handle, "face-a", historical_truth_view_request()),
    ) {
        crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::Prepared(prepared) => {
            prepared
        }
        _ => panic!("expected prepared historical continuation"),
    };

    assert!(matches!(
        drifted_readmission_handle("main", ReadmissionDrift::BasisMismatch)
            .execute_prepared_continuation(prepared),
        WorthQueryContinuationExecutionOutcome::BasisMismatch(_)
    ));
}

#[test]
fn execution_stops_on_authority_mismatch_before_handle_alignment() {
    let handle = admitted_handle("main");
    let prepared = match handle.prepare_continuation_from_target(target_request::<RuntimeFamily>(
        &handle,
        "face-a",
        runtime_route_request(),
    )) {
        crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::Prepared(prepared) => {
            prepared
        }
        _ => panic!("expected prepared preview continuation"),
    };

    assert!(matches!(
        drifted_readmission_handle("main", ReadmissionDrift::AuthorityMismatch)
            .execute_prepared_continuation(prepared),
        WorthQueryContinuationExecutionOutcome::AuthorityMismatch(_)
    ));
}

#[test]
fn execution_stops_on_async_request_drift_before_handle_alignment() {
    let handle = admitted_handle("main");
    let prepared = match handle.prepare_continuation_from_target(target_request::<RuntimeFamily>(
        &handle,
        "face-a",
        runtime_route_request(),
    )) {
        crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::Prepared(prepared) => {
            prepared
        }
        _ => panic!("expected prepared runtime continuation"),
    };

    assert!(matches!(
        drifted_readmission_handle("main", ReadmissionDrift::AsyncRequest)
            .execute_prepared_continuation(prepared),
        WorthQueryContinuationExecutionOutcome::AsyncRequestDrift(_)
    ));
}

#[test]
fn execution_stops_on_replay_drift_before_handle_alignment() {
    let handle = admitted_handle("main");
    let prepared = match handle.prepare_continuation_from_target(
        target_request::<HistoricalFamily>(&handle, "face-a", historical_truth_view_request()),
    ) {
        crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::Prepared(prepared) => {
            prepared
        }
        _ => panic!("expected prepared historical continuation"),
    };

    assert!(matches!(
        drifted_readmission_handle("main", ReadmissionDrift::Replay)
            .execute_prepared_continuation(prepared),
        WorthQueryContinuationExecutionOutcome::ReplayDrift(_)
    ));
}

#[test]
fn execution_stops_on_remask_drift_before_handle_alignment() {
    let handle = admitted_handle("main");
    let prepared = match handle.prepare_continuation_from_target(target_request::<RuntimeFamily>(
        &handle,
        "face-a",
        runtime_route_request(),
    )) {
        crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::Prepared(prepared) => {
            prepared
        }
        _ => panic!("expected prepared runtime continuation"),
    };

    assert!(matches!(
        drifted_readmission_handle("main", ReadmissionDrift::Remask)
            .execute_prepared_continuation(prepared),
        WorthQueryContinuationExecutionOutcome::RemaskDrift(_)
    ));
}

#[test]
fn execution_stops_on_preview_crossed_residue_before_handle_alignment() {
    let handle = admitted_handle("main");
    let prepared = match handle.prepare_continuation_from_target(target_request::<PreviewFamily>(
        &handle,
        "face-a",
        preview_session_request(),
    )) {
        crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::Prepared(prepared) => {
            prepared
        }
        _ => panic!("expected prepared preview continuation"),
    };

    assert!(matches!(
        drifted_readmission_handle("main", ReadmissionDrift::PreviewCrossedResidue)
            .execute_prepared_continuation(prepared),
        WorthQueryContinuationExecutionOutcome::PreviewCrossedResidue(_)
    ));
}

#[test]
fn execution_stops_on_stale_completion_before_handle_alignment() {
    let handle = admitted_handle("main");
    let prepared = match handle.prepare_continuation_from_target(target_request::<RuntimeFamily>(
        &handle,
        "face-a",
        runtime_route_request(),
    )) {
        crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::Prepared(prepared) => {
            prepared
        }
        _ => panic!("expected prepared runtime continuation"),
    };

    assert!(matches!(
        drifted_readmission_handle("main", ReadmissionDrift::StaleCompletion)
            .execute_prepared_continuation(prepared),
        WorthQueryContinuationExecutionOutcome::StaleCompletion(_)
    ));
}

#[test]
fn execution_readmission_preserves_basis_identity_for_equivalent_runtime_meaning() {
    let handle = admitted_handle("main");
    let prepared = match handle.prepare_continuation_from_target(target_request::<RuntimeFamily>(
        &handle,
        "face-a",
        runtime_route_request(),
    )) {
        crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::Prepared(prepared) => {
            prepared
        }
        _ => panic!("expected prepared continuation"),
    };
    assert_eq!(
        prepared
            .execution_readmission()
            .basis_witness()
            .basis_identity_digest(),
        handle
            .operating_context()
            .continuation_execution_readmission_observation(
                prepared.execution_readmission(),
                handle.support_snapshot()
            )
            .basis_identity_digest()
    );
}

#[test]
fn continuation_ordinary_outcome_keeps_new_execution_denials_visible() {
    let handle = admitted_handle("main");
    let prepared = match handle.prepare_continuation_from_target(target_request::<RuntimeFamily>(
        &handle,
        "face-a",
        runtime_route_request(),
    )) {
        crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::Prepared(prepared) => {
            prepared
        }
        _ => panic!("expected prepared runtime continuation"),
    };

    match crate::continuation_pipeline::ordinary_outcome_from_execution_checked(
        drifted_readmission_handle("main", ReadmissionDrift::AuthorityMismatch)
            .execute_prepared_continuation_checked(prepared),
    ) {
        WorthQueryOrdinaryOutcome::AuthorityMismatch(posture) => {
            assert_eq!(
                posture.checked_topology().continuation_kind(),
                Some(WorthQueryOrdinaryContinuationCheckedTopologyKind::AuthorityMismatch)
            );
        }
        _ => panic!("expected authority mismatch ordinary outcome"),
    }
}

#[test]
fn continuation_ordinary_outcome_keeps_execution_drift_topology_visible() {
    let handle = admitted_handle("main");
    let prepared = match handle.prepare_continuation_from_target(target_request::<RuntimeFamily>(
        &handle,
        "face-a",
        runtime_route_request(),
    )) {
        crate::continuation_pipeline::WorthQueryPreparedContinuationOutcome::Prepared(prepared) => {
            prepared
        }
        _ => panic!("expected prepared runtime continuation"),
    };

    match crate::continuation_pipeline::ordinary_outcome_from_execution_checked(
        drifted_readmission_handle("main", ReadmissionDrift::AsyncRequest)
            .execute_prepared_continuation_checked(prepared),
    ) {
        WorthQueryOrdinaryOutcome::RebindRequired(posture) => {
            assert_eq!(
                posture.checked_topology().continuation_kind(),
                Some(WorthQueryOrdinaryContinuationCheckedTopologyKind::AsyncRequestDrift)
            );
        }
        _ => panic!("expected async-request-drift ordinary outcome"),
    }
}

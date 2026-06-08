use super::support::*;
use crate::application::ForgeQueryDomainOperatingContext;
use crate::continuation_pipeline::{
    ForgeQueryContinuationExecutionOutcome, ForgeQueryPreparedContinuationOutcome,
};
use crate::recovery_boundary::{
    forge_query_recovery_brief_from_continuation_execution_checked, ForgeQueryRecoveryAction,
    ForgeQueryRecoveryStopKind,
};

pub(crate) struct RuntimeBackedContinuationClosureSummary {
    pub runtime_basis_identity_digest: String,
    pub observed_basis_identity_digest: String,
    pub replay_recovery_stop_kind: ForgeQueryRecoveryStopKind,
    pub replay_recovery_action: ForgeQueryRecoveryAction,
    pub preview_recovery_stop_kind: ForgeQueryRecoveryStopKind,
    pub preview_recovery_action: ForgeQueryRecoveryAction,
    pub stale_completion_stop_is_typed: bool,
}

pub(crate) fn runtime_backed_continuation_closure_summary(
) -> RuntimeBackedContinuationClosureSummary {
    let runtime_handle = admitted_handle("main");
    let runtime_prepared =
        match runtime_handle.prepare_continuation_from_target(target_request::<RuntimeFamily>(
            &runtime_handle,
            "face-a",
            runtime_route_request(),
        )) {
            ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
            _ => panic!("expected prepared runtime continuation"),
        };
    let observed = runtime_handle
        .operating_context()
        .continuation_execution_readmission_observation(
            runtime_prepared.execution_readmission(),
            runtime_handle.support_snapshot(),
        );

    let historical_handle = admitted_handle("main");
    let historical_prepared = match historical_handle.prepare_continuation_from_target(
        target_request::<HistoricalFamily>(
            &historical_handle,
            "face-a",
            historical_truth_view_request(),
        ),
    ) {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
        _ => panic!("expected prepared historical continuation"),
    };
    let replay_checked = drifted_readmission_handle("main", ReadmissionDrift::Replay)
        .execute_prepared_continuation_checked(historical_prepared);
    let replay_brief =
        forge_query_recovery_brief_from_continuation_execution_checked(replay_checked)
            .expect("replay drift should yield a recovery brief");

    let preview_handle = admitted_handle("main");
    let preview_prepared =
        match preview_handle.prepare_continuation_from_target(target_request::<PreviewFamily>(
            &preview_handle,
            "face-a",
            preview_session_request(),
        )) {
            ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
            _ => panic!("expected prepared preview continuation"),
        };
    let preview_checked =
        drifted_readmission_handle("main", ReadmissionDrift::PreviewCrossedResidue)
            .execute_prepared_continuation_checked(preview_prepared);
    let preview_brief =
        forge_query_recovery_brief_from_continuation_execution_checked(preview_checked)
            .expect("preview-crossed residue should yield a recovery brief");

    let stale_handle = admitted_handle("main");
    let stale_prepared =
        match stale_handle.prepare_continuation_from_target(target_request::<RuntimeFamily>(
            &stale_handle,
            "face-a",
            runtime_route_request(),
        )) {
            ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
            _ => panic!("expected prepared runtime continuation"),
        };
    let stale_completion = drifted_readmission_handle("main", ReadmissionDrift::StaleCompletion)
        .execute_prepared_continuation(stale_prepared);

    RuntimeBackedContinuationClosureSummary {
        runtime_basis_identity_digest: runtime_prepared
            .execution_readmission()
            .basis_witness()
            .basis_identity_digest()
            .to_string(),
        observed_basis_identity_digest: observed.basis_identity_digest().to_string(),
        replay_recovery_stop_kind: replay_brief.stop_kind().clone(),
        replay_recovery_action: replay_brief.recommended_action().clone(),
        preview_recovery_stop_kind: preview_brief.stop_kind().clone(),
        preview_recovery_action: preview_brief.recommended_action().clone(),
        stale_completion_stop_is_typed: matches!(
            stale_completion,
            ForgeQueryContinuationExecutionOutcome::StaleCompletion(_)
        ),
    }
}

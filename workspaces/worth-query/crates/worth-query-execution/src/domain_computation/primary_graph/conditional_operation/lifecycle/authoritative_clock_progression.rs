use worth_runtime_bridge::facade::{BridgeInstalledConditionalLowering, BridgeOwnedSignalRuntime};

use super::{
    ErasedClockObservationOutcome, WorthQueryConditionalClockObservationFailureKind,
    WorthQueryConditionalTruthBasis,
};
use crate::domain_computation::primary_graph::conditional_operation::signal_decision_reentry::WorthQueryRetainedConditionalWake;

pub(super) struct AuthoritativeClockWork<'a, Schema> {
    pub(super) runtime:
        &'a crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
            Schema,
        >,
    pub(super) bridge: &'a mut BridgeOwnedSignalRuntime,
    pub(super) lowering: &'a std::sync::Arc<BridgeInstalledConditionalLowering>,
    pub(super) cursor: &'a mut Option<u64>,
    pub(super) maximum_commits: usize,
    pub(super) watched_records: Vec<worth_relational::facade::transactions::RecordRef>,
    pub(super) include_whole_graph: bool,
    pub(super) retained_wakes: &'a mut [WorthQueryRetainedConditionalWake],
    pub(super) runtime_binding_identity: &'a str,
    pub(super) runtime_capability_identity: u64,
    pub(super) truth: &'a WorthQueryConditionalTruthBasis,
}

#[derive(Clone, Copy)]
pub(super) struct AuthoritativeClockProgress {
    pub(super) commit_count: usize,
    pub(super) work_remaining: bool,
}

pub(super) fn reconsider_authoritative_clock_work<Schema>(
    work: AuthoritativeClockWork<'_, Schema>,
) -> Result<AuthoritativeClockProgress, ErasedClockObservationOutcome> {
    let commits = super::super::authoritative_reconsideration::relevant_authoritative_commits(
        work.runtime,
        *work.cursor,
        work.maximum_commits,
        work.watched_records,
        work.include_whole_graph,
    )
    .map_err(runtime_rejection)?;
    let commit_count = commits.commit_count();
    let work_remaining =
        super::super::authoritative_reconsideration::deliver_authoritative_commits(
            work.bridge,
            work.lowering,
            work.cursor,
            commits,
            work.retained_wakes,
            work.runtime_binding_identity,
            work.runtime_capability_identity,
            work.truth,
        )
        .map_err(runtime_rejection)?;
    Ok(AuthoritativeClockProgress {
        commit_count,
        work_remaining,
    })
}

pub(super) fn runtime_rejection(detail: String) -> ErasedClockObservationOutcome {
    ErasedClockObservationOutcome::Failed {
        kind: WorthQueryConditionalClockObservationFailureKind::RuntimeRejected,
        detail,
    }
}

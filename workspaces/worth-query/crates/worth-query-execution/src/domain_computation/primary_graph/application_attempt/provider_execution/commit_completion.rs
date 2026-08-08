use worth_query_installation::facade::ApplicationSchema;

use super::outcome::WorthQueryProviderProgressionOutcome;
use super::support::unknown_commit_recovery_evidence;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;
use crate::domain_computation::{WorthQueryManagedRunTerminalKind, WorthQueryMutationRunBinding};

pub(super) struct WorthQueryProgressedApplicationCommit {
    pub(super) outcome: WorthQueryProviderProgressionOutcome,
    pub(super) lease: super::super::snapshot_lease::WorthQueryApplicationSnapshotLease,
    pub(super) running: crate::domain_computation::WorthQueryRunningDirectRun,
    pub(super) mutation_run: WorthQueryMutationRunBinding,
}

pub(super) fn finish_application_commit<Schema>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    progressed: WorthQueryProgressedApplicationCommit,
) -> super::super::WorthQueryApplicationCommitOutcome
where
    Schema: ApplicationSchema,
{
    let WorthQueryProgressedApplicationCommit {
        outcome,
        lease,
        running,
        mutation_run,
    } = progressed;
    let terminal = terminal_for(&outcome);
    let snapshot_released = lease.release();
    let completion = match mutation_run.finish(running, terminal, snapshot_released) {
        Ok(completion) => completion,
        Err(()) => {
            return super::super::WorthQueryApplicationCommitOutcome::Indeterminate(
                unknown_commit_recovery_evidence(
                    "managed mutation run failed to finish after provider progression",
                ),
            )
        }
    };
    let committed = outcome.finish(completion).unwrap_or_else(|| {
        super::super::WorthQueryApplicationCommitOutcome::Indeterminate(
            unknown_commit_recovery_evidence(
                "provider progression outcome could not complete commit receipt",
            ),
        )
    });
    application.dispatch_committed_external_effect(committed)
}

const fn terminal_for(
    outcome: &WorthQueryProviderProgressionOutcome,
) -> WorthQueryManagedRunTerminalKind {
    match outcome {
        WorthQueryProviderProgressionOutcome::Committed(_)
        | WorthQueryProviderProgressionOutcome::AlreadyCommitted(_)
        | WorthQueryProviderProgressionOutcome::Stale(_) => {
            WorthQueryManagedRunTerminalKind::Completed
        }
        WorthQueryProviderProgressionOutcome::Cancelled => {
            WorthQueryManagedRunTerminalKind::Cancelled
        }
        WorthQueryProviderProgressionOutcome::Denied(_)
        | WorthQueryProviderProgressionOutcome::Aborted
        | WorthQueryProviderProgressionOutcome::Indeterminate(_) => {
            WorthQueryManagedRunTerminalKind::Failed
        }
    }
}

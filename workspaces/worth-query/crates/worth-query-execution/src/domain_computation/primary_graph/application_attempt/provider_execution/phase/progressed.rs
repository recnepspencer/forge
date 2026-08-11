use worth_query_installation::facade::ApplicationSchema;

use super::super::outcome::WorthQueryProviderProgressionOutcome;
use super::super::support::unknown_commit_recovery_evidence;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;
use crate::domain_computation::WorthQueryManagedRunTerminalKind;

pub(in super::super) struct WorthQueryProgressedApplicationCommit {
    outcome: WorthQueryProviderProgressionOutcome,
    lease: super::super::super::snapshot_lease::WorthQueryApplicationSnapshotLease,
    running: crate::domain_computation::WorthQueryRunningDirectRun,
    cleanup: super::mutation_cleanup::WorthQueryApplicationMutationCleanupOwner,
}

impl WorthQueryProgressedApplicationCommit {
    pub(super) fn new(
        outcome: WorthQueryProviderProgressionOutcome,
        lease: super::super::super::snapshot_lease::WorthQueryApplicationSnapshotLease,
        running: crate::domain_computation::WorthQueryRunningDirectRun,
        cleanup: super::mutation_cleanup::WorthQueryApplicationMutationCleanupOwner,
    ) -> Self {
        Self {
            outcome,
            lease,
            running,
            cleanup,
        }
    }
}

pub(in super::super) fn finish_application_commit<Schema>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    progressed: WorthQueryProgressedApplicationCommit,
) -> super::super::super::WorthQueryApplicationCommitOutcome
where
    Schema: ApplicationSchema,
{
    let WorthQueryProgressedApplicationCommit {
        outcome,
        lease,
        running,
        cleanup,
    } = progressed;
    let terminal = terminal_for(&outcome);
    let snapshot_released = lease.release();
    application
        .primary_provider
        .observe_managed_application_cleanup();
    let completion = match cleanup.finish(running, terminal, snapshot_released) {
        Ok(completion) => completion,
        Err(()) => {
            return super::super::super::WorthQueryApplicationCommitOutcome::Indeterminate(
                unknown_commit_recovery_evidence(
                    "managed mutation run failed to finish after provider progression",
                ),
            )
        }
    };
    let committed = outcome.finish(completion).unwrap_or_else(|| {
        super::super::super::WorthQueryApplicationCommitOutcome::Indeterminate(
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

#[cfg(test)]
mod association_tests;

//! The single authoritative Relational commit transition.

mod publication;
pub(in crate::domain_computation::primary_graph) use publication::WorthQueryPrimaryGraphCommittedApplication;

use super::super::WorthQueryPreparedApplicationCommit;
use crate::domain_computation::primary_graph::provider::{
    mutation_work::WorthQueryPrimaryMutationWorkCounters,
    session_commit::{
        provider_failure, snapshot_admission_failure, WorthQueryPreImageRetentionWork,
    },
    WorthQueryPrimaryGraphApplicationAttempt, WorthQueryPrimaryGraphProvider,
};
use crate::domain_computation::{
    WorthQueryProviderSessionFailure, WorthQueryProviderSessionProtocolStage,
};

pub(super) struct WorthQueryCommittedApplicationSession {
    attempt: WorthQueryPrimaryGraphApplicationAttempt,
    work: WorthQueryPrimaryMutationWorkCounters,
    retained_preimage:
        Option<crate::domain_computation::application_aftermath::WorthQueryRetainedPreImage>,
    preimage_retention_work: WorthQueryPreImageRetentionWork,
    branch: worth_relational::facade::history::BranchId,
    before: worth_relational::facade::snapshots::SnapshotHandle,
    next_basis: worth_relational::facade::branch::AdmittedRelationalBranchBasis,
    committed: worth_relational::facade::transactions::CommitResult,
}

pub(super) struct WorthQueryPerformedApplicationSession {
    committed: WorthQueryCommittedApplicationSession,
    settlement_deferred:
        Option<crate::domain_computation::WorthQueryProviderSessionSettlementDeferred>,
}

pub(super) fn commit(
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
    prepared: WorthQueryPreparedApplicationCommit,
    mint: super::WorthQueryCommitProgressionMint,
) -> Result<
    WorthQueryPerformedApplicationSession,
    crate::domain_computation::WorthQueryProviderSessionCommitStop,
> {
    let WorthQueryPreparedApplicationCommit {
        attempt,
        candidate,
        work,
        branch,
        retained_preimage,
        preimage_retention_work,
    } = prepared;
    let _ = mint;
    let before =
        crate::domain_computation::primary_graph::exact_basis_access::open_current_branch_snapshot(
            runtime, &branch,
        )
        .map_err(|denial| {
            snapshot_admission_failure(
                WorthQueryProviderSessionProtocolStage::Commit,
                denial,
                "application branch has no current pre-commit snapshot",
            )
        })
        .map_err(crate::domain_computation::WorthQueryProviderSessionCommitStop::from)?;
    let candidate = match runtime.prepare_validated_proposal(candidate) {
        Ok(candidate) => candidate,
        Err(error) => return Err(transaction_commit_stop(runtime, &before, error)),
    };
    let performed = match runtime.publication_port().compare_and_publish(candidate) {
        worth_relational::facade::mvcc::RelationalPublicationOutcome::Performed(performed) => {
            performed
        }
        worth_relational::facade::mvcc::RelationalPublicationOutcome::Stale(_) => {
            return Err(
                crate::domain_computation::WorthQueryProviderSessionCommitStop::Denied(
                    reject_before_movement(
                        runtime,
                        &before,
                        "Relational application publication lost a same-branch race",
                    ),
                ),
            );
        }
        worth_relational::facade::mvcc::RelationalPublicationOutcome::Denied(_) => {
            return Err(
                crate::domain_computation::WorthQueryProviderSessionCommitStop::Denied(
                    reject_before_movement(
                        runtime,
                        &before,
                        "Relational owner denied application publication",
                    ),
                ),
            );
        }
        worth_relational::facade::mvcc::RelationalPublicationOutcome::Interrupted(event) => {
            crate::relational_snapshot_release::release_query_snapshot(runtime, &before);
            return Err(
                crate::domain_computation::WorthQueryProviderSessionCommitStop::ControlStopped(
                    interruption_control_stopped(event),
                ),
            );
        }
        worth_relational::facade::mvcc::RelationalPublicationOutcome::Deferred(deferred) => {
            crate::relational_snapshot_release::release_query_snapshot(runtime, &before);
            return Err(
                crate::domain_computation::WorthQueryProviderSessionCommitStop::Deferred(
                    publication_deferred(deferred),
                ),
            );
        }
        worth_relational::facade::mvcc::RelationalPublicationOutcome::Failed(_) => {
            return Err(
                crate::domain_computation::WorthQueryProviderSessionCommitStop::Denied(
                    reject_before_movement(
                        runtime,
                        &before,
                        "Relational application publication failed before movement",
                    ),
                ),
            );
        }
    };
    let next_basis = performed.next_basis().clone();
    let (committed, settlement_deferred) = match runtime.settle_performed_publication(performed) {
        Ok(committed) => (committed, None),
        Err(error) => {
            let Some(settlement) = error.deferred_settlement().cloned() else {
                crate::relational_snapshot_release::release_query_snapshot(runtime, &before);
                return Err(
                    crate::domain_computation::WorthQueryProviderSessionCommitStop::Denied(
                        recovery_failure(
                            "Relational application movement requires durability recovery",
                        ),
                    ),
                );
            };
            (
                settlement.performed_result().clone(),
                Some(
                    crate::domain_computation::WorthQueryProviderSessionSettlementDeferred::new(
                        "Relational application movement requires durability settlement repair",
                        settlement,
                    ),
                ),
            )
        }
    };
    Ok(WorthQueryPerformedApplicationSession {
        committed: WorthQueryCommittedApplicationSession {
            attempt,
            work,
            retained_preimage,
            preimage_retention_work,
            branch,
            before,
            next_basis,
            committed,
        },
        settlement_deferred,
    })
}

impl WorthQueryPerformedApplicationSession {
    pub(super) const fn committed(&self) -> &WorthQueryCommittedApplicationSession {
        &self.committed
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        WorthQueryCommittedApplicationSession,
        Option<crate::domain_computation::WorthQueryProviderSessionSettlementDeferred>,
    ) {
        (self.committed, self.settlement_deferred)
    }
}

impl WorthQueryCommittedApplicationSession {
    pub(super) const fn attempt(&self) -> &WorthQueryPrimaryGraphApplicationAttempt {
        &self.attempt
    }

    pub(super) const fn work(&self) -> WorthQueryPrimaryMutationWorkCounters {
        self.work
    }

    pub(super) const fn retained_preimage(
        &self,
    ) -> Option<&crate::domain_computation::application_aftermath::WorthQueryRetainedPreImage> {
        self.retained_preimage.as_ref()
    }

    pub(super) const fn preimage_retention_work(&self) -> WorthQueryPreImageRetentionWork {
        self.preimage_retention_work
    }

    pub(super) const fn committed(&self) -> &worth_relational::facade::transactions::CommitResult {
        &self.committed
    }

    pub(super) fn publish_and_encode(
        self,
        provider: &WorthQueryPrimaryGraphProvider,
        runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
        evidence: super::WorthQueryPrimaryGraphCommitEvidence,
    ) -> Result<
        crate::domain_computation::WorthQueryProviderTerminalDescription,
        WorthQueryProviderSessionFailure,
    > {
        let published = publication::publish(provider, runtime, self, evidence)?;
        publication::encode(provider, published)
    }
}

fn failure(detail: &'static str) -> WorthQueryProviderSessionFailure {
    provider_failure(WorthQueryProviderSessionProtocolStage::Commit, detail)
}

fn reject_before_movement(
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
    before: &worth_relational::facade::snapshots::SnapshotHandle,
    detail: &'static str,
) -> WorthQueryProviderSessionFailure {
    crate::relational_snapshot_release::release_query_snapshot(runtime, before);
    failure(detail)
}

fn recovery_failure(detail: &'static str) -> WorthQueryProviderSessionFailure {
    failure(detail).with_recovery_posture(
        crate::domain_computation::WorthQueryProviderSessionRecoveryPosture::RecoveryRequired,
    )
}

fn transaction_commit_stop(
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
    before: &worth_relational::facade::snapshots::SnapshotHandle,
    error: worth_relational::facade::mvcc::TransactionCommitError,
) -> crate::domain_computation::WorthQueryProviderSessionCommitStop {
    use worth_relational::facade::mvcc::TransactionCommitError as Error;
    crate::relational_snapshot_release::release_query_snapshot(runtime, before);
    match error {
        Error::Interrupted { interruption, .. } => {
            crate::domain_computation::WorthQueryProviderSessionCommitStop::ControlStopped(
                interruption_control_stopped(interruption),
            )
        }
        Error::PublicationDeferred { deferred, .. } => {
            crate::domain_computation::WorthQueryProviderSessionCommitStop::Deferred(
                publication_deferred(deferred),
            )
        }
        Error::PublicationFailed { failure, .. } => {
            crate::domain_computation::WorthQueryProviderSessionCommitStop::Denied(
                publication_failure(failure),
            )
        }
        Error::PerformedButDurabilityDeferred {
            settlement, error, ..
        } => crate::domain_computation::WorthQueryProviderSessionCommitStop::SettlementDeferred(
            crate::domain_computation::WorthQueryProviderSessionSettlementDeferred::new(
                error.detail,
                settlement,
            ),
        ),
        _ => crate::domain_computation::WorthQueryProviderSessionCommitStop::Denied(failure(
            "Relational rejected application commit preparation",
        )),
    }
}

fn interruption_control_stopped(
    event: worth_relational::facade::mvcc::RelationalInterruptionEvent,
) -> crate::domain_computation::WorthQueryProviderSessionCommitControlStopped {
    use crate::domain_computation::WorthQueryProviderSessionControlStopKind as Kind;
    let kind = match event.interruption() {
        worth_relational::facade::mvcc::RelationalOperationInterruption::Cancelled => {
            Kind::Cancelled
        }
        worth_relational::facade::mvcc::RelationalOperationInterruption::TimedOut => Kind::TimedOut,
    };
    crate::domain_computation::WorthQueryProviderSessionCommitControlStopped::new(
        kind,
        format!("{event:?}"),
    )
}

fn publication_deferred(
    deferred: worth_relational::facade::mvcc::RelationalPublicationDeferred,
) -> crate::domain_computation::WorthQueryProviderSessionCommitDeferred {
    use crate::domain_computation::WorthQueryProviderSessionCommitDeferredKind as Kind;
    use worth_relational::facade::mvcc::RelationalPublicationDeferred as Deferred;
    let kind = match deferred {
        Deferred::PatchPositionReservationContended => Kind::PatchPositionReservationContended,
        Deferred::RetentionBackpressure => Kind::RetentionCapacityExhausted,
        Deferred::CandidateLifetimeExpired {
            maximum_lifetime_millis,
        } => Kind::CandidateLifetimeExpired {
            maximum_lifetime_millis,
        },
        Deferred::CandidateCapacityExhausted { maximum_candidates } => {
            Kind::CandidateCapacityExhausted { maximum_candidates }
        }
        Deferred::PublishedSnapshotCapacityExhausted { maximum_handles } => {
            Kind::PublishedSnapshotCapacityExhausted { maximum_handles }
        }
    };
    crate::domain_computation::WorthQueryProviderSessionCommitDeferred::new(
        kind,
        format!("{deferred:?}"),
    )
}

fn publication_failure(
    failure: worth_relational::facade::mvcc::RelationalPublicationFailure,
) -> WorthQueryProviderSessionFailure {
    use worth_relational::facade::mvcc::RelationalPublicationFailureKind as Failure;
    let kind = match failure.kind() {
        Failure::SnapshotIdentityExhausted => {
            crate::domain_computation::WorthQueryProviderSessionDenialKind::SnapshotIdentityExhausted
        }
        Failure::CandidateIdentityExhausted => {
            crate::domain_computation::WorthQueryProviderSessionDenialKind::CandidateIdentityExhausted
        }
        Failure::RetentionIdentityExhausted => {
            crate::domain_computation::WorthQueryProviderSessionDenialKind::RetentionIdentityExhausted
        }
        Failure::PreparedRootBudgetExhausted {
            maximum_bytes,
            required_bytes,
        } => crate::domain_computation::WorthQueryProviderSessionDenialKind::PreparedRootBudgetExhausted {
            maximum_bytes: *maximum_bytes,
            required_bytes: *required_bytes,
        },
        _ => crate::domain_computation::WorthQueryProviderSessionDenialKind::ProviderRejected,
    };
    WorthQueryProviderSessionFailure::new(
        kind,
        WorthQueryProviderSessionProtocolStage::Commit,
        failure.detail(),
        crate::domain_computation::WorthQueryProviderSessionProtocolCounters::default(),
    )
}

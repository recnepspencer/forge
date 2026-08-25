//! The single authoritative Relational commit transition.

mod publication;
pub(in crate::domain_computation::primary_graph) use publication::WorthQueryPrimaryGraphCommittedApplication;

use super::super::WorthQueryPreparedApplicationCommit;
use crate::domain_computation::primary_graph::provider::{
    mutation_work::WorthQueryPrimaryMutationWorkCounters,
    session_commit::{provider_failure, WorthQueryPreImageRetentionWork},
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
        .ok_or_else(|| failure("application branch has no current pre-commit snapshot"))
        .map_err(crate::domain_computation::WorthQueryProviderSessionCommitStop::from)?;
    let candidate = runtime
        .prepare_validated_proposal(candidate)
        .map_err(|_| {
            reject_before_movement(
                runtime,
                &before,
                "Relational rejected application commit preparation",
            )
        })
        .map_err(crate::domain_computation::WorthQueryProviderSessionCommitStop::from)?;
    let performed = match runtime.publication_port().compare_and_publish(candidate) {
        worth_proof::TransitionOutcome::Success(performed) => performed,
        worth_proof::TransitionOutcome::Stale(_) => {
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
        worth_proof::TransitionOutcome::Denied(_) => {
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
        worth_proof::TransitionOutcome::Deferred(_) => {
            let _ = runtime.snapshots().release_snapshot(&before);
            return Err(
                crate::domain_computation::WorthQueryProviderSessionCommitStop::Deferred(
                    crate::domain_computation::WorthQueryProviderSessionCommitDeferred::new(
                        "Relational owner deferred application publication before movement",
                    ),
                ),
            );
        }
        worth_proof::TransitionOutcome::Failed(_) => {
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
        worth_proof::TransitionOutcome::RebindRequired(impossible) => match impossible {},
    };
    let (committed, settlement_deferred) = match runtime.settle_performed_publication(performed) {
        Ok(committed) => (committed, None),
        Err(error) => {
            let Some(settlement) = error.deferred_settlement().cloned() else {
                let _ = runtime.snapshots().release_snapshot(&before);
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
    let _ = runtime.snapshots().release_snapshot(before);
    failure(detail)
}

fn recovery_failure(detail: &'static str) -> WorthQueryProviderSessionFailure {
    failure(detail).with_recovery_posture(
        crate::domain_computation::WorthQueryProviderSessionRecoveryPosture::RecoveryRequired,
    )
}

use worth_proof::TransitionOutcome;

use worth_foundational::FoundationalBranchReferenceGeneration;

use crate::branch::{admit_runtime_signal_branch_observation, AdmittedSignalBranchBasis};
use crate::data::error::SignalError;
use crate::state::{SignalBranchHandle, SignalSnapshotV1, SnapshotRestoreIntent};

use super::super::runtime_state::SignalRuntime;
use super::basis::{
    materialize_branch_basis, SignalBranchBasisArtifact, SignalBranchBasisDenial,
    SignalBranchBasisIdentity, SignalBranchBasisValidationOutcome,
};
use super::basis_definition::signal_definition_basis;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn current_branch_basis_artifact(&mut self) -> SignalBranchBasisArtifact {
        self.with_telemetry(|telemetry| telemetry.transaction.branch_basis_production_count += 1);
        let branch = self.graph.current_branch();
        materialize_branch_basis(
            branch.name.clone(),
            SignalBranchBasisIdentity::from_branch_handle(&branch),
        )
    }

    pub fn branch_basis_artifact(
        &mut self,
        branch: SignalBranchHandle,
    ) -> TransitionOutcome<SignalBranchBasisArtifact, SignalBranchBasisDenial> {
        self.with_telemetry(|telemetry| telemetry.transaction.branch_basis_production_count += 1);
        let live_branch = self
            .graph
            .branch_handle(branch.id)
            .or_else(|| self.branches.branch_handle(branch.id))
            .ok_or(SignalBranchBasisDenial::UnknownBranch {
                branch_id: branch.id,
                branch_name: branch.name,
            });
        match live_branch {
            Ok(branch) => TransitionOutcome::success(materialize_branch_basis(
                branch.name.clone(),
                SignalBranchBasisIdentity::from_branch_handle(&branch),
            )),
            Err(denial) => {
                self.with_telemetry(|telemetry| {
                    telemetry.transaction.branch_basis_denial_count += 1
                });
                TransitionOutcome::denied(denial)
            }
        }
    }

    pub fn snapshot_restore_branch_basis_artifact(
        &mut self,
        snapshot: &SignalSnapshotV1,
        intent: SnapshotRestoreIntent,
    ) -> TransitionOutcome<SignalBranchBasisArtifact, SignalBranchBasisDenial> {
        self.with_telemetry(|telemetry| telemetry.transaction.branch_basis_production_count += 1);
        let branch_id = snapshot.meta.branch_id;
        let snapshot_id = snapshot.meta.snapshot_id;
        let Some(live_branch) = self
            .graph
            .branch_handle(branch_id)
            .or_else(|| self.branches.branch_handle(branch_id))
        else {
            self.with_telemetry(|telemetry| telemetry.transaction.branch_basis_denial_count += 1);
            return TransitionOutcome::denied(SignalBranchBasisDenial::UnknownBranch {
                branch_id,
                branch_name: snapshot.meta.branch_name.clone(),
            });
        };

        if self
            .branches
            .snapshot_state(branch_id, snapshot_id)
            .is_none()
        {
            self.with_telemetry(|telemetry| telemetry.transaction.branch_basis_denial_count += 1);
            return TransitionOutcome::denied(SignalBranchBasisDenial::UntrackedSnapshot {
                branch_id,
                snapshot_id,
            });
        }

        TransitionOutcome::success(materialize_branch_basis(
            live_branch.name.clone(),
            SignalBranchBasisIdentity::from_snapshot_restore(snapshot, intent),
        ))
    }

    pub fn snapshot_branch_basis_artifact(
        &mut self,
        branch: SignalBranchHandle,
        snapshot: &SignalSnapshotV1,
    ) -> TransitionOutcome<SignalBranchBasisArtifact, SignalBranchBasisDenial> {
        self.with_telemetry(|telemetry| telemetry.transaction.branch_basis_production_count += 1);
        let Some(live_branch) = self
            .graph
            .branch_handle(branch.id)
            .or_else(|| self.branches.branch_handle(branch.id))
        else {
            self.with_telemetry(|telemetry| telemetry.transaction.branch_basis_denial_count += 1);
            return TransitionOutcome::denied(SignalBranchBasisDenial::UnknownBranch {
                branch_id: branch.id,
                branch_name: branch.name,
            });
        };

        if snapshot.meta.branch_id != live_branch.id {
            self.with_telemetry(|telemetry| telemetry.transaction.branch_basis_denial_count += 1);
            return TransitionOutcome::denied(SignalBranchBasisDenial::CrossBranchMismatch {
                basis_branch_id: snapshot.meta.branch_id,
                expected_branch_id: live_branch.id,
            });
        }

        if self
            .branches
            .snapshot_state(live_branch.id, snapshot.meta.snapshot_id)
            .is_none()
        {
            self.with_telemetry(|telemetry| telemetry.transaction.branch_basis_denial_count += 1);
            return TransitionOutcome::denied(SignalBranchBasisDenial::UntrackedSnapshot {
                branch_id: live_branch.id,
                snapshot_id: snapshot.meta.snapshot_id,
            });
        }

        TransitionOutcome::success(materialize_branch_basis(
            live_branch.name.clone(),
            SignalBranchBasisIdentity::from_branch_snapshot(
                &live_branch,
                snapshot.meta.snapshot_id,
            ),
        ))
    }

    pub fn validate_branch_basis_artifact(
        &mut self,
        basis: SignalBranchBasisArtifact,
        branch: SignalBranchHandle,
    ) -> SignalBranchBasisValidationOutcome {
        self.with_telemetry(|telemetry| telemetry.transaction.branch_basis_validation_count += 1);
        if basis.payload().branch_id() != branch.id {
            self.with_telemetry(|telemetry| telemetry.transaction.branch_basis_denial_count += 1);
            return TransitionOutcome::denied(SignalBranchBasisDenial::CrossBranchMismatch {
                basis_branch_id: basis.payload().branch_id(),
                expected_branch_id: branch.id,
            });
        }

        let Some(live_branch) = self
            .graph
            .branch_handle(branch.id)
            .or_else(|| self.branches.branch_handle(branch.id))
        else {
            self.with_telemetry(|telemetry| telemetry.transaction.branch_basis_denial_count += 1);
            return TransitionOutcome::denied(SignalBranchBasisDenial::UnknownBranch {
                branch_id: branch.id,
                branch_name: branch.name,
            });
        };

        let live_identity = SignalBranchBasisIdentity::from_branch_handle(&live_branch);
        if basis.strong_basis().value() != &live_identity {
            self.with_telemetry(|telemetry| telemetry.transaction.branch_basis_stale_count += 1);
            return TransitionOutcome::stale(basis.downgrade_to_stale_readable());
        }

        TransitionOutcome::success(basis)
    }

    /// Observe one live Signal branch through the shared Foundational
    /// reference grammar and immediately attach the owner-issued basis proof.
    /// The old numeric head remains an internal engine detail; callers receive
    /// only the exact immutable observation that can be compared or carried.
    pub fn observe_signal_branch_basis(
        &self,
        branch: SignalBranchHandle,
    ) -> Result<AdmittedSignalBranchBasis, SignalError> {
        let live_branch = self
            .graph
            .branch_handle(branch.id)
            .or_else(|| self.branches.branch_handle(branch.id))
            .ok_or_else(|| SignalError::unknown_branch(Some(branch.id), branch.name.clone()))?;
        let identity = SignalBranchBasisIdentity::from_branch_handle(&live_branch);
        let observation = identity
            .to_foundational_observation(
                self.graph.runtime_instance_id().to_string(),
                live_branch.name.clone(),
                signal_definition_basis(self),
                FoundationalBranchReferenceGeneration::new(
                    self.branches.branch_head_generation(live_branch.id),
                ),
            )
            .map_err(|denial| {
                SignalError::invalid_input(format!(
                    "Signal branch observation construction denied: {denial:?}"
                ))
            })?;
        Ok(admit_runtime_signal_branch_observation(
            observation,
            live_branch.id,
        ))
    }

    /// Fork from an exact, owner-issued canonical basis. The basis is
    /// borrowed so immutable Signal observations remain cheaply shareable;
    /// the runtime still rechecks the live observation before using its
    /// private fork engine.
    pub fn fork_signal_branch(
        &mut self,
        name: impl Into<String>,
        source: &AdmittedSignalBranchBasis,
    ) -> Result<SignalBranchHandle, SignalError> {
        let branch_id = source.owner_branch_id().ok_or_else(|| {
            SignalError::invalid_input(
                "Signal fork requires a runtime-issued branch basis, not a structural observation",
            )
        })?;
        let branch = self
            .branch_handle(branch_id)
            .ok_or_else(|| SignalError::unknown_branch(Some(branch_id), "fork-source"))?;
        let live = self.observe_signal_branch_basis(branch.clone())?;
        live.observation()
            .compare(source.observation())
            .map_err(|mismatch| {
                SignalError::invalid_input(format!(
                    "Signal fork source basis is stale or foreign: {mismatch:?}"
                ))
            })?;
        let request = super::SignalBranchForkRequest::from_parent_branch_head(name, branch_id);
        match self.fork_branch(request) {
            TransitionOutcome::Success(receipt) => Ok(receipt.created_branch().clone()),
            TransitionOutcome::Denied(denial) => Err(Self::fork_denial_to_signal_error(denial)),
            other => Err(SignalError::internal(format!(
                "unexpected non-terminal Signal fork outcome: {other:?}"
            ))),
        }
    }

    /// Canonical-basis adapter for targeted transactions. The returned plan
    /// is opaque to callers; only the canonical observation crosses the
    /// facade while the legacy head tuple remains private engine state.
    pub fn plan_signal_branch_targeted_transaction(
        &mut self,
        branch: SignalBranchHandle,
        expected: &AdmittedSignalBranchBasis,
    ) -> TransitionOutcome<
        super::LoweredBranchTargetedTransactionPlan,
        super::BranchTargetedTransactionDenial,
    > {
        let Ok(live) = self.observe_signal_branch_basis(branch.clone()) else {
            return TransitionOutcome::denied(
                super::BranchTargetedTransactionDenial::UnknownTargetBranch {
                    branch_id: branch.id,
                },
            );
        };
        if live.observation().compare(expected.observation()).is_err() {
            return TransitionOutcome::denied(
                super::BranchTargetedTransactionDenial::CanonicalBasisMismatch,
            );
        }
        let head = match self.branch_transaction_head(branch.clone()) {
            TransitionOutcome::Success(head) => head,
            TransitionOutcome::Denied(denial) => return TransitionOutcome::denied(denial),
            _ => {
                return TransitionOutcome::denied(
                    super::BranchTargetedTransactionDenial::UnknownTargetBranch {
                        branch_id: branch.id,
                    },
                )
            }
        };
        self.plan_branch_targeted_transaction(super::BranchTargetedTransactionRequest::new(
            branch, head,
        ))
    }

    /// Canonical-basis adapter for one branch retirement plan.
    pub fn plan_signal_branch_retirement(
        &mut self,
        branch: SignalBranchHandle,
        expected: &AdmittedSignalBranchBasis,
        reason: super::SignalBranchRetirementReason,
    ) -> TransitionOutcome<super::PlannedSignalBranchRetirement, super::SignalBranchRetirementDenial>
    {
        let Ok(live) = self.observe_signal_branch_basis(branch.clone()) else {
            return TransitionOutcome::denied(super::SignalBranchRetirementDenial::UnknownBranch {
                branch_id: branch.id,
            });
        };
        if live.observation().compare(expected.observation()).is_err() {
            return TransitionOutcome::denied(
                super::SignalBranchRetirementDenial::CanonicalBasisMismatch,
            );
        }
        let head = match self.branch_transaction_head(branch.clone()) {
            TransitionOutcome::Success(head) => head,
            TransitionOutcome::Denied(denial) => {
                return TransitionOutcome::denied(match denial {
                    super::BranchTargetedTransactionDenial::UnknownTargetBranch { branch_id } => {
                        super::SignalBranchRetirementDenial::UnknownBranch { branch_id }
                    }
                    super::BranchTargetedTransactionDenial::StaleTargetHead {
                        expected,
                        observed,
                    } => {
                        super::SignalBranchRetirementDenial::StaleBranchHead { expected, observed }
                    }
                    _ => super::SignalBranchRetirementDenial::UnknownBranch {
                        branch_id: branch.id,
                    },
                })
            }
            _ => {
                return TransitionOutcome::denied(
                    super::SignalBranchRetirementDenial::UnknownBranch {
                        branch_id: branch.id,
                    },
                )
            }
        };
        self.plan_branch_retirement(super::SignalBranchRetirementRequest::new(
            branch, head, reason,
        ))
    }

    /// Canonical-basis adapter for an atomic retirement batch. Each basis is
    /// checked before any legacy request is assembled, so a stale or foreign
    /// observation cannot be smuggled through the batch lane.
    pub fn plan_signal_branch_retirement_batch(
        &mut self,
        requests: Vec<(
            SignalBranchHandle,
            AdmittedSignalBranchBasis,
            super::SignalBranchRetirementReason,
        )>,
    ) -> TransitionOutcome<
        super::PlannedSignalBranchRetirementBatch,
        super::SignalBranchRetirementBatchDenial,
    > {
        let mut native_requests = Vec::with_capacity(requests.len());
        for (position, (branch, basis, reason)) in requests.into_iter().enumerate() {
            let Ok(live) = self.observe_signal_branch_basis(branch.clone()) else {
                return TransitionOutcome::denied(
                    super::SignalBranchRetirementBatchDenial::Retirement {
                        position: position as u32,
                        denial: super::SignalBranchRetirementDenial::UnknownBranch {
                            branch_id: branch.id,
                        },
                    },
                );
            };
            if live.observation().compare(basis.observation()).is_err() {
                return TransitionOutcome::denied(
                    super::SignalBranchRetirementBatchDenial::Retirement {
                        position: position as u32,
                        denial: super::SignalBranchRetirementDenial::CanonicalBasisMismatch,
                    },
                );
            }
            let head = match self.branch_transaction_head(branch.clone()) {
                TransitionOutcome::Success(head) => head,
                TransitionOutcome::Denied(_) | TransitionOutcome::Failed(_) => {
                    return TransitionOutcome::denied(
                        super::SignalBranchRetirementBatchDenial::Retirement {
                            position: position as u32,
                            denial: super::SignalBranchRetirementDenial::UnknownBranch {
                                branch_id: branch.id,
                            },
                        },
                    )
                }
            };
            native_requests.push(super::SignalBranchRetirementRequest::new(
                branch, head, reason,
            ));
        }
        self.plan_branch_retirement_batch(super::SignalBranchRetirementBatchRequest::new(
            native_requests,
        ))
    }
}

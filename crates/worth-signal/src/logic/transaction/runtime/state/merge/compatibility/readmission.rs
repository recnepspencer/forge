use worth_proof::TransitionOutcome;

use crate::diagnostics::replay::{ReplayEvent, ReplayEventDetail};
use crate::logic::transaction::runtime::state::merge::{
    BranchMergeExecutionSummary, BranchMergeResult, LoweredMergePlan, ScopedMergeProofPacket,
    SignalMergeStrategyWitness,
};
use crate::logic::transaction::runtime::state::{
    SignalBranchBasisArtifact, SignalBranchBasisIdentity, SignalBranchBasisValidationOutcome,
    StaleSignalBranchBasisArtifact,
};
use crate::state::SignalBranchHandle;

use super::denial::SignalMergeCompatibilityDenial;
use super::facts::SignalMergeCompatibilityFactInventory;
use super::witness::{
    new_signal_merge_compatibility_artifact, BoundaryBridgedSignalMergeCompatibilityArtifact,
    SignalMergeCompatibilityArtifact, SignalMergeCompatibilityBasis,
    SignalMergeCompatibilityReadmissionAuthority, SignalMergeCompatibilityWitness,
};
use crate::logic::transaction::runtime::state::runtime_state::SignalRuntime;

pub type SignalMergeCompatibilityOutcome =
    TransitionOutcome<SignalMergeCompatibilityArtifact, SignalMergeCompatibilityDenial>;

fn build_compatibility_basis(
    branch_basis_identity: &SignalBranchBasisIdentity,
    facts: &SignalMergeCompatibilityFactInventory,
) -> SignalMergeCompatibilityBasis {
    SignalMergeCompatibilityBasis::new(
        branch_basis_identity.clone(),
        facts.declaration_digest().to_owned(),
        facts.admitted_scope_digest().to_owned(),
        facts.strategy_witness_digest().to_owned(),
    )
}

fn branch_basis_validation_to_denial(
    outcome: SignalBranchBasisValidationOutcome,
    branch: &SignalBranchHandle,
) -> Result<SignalBranchBasisArtifact, SignalMergeCompatibilityDenial> {
    match outcome {
        TransitionOutcome::Success(artifact) => Ok(artifact),
        TransitionOutcome::Denied(denial) => {
            Err(SignalMergeCompatibilityDenial::BranchBasisDenied(denial))
        }
        TransitionOutcome::Stale(stale) => Err(SignalMergeCompatibilityDenial::StaleBranchBasis {
            branch_id: branch.id,
            basis_digest: {
                let stale: StaleSignalBranchBasisArtifact = stale;
                stale.payload().basis_digest().to_owned()
            },
        }),
        TransitionOutcome::Deferred(impossible) => match impossible {},
        TransitionOutcome::RebindRequired(impossible) => match impossible {},
        TransitionOutcome::Failed(impossible) => match impossible {},
    }
}

fn compare_retained_inputs(
    expected_facts: &SignalMergeCompatibilityFactInventory,
    scoped_merge_proof: &ScopedMergeProofPacket,
    strategy_witness: &SignalMergeStrategyWitness,
) -> Result<(), SignalMergeCompatibilityDenial> {
    if expected_facts.declaration_digest() != scoped_merge_proof.declaration_digest() {
        return Err(SignalMergeCompatibilityDenial::ScopedMergeProofMismatch {
            expected_declaration_digest: expected_facts.declaration_digest().to_owned(),
            observed_declaration_digest: scoped_merge_proof.declaration_digest().to_owned(),
        });
    }
    if expected_facts.strategy_witness_digest() != strategy_witness.witness_digest() {
        return Err(SignalMergeCompatibilityDenial::StrategyWitnessMismatch {
            expected_witness_digest: expected_facts.strategy_witness_digest().to_owned(),
            observed_witness_digest: strategy_witness.witness_digest().to_owned(),
        });
    }
    Ok(())
}

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn merge_compatibility_artifact_from_parts(
        &mut self,
        branch_basis: SignalBranchBasisArtifact,
        branch: SignalBranchHandle,
        scoped_merge_proof: Option<ScopedMergeProofPacket>,
        strategy_witness: Option<SignalMergeStrategyWitness>,
    ) -> SignalMergeCompatibilityOutcome {
        self.with_telemetry(|telemetry| telemetry.transaction.merge_compatibility_build_count += 1);
        let validated_basis = match branch_basis_validation_to_denial(
            self.validate_branch_basis_artifact(branch_basis, branch.clone()),
            &branch,
        ) {
            Ok(artifact) => artifact,
            Err(denial) => {
                self.with_telemetry(|telemetry| {
                    telemetry.transaction.merge_compatibility_denial_count += 1
                });
                return TransitionOutcome::denied(denial);
            }
        };

        let Some(scoped_merge_proof) = scoped_merge_proof else {
            self.with_telemetry(|telemetry| {
                telemetry.transaction.merge_compatibility_denial_count += 1
            });
            return TransitionOutcome::denied(
                SignalMergeCompatibilityDenial::MissingScopedMergeProof {
                    branch_id: branch.id,
                },
            );
        };
        let Some(strategy_witness) = strategy_witness else {
            self.with_telemetry(|telemetry| {
                telemetry.transaction.merge_compatibility_denial_count += 1
            });
            return TransitionOutcome::denied(
                SignalMergeCompatibilityDenial::MissingStrategyWitness {
                    branch_id: branch.id,
                },
            );
        };

        let facts = SignalMergeCompatibilityFactInventory::from_retained(
            validated_basis.payload(),
            &scoped_merge_proof,
            &strategy_witness,
        );
        let payload = SignalMergeCompatibilityWitness::new(facts.clone());
        let basis = build_compatibility_basis(validated_basis.strong_basis().value(), &facts);
        TransitionOutcome::success(new_signal_merge_compatibility_artifact(payload, basis))
    }

    pub fn planned_merge_compatibility_artifact(
        &mut self,
        branch_basis: SignalBranchBasisArtifact,
        branch: SignalBranchHandle,
        plan: &LoweredMergePlan,
    ) -> SignalMergeCompatibilityOutcome {
        if branch.id != plan.target_branch_id() {
            self.with_telemetry(|telemetry| {
                telemetry.transaction.merge_compatibility_denial_count += 1
            });
            return TransitionOutcome::denied(SignalMergeCompatibilityDenial::CrossBasisMismatch {
                expected_branch_id: plan.target_branch_id(),
                observed_branch_id: branch.id,
            });
        }
        self.merge_compatibility_artifact_from_parts(
            branch_basis,
            branch,
            Some(plan.scoped_merge_proof().clone()),
            Some(plan.strategy_witness().clone()),
        )
    }

    pub fn merge_result_compatibility_artifact(
        &mut self,
        branch_basis: SignalBranchBasisArtifact,
        branch: SignalBranchHandle,
        result: &BranchMergeResult,
    ) -> SignalMergeCompatibilityOutcome {
        if branch.id != result.target_branch {
            self.with_telemetry(|telemetry| {
                telemetry.transaction.merge_compatibility_denial_count += 1
            });
            return TransitionOutcome::denied(SignalMergeCompatibilityDenial::CrossBasisMismatch {
                expected_branch_id: result.target_branch,
                observed_branch_id: branch.id,
            });
        }
        self.merge_compatibility_artifact_from_parts(
            branch_basis,
            branch,
            Some(result.scoped_merge_proof.clone()),
            Some(result.strategy_witness.clone()),
        )
    }

    pub fn merge_execution_summary_compatibility_artifact(
        &mut self,
        branch_basis: SignalBranchBasisArtifact,
        branch: SignalBranchHandle,
        summary: &BranchMergeExecutionSummary,
    ) -> SignalMergeCompatibilityOutcome {
        if branch.id != summary.target_branch_id {
            self.with_telemetry(|telemetry| {
                telemetry.transaction.merge_compatibility_denial_count += 1
            });
            return TransitionOutcome::denied(SignalMergeCompatibilityDenial::CrossBasisMismatch {
                expected_branch_id: summary.target_branch_id,
                observed_branch_id: branch.id,
            });
        }
        self.merge_compatibility_artifact_from_parts(
            branch_basis,
            branch,
            Some(summary.scoped_merge_proof.clone()),
            Some(summary.strategy_witness.clone()),
        )
    }

    pub fn replay_merge_compatibility_artifact(
        &mut self,
        branch_basis: SignalBranchBasisArtifact,
        branch: SignalBranchHandle,
        replay_event: &ReplayEvent,
    ) -> SignalMergeCompatibilityOutcome {
        if branch.id != replay_event.branch_id {
            self.with_telemetry(|telemetry| {
                telemetry.transaction.merge_compatibility_denial_count += 1
            });
            return TransitionOutcome::denied(SignalMergeCompatibilityDenial::CrossBasisMismatch {
                expected_branch_id: replay_event.branch_id,
                observed_branch_id: branch.id,
            });
        }
        let (scoped_merge_proof, strategy_witness) = match replay_event.detail.as_ref() {
            Some(ReplayEventDetail::BranchMergeSummary {
                scoped_merge_proof,
                strategy_witness,
                ..
            }) => (
                Some(scoped_merge_proof.clone()),
                Some(strategy_witness.clone()),
            ),
            _ => (None, None),
        };
        self.merge_compatibility_artifact_from_parts(
            branch_basis,
            branch,
            scoped_merge_proof,
            strategy_witness,
        )
    }

    pub fn readmit_merge_compatibility_artifact(
        &mut self,
        bridged: BoundaryBridgedSignalMergeCompatibilityArtifact,
        branch_basis: SignalBranchBasisArtifact,
        branch: SignalBranchHandle,
        scoped_merge_proof: Option<ScopedMergeProofPacket>,
        strategy_witness: Option<SignalMergeStrategyWitness>,
    ) -> SignalMergeCompatibilityOutcome {
        self.with_telemetry(|telemetry| {
            telemetry.transaction.merge_compatibility_readmission_count += 1;
        });
        let validated_basis = match branch_basis_validation_to_denial(
            self.validate_branch_basis_artifact(branch_basis, branch.clone()),
            &branch,
        ) {
            Ok(artifact) => artifact,
            Err(denial) => {
                self.with_telemetry(|telemetry| {
                    telemetry
                        .transaction
                        .merge_compatibility_readmission_denial_count += 1;
                });
                return TransitionOutcome::denied(denial);
            }
        };

        let Some(scoped_merge_proof) = scoped_merge_proof else {
            self.with_telemetry(|telemetry| {
                telemetry
                    .transaction
                    .merge_compatibility_readmission_denial_count += 1;
            });
            return TransitionOutcome::denied(
                SignalMergeCompatibilityDenial::MissingScopedMergeProof {
                    branch_id: branch.id,
                },
            );
        };
        let Some(strategy_witness) = strategy_witness else {
            self.with_telemetry(|telemetry| {
                telemetry
                    .transaction
                    .merge_compatibility_readmission_denial_count += 1;
            });
            return TransitionOutcome::denied(
                SignalMergeCompatibilityDenial::MissingStrategyWitness {
                    branch_id: branch.id,
                },
            );
        };

        let expected_facts = bridged.payload().fact_inventory();
        if expected_facts.branch_basis_digest() != validated_basis.payload().basis_digest() {
            self.with_telemetry(|telemetry| {
                telemetry
                    .transaction
                    .merge_compatibility_readmission_denial_count += 1;
            });
            return TransitionOutcome::denied(
                SignalMergeCompatibilityDenial::ReadmissionBasisMismatch {
                    expected_branch_basis_digest: expected_facts.branch_basis_digest().to_owned(),
                    observed_branch_basis_digest: validated_basis
                        .payload()
                        .basis_digest()
                        .to_owned(),
                },
            );
        }
        if let Err(denial) =
            compare_retained_inputs(expected_facts, &scoped_merge_proof, &strategy_witness)
        {
            self.with_telemetry(|telemetry| {
                telemetry
                    .transaction
                    .merge_compatibility_readmission_denial_count += 1;
            });
            return TransitionOutcome::denied(denial);
        }

        let facts = SignalMergeCompatibilityFactInventory::from_retained(
            validated_basis.payload(),
            &scoped_merge_proof,
            &strategy_witness,
        );
        let basis = build_compatibility_basis(validated_basis.strong_basis().value(), &facts);
        let readmitted = bridged.readmit_with_authority(
            basis,
            worth_proof::AuthorityWitness::from_authority_marker(
                SignalMergeCompatibilityReadmissionAuthority::new(),
            ),
        );
        TransitionOutcome::success(readmitted)
    }
}

use serde::Serialize;
use worth_proof::TransitionOutcome;

use crate::diagnostics::replay::{ReplayEvent, ReplayEventDetail};
use crate::logic::transaction::runtime::state::merge::{
    bridged_compatibility_posture_kind, compatibility_posture_kind,
    BoundaryBridgedSignalMergeCompatibilityArtifact, BranchMergeExecutionSummary,
    BranchMergeResult, ScopedMergeProofPacket, SignalMergeCompatibilityArtifact,
    SignalMergeCompatibilityDenial, SignalMergeCompatibilityWitness, SignalMergeStrategyWitness,
};
use crate::logic::transaction::runtime::state::runtime_state::SignalRuntime;
use crate::logic::transaction::runtime::state::{
    canonical_digest, SignalBranchBasisArtifact, SignalBranchBasisValidationOutcome,
    SignalMergeCompatibilityPostureKind, StaleSignalBranchBasisArtifact,
};
use crate::state::SignalBranchHandle;

use super::absence::SignalMergeSupportInspectionAbsence;
use super::readiness::SignalMergeSupportReadinessPosture;
use super::support_rows::{
    SignalBranchBasisInspectionRow, SignalCompatibilityInspectionRow,
    SignalScopedMergeInspectionRow, SignalStrategyInspectionRow,
};

pub type SignalMergeSupportInspectionOutcome =
    TransitionOutcome<SignalMergeSupportInspectionWitness, SignalMergeSupportInspectionAbsence>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SignalMergeSupportInspectionWitness {
    inspection_digest: String,
    branch_basis_row: SignalBranchBasisInspectionRow,
    scope_row: SignalScopedMergeInspectionRow,
    strategy_row: SignalStrategyInspectionRow,
    compatibility_row: SignalCompatibilityInspectionRow,
    readiness_posture: SignalMergeSupportReadinessPosture,
}

impl SignalMergeSupportInspectionWitness {
    fn new(
        branch_basis_row: SignalBranchBasisInspectionRow,
        scope_row: SignalScopedMergeInspectionRow,
        strategy_row: SignalStrategyInspectionRow,
        compatibility_row: SignalCompatibilityInspectionRow,
        readiness_posture: SignalMergeSupportReadinessPosture,
    ) -> Self {
        let inspection_digest = canonical_digest(&(
            &branch_basis_row,
            &scope_row,
            &strategy_row,
            &compatibility_row,
            readiness_posture,
        ));
        Self {
            inspection_digest,
            branch_basis_row,
            scope_row,
            strategy_row,
            compatibility_row,
            readiness_posture,
        }
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }

    pub fn branch_basis_row(&self) -> &SignalBranchBasisInspectionRow {
        &self.branch_basis_row
    }

    pub fn scope_row(&self) -> &SignalScopedMergeInspectionRow {
        &self.scope_row
    }

    pub fn strategy_row(&self) -> &SignalStrategyInspectionRow {
        &self.strategy_row
    }

    pub fn compatibility_row(&self) -> &SignalCompatibilityInspectionRow {
        &self.compatibility_row
    }

    pub fn readiness_posture(&self) -> SignalMergeSupportReadinessPosture {
        self.readiness_posture
    }
}

fn branch_basis_validation_to_absence(
    outcome: SignalBranchBasisValidationOutcome,
    branch: &SignalBranchHandle,
) -> Result<SignalBranchBasisArtifact, SignalMergeSupportInspectionAbsence> {
    match outcome {
        TransitionOutcome::Success(artifact) => Ok(artifact),
        TransitionOutcome::Denied(denial) => {
            Err(SignalMergeSupportInspectionAbsence::CompatibilityDenied(
                SignalMergeCompatibilityDenial::BranchBasisDenied(denial),
            ))
        }
        TransitionOutcome::Stale(stale) => {
            Err(SignalMergeSupportInspectionAbsence::CompatibilityDenied(
                SignalMergeCompatibilityDenial::StaleBranchBasis {
                    branch_id: branch.id,
                    basis_digest: {
                        let stale: StaleSignalBranchBasisArtifact = stale;
                        stale.payload().basis_digest().to_owned()
                    },
                },
            ))
        }
        TransitionOutcome::Deferred(impossible) => match impossible {},
        TransitionOutcome::RebindRequired(impossible) => match impossible {},
        TransitionOutcome::Failed(impossible) => match impossible {},
    }
}

fn cross_basis_absence(
    expected_branch_id: crate::state::SignalBranchId,
    observed_branch_id: crate::state::SignalBranchId,
) -> SignalMergeSupportInspectionAbsence {
    SignalMergeSupportInspectionAbsence::CompatibilityDenied(
        SignalMergeCompatibilityDenial::CrossBasisMismatch {
            expected_branch_id,
            observed_branch_id,
        },
    )
}

fn compare_retained_support_inputs<'a>(
    branch_basis: &crate::logic::transaction::runtime::state::SignalBranchBasis,
    scoped_merge_proof: Option<&ScopedMergeProofPacket>,
    strategy_witness: Option<&SignalMergeStrategyWitness>,
    compatibility_witness: Option<&'a SignalMergeCompatibilityWitness>,
    branch_id: crate::state::SignalBranchId,
) -> Result<&'a SignalMergeCompatibilityWitness, SignalMergeSupportInspectionAbsence> {
    let Some(scoped_merge_proof) = scoped_merge_proof else {
        return Err(SignalMergeSupportInspectionAbsence::CompatibilityDenied(
            SignalMergeCompatibilityDenial::MissingScopedMergeProof { branch_id },
        ));
    };
    let Some(strategy_witness) = strategy_witness else {
        return Err(SignalMergeSupportInspectionAbsence::CompatibilityDenied(
            SignalMergeCompatibilityDenial::MissingStrategyWitness { branch_id },
        ));
    };
    let Some(compatibility_witness) = compatibility_witness else {
        return Err(SignalMergeSupportInspectionAbsence::MissingCompatibilityWitness { branch_id });
    };

    let facts = compatibility_witness.fact_inventory();
    if facts.branch_basis_digest() != branch_basis.basis_digest() {
        return Err(SignalMergeSupportInspectionAbsence::CompatibilityDenied(
            SignalMergeCompatibilityDenial::ReadmissionBasisMismatch {
                expected_branch_basis_digest: facts.branch_basis_digest().to_owned(),
                observed_branch_basis_digest: branch_basis.basis_digest().to_owned(),
            },
        ));
    }
    if facts.declaration_digest() != scoped_merge_proof.declaration_digest()
        || facts.admitted_scope_digest() != scoped_merge_proof.admitted_scope_digest()
    {
        return Err(SignalMergeSupportInspectionAbsence::CompatibilityDenied(
            SignalMergeCompatibilityDenial::ScopedMergeProofMismatch {
                expected_declaration_digest: facts.declaration_digest().to_owned(),
                observed_declaration_digest: scoped_merge_proof.declaration_digest().to_owned(),
            },
        ));
    }
    if facts.strategy_witness_digest() != strategy_witness.witness_digest() {
        return Err(SignalMergeSupportInspectionAbsence::CompatibilityDenied(
            SignalMergeCompatibilityDenial::StrategyWitnessMismatch {
                expected_witness_digest: facts.strategy_witness_digest().to_owned(),
                observed_witness_digest: strategy_witness.witness_digest().to_owned(),
            },
        ));
    }

    Ok(compatibility_witness)
}

fn build_support_witness(
    branch_basis: &crate::logic::transaction::runtime::state::SignalBranchBasis,
    strategy_witness: &SignalMergeStrategyWitness,
    compatibility_witness: &SignalMergeCompatibilityWitness,
    posture_kind: SignalMergeCompatibilityPostureKind,
) -> SignalMergeSupportInspectionWitness {
    SignalMergeSupportInspectionWitness::new(
        SignalBranchBasisInspectionRow::from_branch_basis(branch_basis),
        SignalScopedMergeInspectionRow::from_compatibility_facts(
            compatibility_witness.fact_inventory(),
        ),
        SignalStrategyInspectionRow::from_strategy_witness(strategy_witness),
        SignalCompatibilityInspectionRow::from_witness(compatibility_witness, posture_kind),
        SignalMergeSupportReadinessPosture::from_compatibility_posture(posture_kind),
    )
}

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn merge_support_inspection_from_retained_parts(
        &mut self,
        branch_basis: SignalBranchBasisArtifact,
        branch: SignalBranchHandle,
        scoped_merge_proof: Option<&ScopedMergeProofPacket>,
        strategy_witness: Option<&SignalMergeStrategyWitness>,
        compatibility_witness: Option<&SignalMergeCompatibilityWitness>,
        posture_kind: SignalMergeCompatibilityPostureKind,
    ) -> SignalMergeSupportInspectionOutcome {
        let validated_basis = match branch_basis_validation_to_absence(
            self.validate_branch_basis_artifact(branch_basis, branch.clone()),
            &branch,
        ) {
            Ok(artifact) => artifact,
            Err(absence) => return TransitionOutcome::Denied(absence),
        };

        let compatibility_witness = match compare_retained_support_inputs(
            validated_basis.payload(),
            scoped_merge_proof,
            strategy_witness,
            compatibility_witness,
            branch.id,
        ) {
            Ok(witness) => witness,
            Err(absence) => return TransitionOutcome::Denied(absence),
        };

        TransitionOutcome::success(build_support_witness(
            validated_basis.payload(),
            strategy_witness.expect("validated strategy witness should exist"),
            compatibility_witness,
            posture_kind,
        ))
    }

    pub fn merge_result_support_inspection(
        &mut self,
        branch_basis: SignalBranchBasisArtifact,
        branch: SignalBranchHandle,
        result: &BranchMergeResult,
    ) -> SignalMergeSupportInspectionOutcome {
        if branch.id != result.target_branch {
            return TransitionOutcome::Denied(cross_basis_absence(result.target_branch, branch.id));
        }
        self.merge_support_inspection_from_retained_parts(
            branch_basis,
            branch,
            Some(&result.scoped_merge_proof),
            Some(&result.strategy_witness),
            Some(&result.compatibility_witness),
            SignalMergeCompatibilityPostureKind::CurrentBasis,
        )
    }

    pub fn merge_execution_summary_support_inspection(
        &mut self,
        branch_basis: SignalBranchBasisArtifact,
        branch: SignalBranchHandle,
        summary: &BranchMergeExecutionSummary,
    ) -> SignalMergeSupportInspectionOutcome {
        if branch.id != summary.target_branch_id {
            return TransitionOutcome::Denied(cross_basis_absence(
                summary.target_branch_id,
                branch.id,
            ));
        }
        self.merge_support_inspection_from_retained_parts(
            branch_basis,
            branch,
            Some(&summary.scoped_merge_proof),
            Some(&summary.strategy_witness),
            Some(&summary.compatibility_witness),
            SignalMergeCompatibilityPostureKind::CurrentBasis,
        )
    }

    pub fn replay_merge_support_inspection(
        &mut self,
        branch_basis: SignalBranchBasisArtifact,
        branch: SignalBranchHandle,
        replay_event: &ReplayEvent,
    ) -> SignalMergeSupportInspectionOutcome {
        if branch.id != replay_event.branch_id {
            return TransitionOutcome::Denied(cross_basis_absence(
                replay_event.branch_id,
                branch.id,
            ));
        }
        let Some(ReplayEventDetail::BranchMergeSummary {
            strategy_witness,
            compatibility_witness,
            scoped_merge_proof,
            ..
        }) = replay_event.detail.as_ref()
        else {
            return TransitionOutcome::Denied(
                SignalMergeSupportInspectionAbsence::ReplayDetailUnavailable {
                    branch_id: replay_event.branch_id,
                    replay_kind: replay_event.kind,
                },
            );
        };
        self.merge_support_inspection_from_retained_parts(
            branch_basis,
            branch,
            Some(scoped_merge_proof),
            Some(strategy_witness),
            Some(compatibility_witness),
            SignalMergeCompatibilityPostureKind::CurrentBasis,
        )
    }

    pub fn merge_compatibility_support_inspection(
        &mut self,
        branch_basis: SignalBranchBasisArtifact,
        branch: SignalBranchHandle,
        scoped_merge_proof: &ScopedMergeProofPacket,
        strategy_witness: &SignalMergeStrategyWitness,
        compatibility_artifact: &SignalMergeCompatibilityArtifact,
    ) -> SignalMergeSupportInspectionOutcome {
        self.merge_support_inspection_from_retained_parts(
            branch_basis,
            branch,
            Some(scoped_merge_proof),
            Some(strategy_witness),
            Some(compatibility_artifact.payload()),
            compatibility_posture_kind(compatibility_artifact),
        )
    }

    pub fn bridged_merge_compatibility_support_inspection(
        &mut self,
        branch_basis: SignalBranchBasisArtifact,
        branch: SignalBranchHandle,
        scoped_merge_proof: &ScopedMergeProofPacket,
        strategy_witness: &SignalMergeStrategyWitness,
        compatibility_artifact: &BoundaryBridgedSignalMergeCompatibilityArtifact,
    ) -> SignalMergeSupportInspectionOutcome {
        self.merge_support_inspection_from_retained_parts(
            branch_basis,
            branch,
            Some(scoped_merge_proof),
            Some(strategy_witness),
            Some(compatibility_artifact.payload()),
            bridged_compatibility_posture_kind(compatibility_artifact),
        )
    }
}

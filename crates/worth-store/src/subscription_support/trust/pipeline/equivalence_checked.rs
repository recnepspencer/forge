use super::super::equivalence::{
    SupportTrustEquivalenceContract, SupportTrustEquivalenceEvidence, SupportTrustEquivalenceLane,
    SupportTrustTransformedEquivalenceWitness,
};
use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::super::taxonomy::{SupportTrustProvenance, SupportTrustStrength};
use super::drift_checked::SupportTrustDriftChecked;
use super::request::RawSupportTrustRequest;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustEquivalenceChecked {
    drift_checked: SupportTrustDriftChecked,
    transformed_equivalence: Option<SupportTrustTransformedEquivalenceWitness>,
    equivalence_checks_performed: u64,
}

impl SupportTrustEquivalenceChecked {
    pub(super) fn into_operational_inputs(
        self,
    ) -> (
        SupportTrustDriftChecked,
        Option<SupportTrustTransformedEquivalenceWitness>,
        u64,
    ) {
        (
            self.drift_checked,
            self.transformed_equivalence,
            self.equivalence_checks_performed,
        )
    }
}

pub fn check_support_trust_equivalence(
    drift_checked: SupportTrustDriftChecked,
    equivalence_evidence: SupportTrustEquivalenceEvidence,
) -> Result<SupportTrustEquivalenceChecked, SupportTrustFailure> {
    let request = drift_checked.translated().admitted().request();
    let provenance = request.provenance();
    let requested_strength = request.requested_strength();
    let transformed_exact_lane = if requested_strength == SupportTrustStrength::Exact {
        equivalence_lane_for(provenance)
    } else {
        None
    };
    if transformed_exact_lane.is_none() && equivalence_evidence.contract_count() > 0 {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustEquivalenceMissing,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence evidence is only admissible for transformed exact trust",
        ));
    }
    let transformed_equivalence = match transformed_exact_lane {
        Some(lane) => {
            let contract = equivalence_evidence.contract_for(lane).ok_or_else(|| {
                SupportTrustFailure::new(
                    SupportTrustFailureKind::SupportTrustEquivalenceMissing,
                    SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                    "transformed exact support trust requires Phase 3 equivalence proof",
                )
            })?;
            validate_equivalence_contract(&drift_checked, lane, contract)?;
            Some(SupportTrustTransformedEquivalenceWitness::from_contract(
                contract,
            )?)
        }
        None => None,
    };
    if provenance.requires_equivalence_for_exact()
        && requested_strength == SupportTrustStrength::Exact
        && transformed_equivalence.is_none()
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustEquivalenceMissing,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "transformed exact support trust requires Phase 3 equivalence proof",
        ));
    }
    Ok(SupportTrustEquivalenceChecked {
        drift_checked,
        transformed_equivalence,
        equivalence_checks_performed: 1 + equivalence_evidence.contract_count(),
    })
}

fn equivalence_lane_for(provenance: SupportTrustProvenance) -> Option<SupportTrustEquivalenceLane> {
    match provenance {
        SupportTrustProvenance::Rebuilt => Some(SupportTrustEquivalenceLane::Rebuild),
        SupportTrustProvenance::Migrated => Some(SupportTrustEquivalenceLane::Migration),
        SupportTrustProvenance::Replicated => Some(SupportTrustEquivalenceLane::Replication),
        SupportTrustProvenance::Imported => Some(SupportTrustEquivalenceLane::Import),
        _ => None,
    }
}

fn validate_equivalence_contract(
    drift_checked: &SupportTrustDriftChecked,
    lane: SupportTrustEquivalenceLane,
    contract: &SupportTrustEquivalenceContract,
) -> Result<(), SupportTrustFailure> {
    let request = drift_checked.translated().admitted().request();
    let basis = drift_checked
        .translated()
        .admitted()
        .receipt_bundle()
        .operational()
        .basis();
    validate_equivalence_source_binding(basis, lane, contract)?;
    validate_equivalence_target_identity(request, contract)?;
    validate_equivalence_target_basis(basis, contract)?;
    validate_equivalence_target_compatibility(basis, contract)?;
    validate_equivalence_target_portability(basis, contract)?;
    validate_equivalence_target_verdict(contract)
}

fn validate_equivalence_source_binding(
    basis: &SubscriptionSupportOperationalBasis,
    lane: SupportTrustEquivalenceLane,
    contract: &SupportTrustEquivalenceContract,
) -> Result<(), SupportTrustFailure> {
    if contract.lane() != lane || contract.source_basis() != basis {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustBasisMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence contract must be bound to the drift-checked source basis",
        ));
    }
    Ok(())
}

fn validate_equivalence_target_identity(
    request: &RawSupportTrustRequest,
    contract: &SupportTrustEquivalenceContract,
) -> Result<(), SupportTrustFailure> {
    if contract.target_family_id() != request.family_id() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustFamilyMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence target family must match the trust request",
        ));
    }
    if contract.target_support_role() != request.support_role() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustRoleMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence target role must match the trust request",
        ));
    }
    if contract.target_artifact_id() != request.artifact_id() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustBasisMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence target artifact must match the trust request",
        ));
    }
    Ok(())
}

fn validate_equivalence_target_basis(
    basis: &SubscriptionSupportOperationalBasis,
    contract: &SupportTrustEquivalenceContract,
) -> Result<(), SupportTrustFailure> {
    if contract.target_basis_digest() != basis.basis_digest()
        || contract.target_cursor_digest() != basis.cursor_digest()
        || contract.target_checkpoint_digest() != basis.checkpoint_digest()
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustBasisMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence target basis, cursor, and checkpoint must match source proof",
        ));
    }
    Ok(())
}

fn validate_equivalence_target_compatibility(
    basis: &SubscriptionSupportOperationalBasis,
    contract: &SupportTrustEquivalenceContract,
) -> Result<(), SupportTrustFailure> {
    if contract.target_compatibility_digest() != basis.compatibility_digest() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCompatibilityMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence target compatibility must match source proof",
        ));
    }
    Ok(())
}

fn validate_equivalence_target_portability(
    basis: &SubscriptionSupportOperationalBasis,
    contract: &SupportTrustEquivalenceContract,
) -> Result<(), SupportTrustFailure> {
    if contract.target_portability_digest() != basis.portability_digest() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustPortabilityMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence target portability scope must match source proof",
        ));
    }
    Ok(())
}

fn validate_equivalence_target_verdict(
    contract: &SupportTrustEquivalenceContract,
) -> Result<(), SupportTrustFailure> {
    if contract.resume_classification() != SubscriptionResumeClassification::Exact {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustResumeClassificationMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence must prove exact resume classifier equivalence",
        ));
    }
    if contract.operational_verdict() != SubscriptionSupportOperationalVerdict::ExactResumePreserved
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustOperationalVerdictMismatch,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust equivalence must preserve exact operational verdict",
        ));
    }
    Ok(())
}

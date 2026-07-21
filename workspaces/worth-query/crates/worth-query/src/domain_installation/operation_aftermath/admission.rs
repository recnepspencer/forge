use std::marker::PhantomData;

use super::{
    mint_aftermath_authority, WorthQueryAftermathAdmissionDenial,
    WorthQueryAftermathAuthorityBasis, WorthQueryAftermathAuthorityProof,
    WorthQueryAftermathCounters, WorthQueryAftermathKind, WorthQueryAftermathOriginalEvidence,
    WorthQueryAftermathPosture,
};
use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::operation_identity_basis::aftermath_material;
use crate::domain_installation::{
    WorthQueryAftermathPostcondition, WorthQueryBoundDomainOperation,
    WorthQueryCompletedWorkflowTrace, WorthQueryOperationReversalContract,
};
use crate::identity::hash_parts;

type AftermathMarker<OO, OF, OL> = fn() -> (OO, OF, OL);

pub(crate) struct WorthQueryAdmittedAftermath<D, OO, OF, OL, CO, CF, CL>
where
    OL: BasisOperationLane,
    CL: BasisOperationLane,
{
    pub(crate) candidate: WorthQueryBoundDomainOperation<D, CO, CF, CL>,
    pub(crate) kind: WorthQueryAftermathKind,
    pub(crate) postcondition: WorthQueryAftermathPostcondition,
    pub(crate) original_trace_identity: String,
    pub(crate) counters: WorthQueryAftermathCounters,
    pub(crate) proof: WorthQueryAftermathAuthorityProof,
    pub(crate) original_evidence: WorthQueryAftermathOriginalEvidence,
    pub(crate) _original: PhantomData<AftermathMarker<OO, OF, OL>>,
}

pub struct WorthQueryExactInverseCapability<D, OO, OF, OL, CO, CF, CL>
where
    OL: BasisOperationLane,
    CL: BasisOperationLane,
{
    pub(crate) admitted: WorthQueryAdmittedAftermath<D, OO, OF, OL, CO, CF, CL>,
}

pub struct WorthQueryCompensationCapability<D, OO, OF, OL, CO, CF, CL>
where
    OL: BasisOperationLane,
    CL: BasisOperationLane,
{
    pub(crate) admitted: WorthQueryAdmittedAftermath<D, OO, OF, OL, CO, CF, CL>,
}

pub enum WorthQueryAftermathAdmission<D, OO, OF, OL, CO, CF, CL>
where
    OL: BasisOperationLane,
    CL: BasisOperationLane,
{
    ExactInverse(WorthQueryExactInverseCapability<D, OO, OF, OL, CO, CF, CL>),
    Compensation(WorthQueryCompensationCapability<D, OO, OF, OL, CO, CF, CL>),
    Denied {
        denial: WorthQueryAftermathAdmissionDenial,
        posture: WorthQueryAftermathPosture,
        counters: WorthQueryAftermathCounters,
    },
}

impl<D, OO, OF, OL: BasisOperationLane> WorthQueryCompletedWorkflowTrace<D, OO, OF, OL> {
    pub fn aftermath_posture(&self) -> WorthQueryAftermathPosture {
        posture(self.bound().definition().semantics().reversal.clone())
    }

    pub fn admit_aftermath<CO, CF, CL: BasisOperationLane>(
        &self,
        candidate: WorthQueryBoundDomainOperation<D, CO, CF, CL>,
    ) -> WorthQueryAftermathAdmission<D, OO, OF, OL, CO, CF, CL> {
        let mut counters = WorthQueryAftermathCounters {
            runtime_authority_checks: 1,
            installation_generation_checks: 1,
            basis_checks: 1,
            candidate_operation_checks: 1,
            ..Default::default()
        };
        let declared = self.bound().definition().semantics().reversal.clone();
        let posture = posture(declared.clone());
        if !self.bound().installation_is_current() {
            return denied(
                WorthQueryAftermathAdmissionDenial::OriginalInstallationStale,
                posture,
                counters,
            );
        }
        if !candidate.installation_is_current() {
            return denied(
                WorthQueryAftermathAdmissionDenial::CandidateInstallationStale,
                posture,
                counters,
            );
        }
        let original_authority = self.bound().operation().domain_authority();
        let candidate_authority = candidate.operation().domain_authority();
        if original_authority.runtime_authority() != candidate_authority.runtime_authority() {
            return denied(
                WorthQueryAftermathAdmissionDenial::ForeignRuntime,
                posture,
                counters,
            );
        }
        if self.bound().operation().installation_generation()
            != candidate.operation().installation_generation()
        {
            return denied(
                WorthQueryAftermathAdmissionDenial::InstallationGenerationMismatch,
                posture,
                counters,
            );
        }
        if self.bound().basis().capability_digest() != candidate.basis().capability_digest() {
            return denied(
                WorthQueryAftermathAdmissionDenial::BasisMismatch,
                posture,
                counters,
            );
        }
        let (kind, expected_operation, expected_lowering, postcondition) = match declared {
            WorthQueryOperationReversalContract::ExactInverseWithPostcondition {
                operation,
                lowering_family,
                postcondition,
            } => (
                WorthQueryAftermathKind::ExactInverse,
                operation,
                Some(lowering_family),
                postcondition,
            ),
            WorthQueryOperationReversalContract::CompensationWithPostcondition {
                operation,
                postcondition,
            } => (
                WorthQueryAftermathKind::Compensation,
                operation,
                None,
                postcondition,
            ),
            WorthQueryOperationReversalContract::ExactInverse { .. }
            | WorthQueryOperationReversalContract::Compensation { .. } => {
                return denied(
                    WorthQueryAftermathAdmissionDenial::DeclarationIncomplete,
                    posture,
                    counters,
                )
            }
            WorthQueryOperationReversalContract::Irreversible => {
                return denied(
                    WorthQueryAftermathAdmissionDenial::Irreversible,
                    posture,
                    counters,
                )
            }
            WorthQueryOperationReversalContract::ProvisionalDiscard => {
                return denied(
                    WorthQueryAftermathAdmissionDenial::ProvisionalDiscardOnly,
                    posture,
                    counters,
                )
            }
            WorthQueryOperationReversalContract::RebuildRequired { .. } => {
                return denied(
                    WorthQueryAftermathAdmissionDenial::RebuildRequired,
                    posture,
                    counters,
                )
            }
        };
        let effect_receipt_identities = self
            .stage_receipts()
            .iter()
            .flat_map(|stage| stage.effect_evidence())
            .map(|effect| effect.receipt_identity().to_owned())
            .collect::<Vec<_>>();
        counters.effect_receipt_checks = effect_receipt_identities.len();
        if effect_receipt_identities.is_empty() {
            return denied(
                WorthQueryAftermathAdmissionDenial::NoExecutedEffects,
                posture,
                counters,
            );
        }
        if candidate.definition().identity() != &expected_operation {
            return denied(
                WorthQueryAftermathAdmissionDenial::CandidateOperationMismatch,
                posture,
                counters,
            );
        }
        counters.candidate_lowering_checks = usize::from(expected_lowering.is_some());
        if expected_lowering
            .is_some_and(|expected| candidate.definition().semantics().lowering.family != expected)
        {
            return denied(
                WorthQueryAftermathAdmissionDenial::CandidateLoweringMismatch,
                posture,
                counters,
            );
        }
        counters.postcondition_checks = 1;
        if !valid_postcondition(kind, &postcondition) {
            return denied(
                WorthQueryAftermathAdmissionDenial::InvalidPostcondition,
                posture,
                counters,
            );
        }
        let identity = hash_parts(&[
            "worth_query_admitted_aftermath_v1".into(),
            format!("original:{}", self.identity()),
            format!("candidate:{}", candidate.binding_identity()),
            format!("candidate-capability:{}", candidate.capability_identity()),
            format!("effects:{}", effect_receipt_identities.join(",")),
            format!(
                "lineage:{}",
                self.lineage_report().map_or(
                    "none",
                    crate::domain_installation::WorthQueryTraceLineageReport::identity
                )
            ),
            aftermath_material(kind, &postcondition),
        ]);
        let basis = WorthQueryAftermathAuthorityBasis {
            runtime_authority: original_authority.runtime_authority().as_u64(),
            installation_generation: self.bound().operation().installation_generation().ordinal(),
            original_operation_identity: self.bound().definition().canonical_identity().to_owned(),
            original_binding_identity: self.bound().binding_identity().to_owned(),
            original_capability_identity: self.bound().capability_identity(),
            original_trace_identity: self.identity().to_owned(),
            candidate_operation_identity: candidate.definition().canonical_identity().to_owned(),
            candidate_binding_identity: candidate.binding_identity().to_owned(),
            candidate_capability_identity: candidate.capability_identity(),
            basis_identity: candidate.basis().capability_digest().to_owned(),
            effect_receipt_identities,
            original_lineage_report_identity: self
                .lineage_report()
                .map(|report| report.identity().to_owned()),
        };
        let proof = mint_aftermath_authority(
            identity,
            self.phase_proof().payload().identity().to_owned(),
            basis,
        );
        debug_assert_eq!(proof.payload().predecessor_identity(), self.identity());
        let admitted = WorthQueryAdmittedAftermath {
            candidate,
            kind,
            postcondition: postcondition.clone(),
            original_trace_identity: self.identity().to_owned(),
            counters,
            proof,
            original_evidence: WorthQueryAftermathOriginalEvidence::new(
                self.identity().to_owned(),
                kind,
                postcondition.clone(),
                self.stage_receipts()
                    .iter()
                    .flat_map(|stage| stage.effect_evidence().iter().cloned())
                    .collect(),
                self.lineage_report()
                    .map(|report| report.identity().to_owned()),
            ),
            _original: PhantomData,
        };
        match kind {
            WorthQueryAftermathKind::ExactInverse => {
                WorthQueryAftermathAdmission::ExactInverse(WorthQueryExactInverseCapability {
                    admitted,
                })
            }
            WorthQueryAftermathKind::Compensation => {
                WorthQueryAftermathAdmission::Compensation(WorthQueryCompensationCapability {
                    admitted,
                })
            }
        }
    }
}

fn posture(contract: WorthQueryOperationReversalContract) -> WorthQueryAftermathPosture {
    match contract {
        WorthQueryOperationReversalContract::Irreversible => {
            WorthQueryAftermathPosture::Irreversible
        }
        WorthQueryOperationReversalContract::ProvisionalDiscard => {
            WorthQueryAftermathPosture::ProvisionalDiscard
        }
        WorthQueryOperationReversalContract::ExactInverse { .. }
        | WorthQueryOperationReversalContract::Compensation { .. } => {
            WorthQueryAftermathPosture::DeclarationIncomplete
        }
        WorthQueryOperationReversalContract::ExactInverseWithPostcondition {
            operation,
            lowering_family,
            postcondition,
        } => WorthQueryAftermathPosture::ExactInverse {
            operation,
            lowering_family,
            postcondition,
        },
        WorthQueryOperationReversalContract::CompensationWithPostcondition {
            operation,
            postcondition,
        } => WorthQueryAftermathPosture::Compensation {
            operation,
            postcondition,
        },
        WorthQueryOperationReversalContract::RebuildRequired { recovery_family } => {
            WorthQueryAftermathPosture::RebuildRequired { recovery_family }
        }
    }
}

fn valid_postcondition(
    kind: WorthQueryAftermathKind,
    postcondition: &WorthQueryAftermathPostcondition,
) -> bool {
    matches!(
        (kind, postcondition),
        (
            WorthQueryAftermathKind::ExactInverse,
            WorthQueryAftermathPostcondition::ExactPriorTruth
        ) | (
            WorthQueryAftermathKind::Compensation,
            WorthQueryAftermathPostcondition::InvariantRestored { .. }
                | WorthQueryAftermathPostcondition::BusinessPostcondition { .. }
        )
    )
}

fn denied<D, OO, OF, OL, CO, CF, CL>(
    denial: WorthQueryAftermathAdmissionDenial,
    posture: WorthQueryAftermathPosture,
    counters: WorthQueryAftermathCounters,
) -> WorthQueryAftermathAdmission<D, OO, OF, OL, CO, CF, CL>
where
    OL: BasisOperationLane,
    CL: BasisOperationLane,
{
    WorthQueryAftermathAdmission::Denied {
        denial,
        posture,
        counters,
    }
}

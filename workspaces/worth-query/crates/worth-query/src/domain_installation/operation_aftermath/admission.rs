use std::marker::PhantomData;

use super::{
    WorthQueryAftermathAdmissionDenial, WorthQueryAftermathAuthorityProof,
    WorthQueryAftermathCounters, WorthQueryAftermathKind, WorthQueryAftermathOriginalEvidence,
    WorthQueryAftermathPosture,
};
use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::{
    WorthQueryAftermathPostcondition, WorthQueryBoundDomainOperation,
    WorthQueryCompletedWorkflowTrace, WorthQueryOperationReversalContract,
};

mod validation;

use validation::{mint_validated_aftermath, validate_aftermath_candidate};

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
        let validated =
            match validate_aftermath_candidate(self, &candidate, declared, &mut counters) {
                Ok(validated) => validated,
                Err(denial) => return denied(denial, posture, counters),
            };
        let kind = validated.kind;
        let admitted = mint_validated_aftermath(self, candidate, validated, counters);
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

use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::operation_authority_chain::{
    mint_operation_phase_proof, operation_phase_basis, WorthQueryDiscardedProvisionalPhase,
    WorthQueryOperationPhaseProof,
};
use crate::domain_installation::WorthQueryCompletedWorkflowTrace;
use crate::identity::hash_parts;
use worth_proof::TransitionOutcome;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryProvisionalDiscardDenial {
    StaleInstallationGeneration,
    ExecutedEffectsPresent,
}

pub struct WorthQueryDiscardedProvisionalTrace<D, O, F, L: BasisOperationLane> {
    trace: WorthQueryCompletedWorkflowTrace<D, O, F, L>,
    identity: String,
    proof: WorthQueryOperationPhaseProof<WorthQueryDiscardedProvisionalPhase>,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryDiscardedProvisionalTrace<D, O, F, L> {
    pub fn identity(&self) -> &str {
        debug_assert_eq!(self.proof.payload().identity(), self.identity);
        &self.identity
    }
    pub fn original_trace_identity(&self) -> &str {
        self.trace.identity()
    }
    pub fn into_original_trace(self) -> WorthQueryCompletedWorkflowTrace<D, O, F, L> {
        self.trace
    }
}

impl<D, O, F, L: BasisOperationLane> WorthQueryCompletedWorkflowTrace<D, O, F, L> {
    pub fn discard_provisional(
        self,
    ) -> TransitionOutcome<
        WorthQueryDiscardedProvisionalTrace<D, O, F, L>,
        WorthQueryProvisionalDiscardDenial,
        std::convert::Infallible,
        WorthQueryProvisionalDiscardDenial,
        WorthQueryProvisionalDiscardDenial,
        WorthQueryProvisionalDiscardDenial,
    > {
        if !self.bound().installation_is_current() {
            return TransitionOutcome::Stale(
                WorthQueryProvisionalDiscardDenial::StaleInstallationGeneration,
            );
        }
        if self
            .stage_receipts()
            .iter()
            .any(|stage| !stage.effect_evidence().is_empty())
        {
            return TransitionOutcome::Denied(
                WorthQueryProvisionalDiscardDenial::ExecutedEffectsPresent,
            );
        }
        let identity = hash_parts(&[
            "worth_query_provisional_discard_v1".into(),
            format!("trace:{}", self.identity()),
        ]);
        let proof = mint_operation_phase_proof(
            identity.clone(),
            Some(self.phase_proof().payload().identity()),
            operation_phase_basis(self.phase_proof()).clone(),
        );
        TransitionOutcome::Success(WorthQueryDiscardedProvisionalTrace {
            trace: self,
            identity,
            proof,
        })
    }
}

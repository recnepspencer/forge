use crate::runtime::{
    WorthQueryAuthoritativeMutationObligationDispatch, WorthQueryBackendAdmissibleMutation,
    WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchDescriptorDenial,
    WorthQueryVerifiedExistingTruthAssertion, WorthQueryWriteCommand,
};

use super::{
    WorthQueryAdmittedIntentPlanCore, WorthQueryIntentAdmissionCoveredEntrypoint,
    WorthQueryIntentAdmissionEligibility, WorthQueryIntentAdmissionExecutionSeam,
    WorthQueryIntentAdmissionFamily, WorthQueryIntentEligibilityTraceEvidence,
};
use crate::intent_admission::WorthQueryAuthoritativeMutationBatchIntentSeed;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryAuthoritativeMutationExecutionPlan {
    inner: WorthQueryAdmittedIntentPlanCore,
    command: WorthQueryWriteCommand,
    verified_existing_truth_assertion: Option<WorthQueryVerifiedExistingTruthAssertion>,
    admitted_mutation: WorthQueryBackendAdmissibleMutation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryAuthoritativeMutationBatchExecutionPlan {
    inner: WorthQueryAdmittedIntentPlanCore,
    seed: WorthQueryAuthoritativeMutationBatchIntentSeed,
    obligation_dispatch: Option<WorthQueryAuthoritativeMutationObligationDispatch>,
}

impl WorthQueryAuthoritativeMutationExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: WorthQueryIntentAdmissionEligibility,
        execution_seam: WorthQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        let verified_existing_truth_assertion = eligibility
            .request()
            .authoritative_mutation_seed()
            .and_then(|seed| seed.verified_existing_truth_assertion().cloned());
        let command = eligibility
            .request()
            .authoritative_mutation_seed()
            .expect("authoritative mutation plan requires mutation seed")
            .command()
            .clone();
        let admitted_mutation = eligibility
            .request()
            .authoritative_mutation_seed()
            .and_then(|seed| seed.admitted_mutation().cloned())
            .expect("admitted authoritative mutation plan requires contract-admitted mutation");
        Self {
            inner: WorthQueryAdmittedIntentPlanCore::from_eligibility(
                eligibility,
                Some(execution_seam),
            ),
            command,
            verified_existing_truth_assertion,
            admitted_mutation,
        }
    }

    pub(crate) fn admitted_mutation(&self) -> &WorthQueryBackendAdmissibleMutation {
        &self.admitted_mutation
    }

    pub(crate) fn core(&self) -> &WorthQueryAdmittedIntentPlanCore {
        &self.inner
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        self.inner.entrypoint
    }

    pub fn execution_seam(&self) -> Option<WorthQueryIntentAdmissionExecutionSeam> {
        self.inner.execution_seam
    }

    pub fn command(&self) -> &WorthQueryWriteCommand {
        &self.command
    }

    pub fn verified_existing_truth_assertion(
        &self,
    ) -> Option<&WorthQueryVerifiedExistingTruthAssertion> {
        self.verified_existing_truth_assertion.as_ref()
    }

    pub fn request_digest(&self) -> &str {
        &self.inner.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.inner.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &WorthQueryIntentEligibilityTraceEvidence {
        &self.inner.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.inner.decision_digest
    }
}

impl WorthQueryAuthoritativeMutationBatchExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: WorthQueryIntentAdmissionEligibility,
        execution_seam: WorthQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        let seed = eligibility
            .request()
            .authoritative_mutation_batch_seed()
            .expect("authoritative mutation batch plan requires batch seed")
            .clone();
        Self {
            inner: WorthQueryAdmittedIntentPlanCore::from_eligibility(
                eligibility,
                Some(execution_seam),
            ),
            seed,
            obligation_dispatch: None,
        }
    }

    pub(crate) fn core(&self) -> &WorthQueryAdmittedIntentPlanCore {
        &self.inner
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        self.inner.entrypoint
    }

    pub fn execution_seam(&self) -> Option<WorthQueryIntentAdmissionExecutionSeam> {
        self.inner.execution_seam
    }

    pub fn batch_seed(&self) -> &WorthQueryAuthoritativeMutationBatchIntentSeed {
        &self.seed
    }

    pub fn graph_touch_descriptor(
        &self,
    ) -> Result<WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchDescriptorDenial> {
        self.seed.graph_touch_descriptor()
    }

    pub fn obligation_dispatch(
        &self,
    ) -> Option<&WorthQueryAuthoritativeMutationObligationDispatch> {
        self.obligation_dispatch.as_ref()
    }

    pub fn request_digest(&self) -> &str {
        &self.inner.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.inner.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &WorthQueryIntentEligibilityTraceEvidence {
        &self.inner.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.inner.decision_digest
    }
}

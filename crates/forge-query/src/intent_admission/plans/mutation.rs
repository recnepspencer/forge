use crate::runtime::{ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryWriteCommand};

use super::{
    ForgeQueryAdmittedIntentPlanCore, ForgeQueryIntentAdmissionCoveredEntrypoint,
    ForgeQueryIntentAdmissionEligibility, ForgeQueryIntentAdmissionExecutionSeam,
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentEligibilityTraceEvidence,
};
use crate::intent_admission::ForgeQueryAuthoritativeMutationBatchIntentSeed;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAuthoritativeMutationExecutionPlan {
    inner: ForgeQueryAdmittedIntentPlanCore,
    command: ForgeQueryWriteCommand,
    verified_existing_truth_assertion: Option<ForgeQueryVerifiedExistingTruthAssertion>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAuthoritativeMutationBatchExecutionPlan {
    inner: ForgeQueryAdmittedIntentPlanCore,
    seed: ForgeQueryAuthoritativeMutationBatchIntentSeed,
}

impl ForgeQueryAuthoritativeMutationExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: ForgeQueryIntentAdmissionEligibility,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
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
        Self {
            inner: ForgeQueryAdmittedIntentPlanCore::from_eligibility(
                eligibility,
                Some(execution_seam),
            ),
            command,
            verified_existing_truth_assertion,
        }
    }

    pub(crate) fn core(&self) -> &ForgeQueryAdmittedIntentPlanCore {
        &self.inner
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.inner.entrypoint
    }

    pub fn execution_seam(&self) -> Option<ForgeQueryIntentAdmissionExecutionSeam> {
        self.inner.execution_seam
    }

    pub fn command(&self) -> &ForgeQueryWriteCommand {
        &self.command
    }

    pub fn verified_existing_truth_assertion(
        &self,
    ) -> Option<&ForgeQueryVerifiedExistingTruthAssertion> {
        self.verified_existing_truth_assertion.as_ref()
    }

    pub fn request_digest(&self) -> &str {
        &self.inner.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.inner.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &ForgeQueryIntentEligibilityTraceEvidence {
        &self.inner.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.inner.decision_digest
    }
}

impl ForgeQueryAuthoritativeMutationBatchExecutionPlan {
    pub(crate) fn from_eligibility(
        eligibility: ForgeQueryIntentAdmissionEligibility,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    ) -> Self {
        let seed = eligibility
            .request()
            .authoritative_mutation_batch_seed()
            .expect("authoritative mutation batch plan requires batch seed")
            .clone();
        Self {
            inner: ForgeQueryAdmittedIntentPlanCore::from_eligibility(
                eligibility,
                Some(execution_seam),
            ),
            seed,
        }
    }

    pub(crate) fn core(&self) -> &ForgeQueryAdmittedIntentPlanCore {
        &self.inner
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.inner.family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.inner.entrypoint
    }

    pub fn execution_seam(&self) -> Option<ForgeQueryIntentAdmissionExecutionSeam> {
        self.inner.execution_seam
    }

    pub fn batch_seed(&self) -> &ForgeQueryAuthoritativeMutationBatchIntentSeed {
        &self.seed
    }

    pub fn request_digest(&self) -> &str {
        &self.inner.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.inner.eligibility_digest
    }

    pub fn eligibility_trace(&self) -> &ForgeQueryIntentEligibilityTraceEvidence {
        &self.inner.eligibility_trace
    }

    pub fn decision_digest(&self) -> &str {
        &self.inner.decision_digest
    }
}

use crate::runtime::{
    ForgeQueryBatchWriteReceipt, ForgeQueryRuntime, ForgeQueryRuntimeError, ForgeQueryWriteCommand,
};

use super::super::{
    ForgeQueryAuthoritativeMutationBatchExecutionHandoff, ForgeQueryIntentAdmissionDecision,
};
use super::ForgeQueryRuntimeIntentAdmissionReviewData;

pub struct ForgeQueryRuntimeWriteBatchIntentAuthoring<'a> {
    runtime: &'a mut ForgeQueryRuntime,
    commands: Vec<ForgeQueryWriteCommand>,
}

pub struct ForgeQueryRuntimeWriteBatchIntentAdmissionReview<'a> {
    runtime: &'a mut ForgeQueryRuntime,
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
}

pub struct ForgeQueryAdmittedRuntimeWriteBatchIntent<'a> {
    runtime: &'a mut ForgeQueryRuntime,
    handoff: ForgeQueryAuthoritativeMutationBatchExecutionHandoff,
}

impl<'a> ForgeQueryRuntimeWriteBatchIntentAuthoring<'a> {
    pub(crate) fn new(
        runtime: &'a mut ForgeQueryRuntime,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Self {
        Self { runtime, commands }
    }

    pub fn review(
        self,
    ) -> Result<ForgeQueryRuntimeWriteBatchIntentAdmissionReview<'a>, ForgeQueryRuntimeError> {
        let review = self
            .runtime
            .review_authoritative_runtime_write_batch(self.commands)?;
        Ok(ForgeQueryRuntimeWriteBatchIntentAdmissionReview {
            runtime: self.runtime,
            review,
        })
    }

    pub fn execute(self) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        self.review()?.admit()?.execute()
    }
}

impl<'a> ForgeQueryRuntimeWriteBatchIntentAdmissionReview<'a> {
    pub fn request(&self) -> &super::super::ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }

    pub fn eligibility(&self) -> &super::super::ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }

    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }

    pub fn decision_trace_envelope(
        &self,
    ) -> Option<&super::super::ForgeQueryIntentDecisionTraceEnvelope> {
        self.review.decision_trace_envelope()
    }

    pub fn admit(
        self,
    ) -> Result<ForgeQueryAdmittedRuntimeWriteBatchIntent<'a>, ForgeQueryRuntimeError> {
        let handoff = self
            .runtime
            .resolve_reviewed_admitted_authoritative_write_batch_handoff(self.review)?;
        Ok(ForgeQueryAdmittedRuntimeWriteBatchIntent {
            runtime: self.runtime,
            handoff,
        })
    }
}

impl<'a> ForgeQueryAdmittedRuntimeWriteBatchIntent<'a> {
    pub fn handoff(&self) -> &ForgeQueryAuthoritativeMutationBatchExecutionHandoff {
        &self.handoff
    }

    pub fn execute(self) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        let binding = self
            .runtime
            .prepare_authoritative_mutation_batch_execution_binding(self.handoff);
        self.runtime
            .execute_authoritative_mutation_batch_execution_binding(binding)
    }
}

use super::*;
use crate::intent_admission::dx::{
    non_admitted_runtime_violation, ForgeQueryRuntimeIntentAdmissionReviewData,
};
use crate::intent_admission::{
    ForgeQueryAuthoritativeMutationExecutionBinding,
    ForgeQueryAuthoritativeMutationExecutionHandoff, ForgeQueryAuthoritativeMutationPreflight,
    ForgeQueryIntentAdmissionDecision,
};
use crate::runtime::runtime_writes::ForgeQueryWriteAdmissionExecutionRecord;

impl ForgeQueryRuntime {
    pub fn write_intent(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> crate::intent_admission::ForgeQueryRuntimeWriteIntentAuthoring<'_> {
        crate::intent_admission::ForgeQueryRuntimeWriteIntentAuthoring::new(self, command)
    }

    pub(crate) fn review_authoritative_runtime_write(
        &self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryRuntimeIntentAdmissionReviewData, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Write)?;
        let seed = self.build_authoritative_mutation_intent_seed(command);
        let request = crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::authoritative_write_entrypoint(seed)
            .map_err(|violation| {
                ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(violation.message()))
            })?;
        Ok(ForgeQueryRuntimeIntentAdmissionReviewData::from_request(
            request,
        ))
    }

    pub(crate) fn resolve_reviewed_admitted_authoritative_write_handoff(
        &self,
        review: ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> Result<ForgeQueryAuthoritativeMutationExecutionHandoff, ForgeQueryRuntimeError> {
        match review.decision().clone() {
            ForgeQueryIntentAdmissionDecision::Admitted(
                crate::intent_admission::ForgeQueryAdmittedIntentPlan::AuthoritativeMutation(plan),
            ) => {
                let handoff = ForgeQueryAuthoritativeMutationExecutionHandoff::from_plan(plan);
                let obligation_dispatch =
                    self.authoritative_mutation_obligation_dispatch(&handoff)?;
                Ok(handoff.with_obligation_dispatch(obligation_dispatch))
            }
            ForgeQueryIntentAdmissionDecision::Admitted(_) => {
                Err(self.authoritative_write_non_admitted_error(&review))
            }
            ForgeQueryIntentAdmissionDecision::Advisory(_)
            | ForgeQueryIntentAdmissionDecision::Violation(_) => {
                Err(self.authoritative_write_non_admitted_error(&review))
            }
        }
    }

    pub(crate) fn resolve_reviewed_admitted_authoritative_write_handoff_with_policy_context(
        &self,
        review: ForgeQueryRuntimeIntentAdmissionReviewData,
        policy_context: &crate::policy_basis::AdmittedPolicyTenantContext,
    ) -> Result<ForgeQueryAuthoritativeMutationExecutionHandoff, ForgeQueryRuntimeError> {
        match review.decision().clone() {
            ForgeQueryIntentAdmissionDecision::Admitted(
                crate::intent_admission::ForgeQueryAdmittedIntentPlan::AuthoritativeMutation(plan),
            ) => {
                let handoff = ForgeQueryAuthoritativeMutationExecutionHandoff::from_plan(plan);
                let obligation_dispatch = self
                    .authoritative_mutation_obligation_dispatch_with_policy_context(
                        &handoff,
                        policy_context,
                    )?;
                Ok(handoff.with_obligation_dispatch(obligation_dispatch))
            }
            ForgeQueryIntentAdmissionDecision::Admitted(_) => {
                Err(self.authoritative_write_non_admitted_error(&review))
            }
            ForgeQueryIntentAdmissionDecision::Advisory(_)
            | ForgeQueryIntentAdmissionDecision::Violation(_) => {
                Err(self.authoritative_write_non_admitted_error(&review))
            }
        }
    }

    pub(crate) fn authoritative_write_non_admitted_error(
        &self,
        review: &ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> ForgeQueryRuntimeError {
        let seed = review
            .request()
            .authoritative_mutation_seed()
            .expect("scalar write review must preserve mutation seed");
        match seed.preflight() {
            ForgeQueryAuthoritativeMutationPreflight::Admitted { .. } => {
                let violation = non_admitted_runtime_violation(review);
                ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(
                    violation.message(),
                ))
            }
            ForgeQueryAuthoritativeMutationPreflight::BindingDenied(denial) => {
                ForgeQueryRuntimeError::MutationBindingDenied(denial.clone())
            }
            ForgeQueryAuthoritativeMutationPreflight::AssertionDenied(denial) => {
                ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial.clone())
            }
            ForgeQueryAuthoritativeMutationPreflight::ContinuityDenied(denial) => {
                ForgeQueryRuntimeError::MutationContinuityDenied(denial.clone())
            }
            ForgeQueryAuthoritativeMutationPreflight::NamingDenied(denial) => {
                ForgeQueryRuntimeError::MutationNamingDenied(denial.clone())
            }
            ForgeQueryAuthoritativeMutationPreflight::TargetReferenceDenied(denial) => {
                ForgeQueryRuntimeError::MutationTargetReferenceDenied(denial.clone())
            }
        }
    }

    pub(crate) fn prepare_authoritative_mutation_execution_binding(
        &self,
        handoff: ForgeQueryAuthoritativeMutationExecutionHandoff,
    ) -> ForgeQueryAuthoritativeMutationExecutionBinding {
        ForgeQueryAuthoritativeMutationExecutionBinding::from_handoff(handoff)
    }

    pub(crate) fn execute_authoritative_mutation_execution_binding(
        &mut self,
        binding: ForgeQueryAuthoritativeMutationExecutionBinding,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Write)?;
        let handoff = binding.handoff().clone();
        let shared_admission = ForgeQueryWriteAdmissionExecutionRecord {
            family: binding.family(),
            entrypoint: binding.entrypoint(),
            execution_seam: binding.execution_seam(),
            request_detail: review_request_detail(handoff.command()),
            request_digest: handoff.request_digest().to_string(),
            eligibility_trace: handoff.eligibility_trace().clone(),
            decision_digest: handoff.decision_digest().to_string(),
            handoff_digest: handoff.handoff_digest().to_string(),
            binding_digest: binding.binding_digest().to_string(),
            obligation_dispatch: binding.obligation_dispatch().cloned(),
        };
        self.execute_authoritative_write_command_direct(
            handoff.command().clone(),
            handoff.verified_existing_truth_assertion().cloned(),
            Some(shared_admission),
        )
    }
}

fn review_request_detail(command: &ForgeQueryWriteCommand) -> String {
    match command.declared_entity_identity_ref() {
        Some(identity) => identity
            .evidence_identity()
            .reporting_projection()
            .to_string(),
        None => command
            .declared_collection_ref()
            .unwrap_or("scalar-write")
            .to_string(),
    }
}

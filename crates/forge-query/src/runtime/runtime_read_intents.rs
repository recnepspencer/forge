use super::read_composition_runtime::{
    execute_runtime_basis_context_read_graph, execute_runtime_current_read_graph,
};
use super::runtime_read_execution_receipts::{
    attach_graph_obligation_dispatch, attach_graph_read_access_receipt,
    attach_read_intent_execution_evidence, provision_graph_indexes_for_read_binding,
};
use super::*;
use crate::intent_admission::dx::{
    non_admitted_runtime_violation, ForgeQueryRuntimeIntentAdmissionReviewData,
};
use crate::intent_admission::{
    ForgeQueryIntentAdmissionDecision, ForgeQueryReadExecutionBinding,
    ForgeQueryReadExecutionHandoff, ForgeQueryReadExecutionIntentSeed,
};
use crate::query_context::AdmittedQueryBasisContext;

impl ForgeQueryWorkspace {
    pub fn read_family_intent(
        &mut self,
        family: &ForgeQueryReadFamily,
    ) -> crate::intent_admission::ForgeQueryWorkspaceReadIntentAuthoring<'_> {
        crate::intent_admission::ForgeQueryWorkspaceReadIntentAuthoring::new(
            self,
            family.clone(),
            None,
            None,
        )
    }

    pub fn read_family_in_basis_context_intent(
        &mut self,
        family: &ForgeQueryReadFamily,
        context: &AdmittedQueryBasisContext,
    ) -> crate::intent_admission::ForgeQueryWorkspaceReadIntentAuthoring<'_> {
        crate::intent_admission::ForgeQueryWorkspaceReadIntentAuthoring::new(
            self,
            family.clone(),
            Some(context.clone()),
            None,
        )
    }

    pub fn read_family_intent_in_graph_read_authority(
        &mut self,
        family: &ForgeQueryReadFamily,
        authority: &ForgeQueryGraphReadAccessAuthorityContext,
    ) -> crate::intent_admission::ForgeQueryWorkspaceReadIntentAuthoring<'_> {
        crate::intent_admission::ForgeQueryWorkspaceReadIntentAuthoring::new(
            self,
            family.clone(),
            None,
            Some(authority.clone()),
        )
    }

    pub(crate) fn review_read_execution(
        &self,
        family: ForgeQueryReadFamily,
        basis_context: Option<AdmittedQueryBasisContext>,
    ) -> Result<ForgeQueryRuntimeIntentAdmissionReviewData, ForgeQueryRuntimeError> {
        self.runtime
            .review_runtime_read_execution(family, basis_context)
    }

    pub(crate) fn resolve_reviewed_admitted_read_execution_handoff(
        &self,
        review: ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> Result<ForgeQueryReadExecutionHandoff, ForgeQueryRuntimeError> {
        self.runtime
            .resolve_reviewed_admitted_read_execution_handoff(review)
    }

    pub(crate) fn read_execution_non_admitted_error(
        &self,
        review: &ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> ForgeQueryRuntimeError {
        self.runtime.read_execution_non_admitted_error(review)
    }

    pub(crate) fn into_runtime_read_execution_binding(
        &self,
        handoff: ForgeQueryReadExecutionHandoff,
    ) -> Result<ForgeQueryReadExecutionBinding, ForgeQueryRuntimeError> {
        self.runtime.prepare_read_execution_binding(handoff, None)
    }

    pub(crate) fn into_runtime_read_execution_binding_in_authority(
        &self,
        handoff: ForgeQueryReadExecutionHandoff,
        authority: Option<&ForgeQueryGraphReadAccessAuthorityContext>,
    ) -> Result<ForgeQueryReadExecutionBinding, ForgeQueryRuntimeError> {
        self.runtime
            .prepare_read_execution_binding(handoff, authority)
    }

    pub(crate) fn into_runtime_read_execution_binding_with_access_plan(
        &self,
        handoff: ForgeQueryReadExecutionHandoff,
        graph_read_access_plan: ForgeQueryAdmittedGraphReadAccessPlan,
    ) -> Result<ForgeQueryReadExecutionBinding, ForgeQueryRuntimeError> {
        self.runtime
            .prepare_read_execution_binding_with_access_plan(handoff, graph_read_access_plan)
    }

    pub(crate) fn execute_bound_read_execution(
        &mut self,
        binding: ForgeQueryReadExecutionBinding,
    ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.runtime.execute_read_execution_binding(binding)
    }
}

impl ForgeQueryRuntime {
    pub(crate) fn review_runtime_read_execution(
        &self,
        family: ForgeQueryReadFamily,
        basis_context: Option<AdmittedQueryBasisContext>,
    ) -> Result<ForgeQueryRuntimeIntentAdmissionReviewData, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Read)?;
        let request = match basis_context {
            Some(context) => {
                let seed = ForgeQueryReadExecutionIntentSeed::in_basis_context(family, context);
                crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::read_family_in_basis_context_entrypoint(seed)
            }
            None => {
                let seed = ForgeQueryReadExecutionIntentSeed::current_runtime(family);
                crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::read_family_entrypoint(seed)
            }
        }
        .map_err(|violation| ForgeQueryRuntimeError::ReadCompositionDenied(
            ForgeQueryReadDenial::new(ForgeQueryReadDenialKind::AuthoringDenied, violation.message()),
        ))?;
        Ok(ForgeQueryRuntimeIntentAdmissionReviewData::from_request(
            request,
        ))
    }

    pub(crate) fn resolve_reviewed_admitted_read_execution_handoff(
        &self,
        review: ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> Result<ForgeQueryReadExecutionHandoff, ForgeQueryRuntimeError> {
        match review.decision().clone() {
            ForgeQueryIntentAdmissionDecision::Admitted(
                crate::intent_admission::ForgeQueryAdmittedIntentPlan::ReadExecution(plan),
            ) => Ok(ForgeQueryReadExecutionHandoff::from_plan(plan)),
            ForgeQueryIntentAdmissionDecision::Admitted(_)
            | ForgeQueryIntentAdmissionDecision::Advisory(_)
            | ForgeQueryIntentAdmissionDecision::Violation(_) => {
                Err(self.read_execution_non_admitted_error(&review))
            }
        }
    }

    pub(crate) fn read_execution_non_admitted_error(
        &self,
        review: &ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> ForgeQueryRuntimeError {
        let violation = non_admitted_runtime_violation(review);
        ForgeQueryRuntimeError::ReadCompositionDenied(ForgeQueryReadDenial::new(
            ForgeQueryReadDenialKind::BasisPreflightDenied,
            violation.message(),
        ))
    }

    pub(crate) fn prepare_read_execution_binding(
        &self,
        handoff: ForgeQueryReadExecutionHandoff,
        graph_read_authority: Option<&ForgeQueryGraphReadAccessAuthorityContext>,
    ) -> Result<ForgeQueryReadExecutionBinding, ForgeQueryRuntimeError> {
        let graph_obligation_dispatch = self.read_family_obligation_dispatch(&handoff)?;
        let graph_read_access_plan =
            self.admit_graph_read_access_plan_for_handoff(&handoff, graph_read_authority)?;
        Ok(ForgeQueryReadExecutionBinding::from_handoff(
            handoff,
            graph_obligation_dispatch,
            graph_read_access_plan,
        ))
    }

    pub(crate) fn prepare_read_execution_binding_with_access_plan(
        &self,
        handoff: ForgeQueryReadExecutionHandoff,
        graph_read_access_plan: ForgeQueryAdmittedGraphReadAccessPlan,
    ) -> Result<ForgeQueryReadExecutionBinding, ForgeQueryRuntimeError> {
        validate_graph_read_access_plan_matches_handoff(&handoff, &graph_read_access_plan)?;
        let graph_obligation_dispatch = self.read_family_obligation_dispatch(&handoff)?;
        Ok(ForgeQueryReadExecutionBinding::from_handoff(
            handoff,
            graph_obligation_dispatch,
            graph_read_access_plan,
        ))
    }

    pub(crate) fn execute_read_execution_binding(
        &mut self,
        binding: ForgeQueryReadExecutionBinding,
    ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Read)?;
        let snapshot_identity = self.current_snapshot_identity().evidence_identity();
        let ephemeral_graph_index_receipt =
            provision_graph_indexes_for_read_binding(&binding, snapshot_identity.as_str())?;
        let mut executed_read = self.execute_graph_read_binding(&binding)?;
        executed_read.record_ephemeral_index_receipt(ephemeral_graph_index_receipt.as_ref());

        attach_graph_read_access_receipt(
            &mut executed_read,
            &binding,
            snapshot_identity.as_str(),
            ephemeral_graph_index_receipt,
        );
        attach_graph_obligation_dispatch(&mut executed_read, &binding);
        attach_read_intent_execution_evidence(&mut executed_read, &binding, &snapshot_identity);

        Ok(executed_read.into_result())
    }

    fn execute_graph_read_binding(
        &mut self,
        binding: &ForgeQueryReadExecutionBinding,
    ) -> Result<super::read_composition_runtime::ForgeQueryExecutedReadGraph, ForgeQueryRuntimeError>
    {
        match binding.basis_context() {
            Some(context) => execute_runtime_basis_context_read_graph(
                self,
                binding.read_family().read_graph(),
                context,
            ),
            None => execute_runtime_current_read_graph(self, binding.read_family().read_graph()),
        }
        .map_err(ForgeQueryRuntimeError::ReadCompositionDenied)
    }
}

fn validate_graph_read_access_plan_matches_handoff(
    handoff: &ForgeQueryReadExecutionHandoff,
    graph_read_access_plan: &ForgeQueryAdmittedGraphReadAccessPlan,
) -> Result<(), ForgeQueryRuntimeError> {
    let planned_read_graph_digest = graph_read_access_plan
        .admission()
        .requirement_set()
        .read_graph_digest();
    let handoff_read_graph_digest = handoff.read_family().read_graph().digest();
    if planned_read_graph_digest == handoff_read_graph_digest {
        return Ok(());
    }
    Err(ForgeQueryRuntimeError::ReadCompositionDenied(
        ForgeQueryReadDenial::new(
            ForgeQueryReadDenialKind::BasisPreflightDenied,
            format!(
                "graph read access plan was admitted for read graph `{planned_read_graph_digest}` but execution handoff carries `{handoff_read_graph_digest}`"
            ),
        )
        .with_access_plan_binding_mismatch(ForgeQueryReadAccessPlanBindingMismatch::new(
            planned_read_graph_digest,
            handoff_read_graph_digest,
            graph_read_access_plan.digest(),
            graph_read_access_plan.admission().digest(),
        )),
    ))
}

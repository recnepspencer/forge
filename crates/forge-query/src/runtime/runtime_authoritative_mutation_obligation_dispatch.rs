use super::*;

impl ForgeQueryRuntime {
    pub(crate) fn authoritative_mutation_obligation_dispatch(
        &self,
        handoff: &crate::intent_admission::ForgeQueryAuthoritativeMutationExecutionHandoff,
    ) -> Result<Option<ForgeQueryAuthoritativeMutationObligationDispatch>, ForgeQueryRuntimeError>
    {
        let touch_descriptor = self.admit_authoritative_mutation_touch_descriptor(
            ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
                &ForgeQueryGraphCompositionProgram::empty(),
                &ForgeQueryGraphCompositionBreadth::empty(),
                std::slice::from_ref(handoff.command()),
            ),
        )?;
        let operating_world =
            ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority();
        let context = ForgeQueryGraphObligationDispatchContext::scalar_mutation(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )
        .map_err(graph_obligation_dispatch_error)?;
        self.dispatch_authoritative_mutation_obligations(touch_descriptor, operating_world, context)
    }

    pub(crate) fn authoritative_mutation_batch_obligation_dispatch(
        &self,
        handoff: &crate::intent_admission::ForgeQueryAuthoritativeMutationBatchExecutionHandoff,
    ) -> Result<Option<ForgeQueryAuthoritativeMutationObligationDispatch>, ForgeQueryRuntimeError>
    {
        let touch_descriptor =
            self.admit_authoritative_mutation_touch_descriptor(handoff.graph_touch_descriptor())?;
        let operating_world =
            ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority();
        let context = authoritative_mutation_dispatch_context(
            handoff,
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )?;
        self.dispatch_authoritative_mutation_obligations(touch_descriptor, operating_world, context)
    }

    pub(crate) fn authoritative_mutation_batch_obligation_dispatch_with_execution_context(
        &self,
        handoff: &crate::intent_admission::ForgeQueryAuthoritativeMutationBatchExecutionHandoff,
        execution_context: ForgeQueryGraphObligationExecutionContext,
    ) -> Result<Option<ForgeQueryAuthoritativeMutationObligationDispatch>, ForgeQueryRuntimeError>
    {
        let touch_descriptor =
            self.admit_authoritative_mutation_touch_descriptor(handoff.graph_touch_descriptor())?;
        let operating_world =
            ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority();
        let context = authoritative_mutation_dispatch_context(
            handoff,
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )?;
        self.dispatch_authoritative_mutation_obligations_with_execution_context(
            touch_descriptor,
            operating_world,
            context,
            execution_context,
        )
    }

    pub(crate) fn authoritative_mutation_batch_obligation_dispatch_with_policy_context(
        &self,
        handoff: &crate::intent_admission::ForgeQueryAuthoritativeMutationBatchExecutionHandoff,
        policy_context: &crate::policy_basis::AdmittedPolicyTenantContext,
    ) -> Result<Option<ForgeQueryAuthoritativeMutationObligationDispatch>, ForgeQueryRuntimeError>
    {
        admit_graph_mutation_policy_context(policy_context)?;
        let touch_descriptor =
            self.admit_authoritative_mutation_touch_descriptor(handoff.graph_touch_descriptor())?;
        let operating_world =
            ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle();
        let context = authoritative_mutation_dispatch_context(
            handoff,
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )?;
        self.dispatch_authoritative_mutation_obligations_with_policy_context(
            touch_descriptor,
            operating_world,
            context,
            policy_context,
        )
    }

    pub(crate) fn authoritative_mutation_obligation_dispatch_with_policy_context(
        &self,
        handoff: &crate::intent_admission::ForgeQueryAuthoritativeMutationExecutionHandoff,
        policy_context: &crate::policy_basis::AdmittedPolicyTenantContext,
    ) -> Result<Option<ForgeQueryAuthoritativeMutationObligationDispatch>, ForgeQueryRuntimeError>
    {
        admit_graph_mutation_policy_context(policy_context)?;
        let touch_descriptor = self.admit_authoritative_mutation_touch_descriptor(
            ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
                &ForgeQueryGraphCompositionProgram::empty(),
                &ForgeQueryGraphCompositionBreadth::empty(),
                std::slice::from_ref(handoff.command()),
            ),
        )?;
        let operating_world =
            ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle();
        let context = ForgeQueryGraphObligationDispatchContext::scalar_mutation(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )
        .map_err(graph_obligation_dispatch_error)?;
        self.dispatch_authoritative_mutation_obligations_with_policy_context(
            touch_descriptor,
            operating_world,
            context,
            policy_context,
        )
    }

    fn dispatch_authoritative_mutation_obligations(
        &self,
        touch_descriptor: ForgeQueryGraphTouchDescriptor,
        operating_world: ForgeQueryGraphObligationOperatingWorldDescriptor,
        context: ForgeQueryGraphObligationDispatchContext,
    ) -> Result<Option<ForgeQueryAuthoritativeMutationObligationDispatch>, ForgeQueryRuntimeError>
    {
        let selection =
            self.select_graph_obligations_for_touch(&touch_descriptor, &operating_world);
        let dispatch =
            ForgeQueryAuthoritativeMutationObligationDispatch::from_selection(context, selection)
                .map_err(graph_obligation_dispatch_error)?;
        deny_blocking_graph_obligation_dispatch(&dispatch)?;
        Ok(Some(dispatch))
    }

    fn dispatch_authoritative_mutation_obligations_with_execution_context(
        &self,
        touch_descriptor: ForgeQueryGraphTouchDescriptor,
        operating_world: ForgeQueryGraphObligationOperatingWorldDescriptor,
        context: ForgeQueryGraphObligationDispatchContext,
        execution_context: ForgeQueryGraphObligationExecutionContext,
    ) -> Result<Option<ForgeQueryAuthoritativeMutationObligationDispatch>, ForgeQueryRuntimeError>
    {
        let selection =
            self.select_graph_obligations_for_touch(&touch_descriptor, &operating_world);
        let dispatch =
            ForgeQueryAuthoritativeMutationObligationDispatch::from_selection_with_execution_context(
                context,
                selection,
                execution_context,
            )
            .map_err(graph_obligation_dispatch_error)?;
        deny_blocking_graph_obligation_dispatch(&dispatch)?;
        Ok(Some(dispatch))
    }

    fn dispatch_authoritative_mutation_obligations_with_policy_context(
        &self,
        touch_descriptor: ForgeQueryGraphTouchDescriptor,
        operating_world: ForgeQueryGraphObligationOperatingWorldDescriptor,
        context: ForgeQueryGraphObligationDispatchContext,
        policy_context: &crate::policy_basis::AdmittedPolicyTenantContext,
    ) -> Result<Option<ForgeQueryAuthoritativeMutationObligationDispatch>, ForgeQueryRuntimeError>
    {
        let selection =
            self.select_graph_obligations_for_touch(&touch_descriptor, &operating_world);
        let dispatch =
            ForgeQueryAuthoritativeMutationObligationDispatch::from_selection(context, selection)
                .map_err(graph_obligation_dispatch_error)?;
        let dispatch =
            attach_selected_policy_gate_evidence(dispatch, &operating_world, policy_context);
        deny_blocking_graph_obligation_dispatch(&dispatch)?;
        deny_rejected_graph_mutation_policy_gate(&dispatch)?;
        Ok(Some(dispatch))
    }

    fn admit_authoritative_mutation_touch_descriptor(
        &self,
        descriptor: Result<ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial>,
    ) -> Result<ForgeQueryGraphTouchDescriptor, ForgeQueryRuntimeError> {
        descriptor.map_err(graph_obligation_touch_descriptor_error)
    }
}

fn authoritative_mutation_dispatch_context(
    handoff: &crate::intent_admission::ForgeQueryAuthoritativeMutationBatchExecutionHandoff,
    touch_descriptor_digest: &str,
    operating_world_digest: &str,
) -> Result<ForgeQueryGraphObligationDispatchContext, ForgeQueryRuntimeError> {
    let context = if handoff.graph_composition_program().is_empty() {
        ForgeQueryGraphObligationDispatchContext::authoritative_command_batch(
            touch_descriptor_digest,
            operating_world_digest,
        )
    } else {
        ForgeQueryGraphObligationDispatchContext::graph_composition(
            touch_descriptor_digest,
            operating_world_digest,
        )
    };
    context.map_err(graph_obligation_dispatch_error)
}

fn graph_obligation_dispatch_error(
    error: ForgeQueryGraphObligationDispatchError,
) -> ForgeQueryRuntimeError {
    ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(error.to_string()))
}

fn graph_obligation_touch_descriptor_error(
    error: ForgeQueryGraphTouchDescriptorDenial,
) -> ForgeQueryRuntimeError {
    ForgeQueryRuntimeError::GraphObligationTouchDescriptorDenied(error)
}

fn selection_contains_operating_context_gate(
    selection: &ForgeQueryGraphObligationSelection,
) -> bool {
    selection
        .matched_registrations()
        .iter()
        .any(|registration| {
            registration.kind() == ForgeQueryGraphObligationKind::OperatingContextGate
        })
}

fn attach_selected_policy_gate_evidence(
    dispatch: ForgeQueryAuthoritativeMutationObligationDispatch,
    operating_world: &ForgeQueryGraphObligationOperatingWorldDescriptor,
    policy_context: &crate::policy_basis::AdmittedPolicyTenantContext,
) -> ForgeQueryAuthoritativeMutationObligationDispatch {
    if !selection_contains_operating_context_gate(dispatch.selection()) {
        return dispatch;
    }
    let policy_gate =
        ForgeQueryGraphMutationPolicyGateEvidence::from_admitted_context_and_selection(
            policy_context,
            operating_world,
            dispatch.selection(),
        );
    dispatch.with_policy_gate(policy_gate)
}

fn deny_blocking_graph_obligation_dispatch(
    dispatch: &ForgeQueryAuthoritativeMutationObligationDispatch,
) -> Result<(), ForgeQueryRuntimeError> {
    if let Some(denial) = crate::runtime::ForgeQueryGraphObligationDenial::from_dispatch(dispatch) {
        return Err(ForgeQueryRuntimeError::GraphObligationDenied(denial));
    }
    Ok(())
}

fn deny_rejected_graph_mutation_policy_gate(
    dispatch: &ForgeQueryAuthoritativeMutationObligationDispatch,
) -> Result<(), ForgeQueryRuntimeError> {
    if let Some(policy_gate) = dispatch.policy_gate() {
        if policy_gate.verdict() == ForgeQueryGraphMutationPolicyGateVerdict::Deny {
            return Err(ForgeQueryRuntimeError::GraphMutationPolicyGateDenied(
                policy_gate.clone(),
            ));
        }
    }
    Ok(())
}

fn admit_graph_mutation_policy_context(
    policy_context: &crate::policy_basis::AdmittedPolicyTenantContext,
) -> Result<(), ForgeQueryRuntimeError> {
    let expected = crate::policy_basis::PolicyExecutionModeRequest::GraphMutation;
    let actual = policy_context.bundle().execution_mode();
    if actual == expected {
        return Ok(());
    }
    Err(ForgeQueryRuntimeError::GraphMutationPolicyContextDenied {
        expected,
        actual,
        policy_tenant_admission_digest: policy_context.bundle().digest().as_str().to_string(),
    })
}

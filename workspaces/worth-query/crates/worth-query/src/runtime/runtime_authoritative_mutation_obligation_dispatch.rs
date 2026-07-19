use super::*;

impl WorthQueryRuntime {
    pub(crate) fn authoritative_mutation_obligation_dispatch(
        &self,
        handoff: &crate::intent_admission::WorthQueryAuthoritativeMutationExecutionHandoff,
    ) -> Result<Option<WorthQueryAuthoritativeMutationObligationDispatch>, WorthQueryRuntimeError>
    {
        let touch_descriptor = self.admit_authoritative_mutation_touch_descriptor(
            WorthQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
                &WorthQueryGraphCompositionProgram::empty(),
                &WorthQueryGraphCompositionBreadth::empty(),
                std::slice::from_ref(handoff.command()),
            ),
        )?;
        let operating_world =
            WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority();
        let context = WorthQueryGraphObligationDispatchContext::scalar_mutation(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )
        .map_err(graph_obligation_dispatch_error)?;
        self.dispatch_authoritative_mutation_obligations(touch_descriptor, operating_world, context)
    }

    pub(crate) fn authoritative_mutation_batch_obligation_dispatch(
        &self,
        handoff: &crate::intent_admission::WorthQueryAuthoritativeMutationBatchExecutionHandoff,
    ) -> Result<Option<WorthQueryAuthoritativeMutationObligationDispatch>, WorthQueryRuntimeError>
    {
        let touch_descriptor =
            self.admit_authoritative_mutation_touch_descriptor(handoff.graph_touch_descriptor())?;
        let operating_world =
            WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority();
        let context = authoritative_mutation_dispatch_context(
            handoff,
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )?;
        self.dispatch_authoritative_mutation_obligations(touch_descriptor, operating_world, context)
    }

    pub(crate) fn authoritative_mutation_batch_obligation_dispatch_with_execution_context(
        &self,
        handoff: &crate::intent_admission::WorthQueryAuthoritativeMutationBatchExecutionHandoff,
        execution_context: WorthQueryGraphObligationExecutionContext,
    ) -> Result<Option<WorthQueryAuthoritativeMutationObligationDispatch>, WorthQueryRuntimeError>
    {
        let touch_descriptor =
            self.admit_authoritative_mutation_touch_descriptor(handoff.graph_touch_descriptor())?;
        let operating_world =
            WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority();
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
        handoff: &crate::intent_admission::WorthQueryAuthoritativeMutationBatchExecutionHandoff,
        policy_context: &crate::policy_basis::AdmittedPolicyTenantContext,
    ) -> Result<Option<WorthQueryAuthoritativeMutationObligationDispatch>, WorthQueryRuntimeError>
    {
        admit_graph_mutation_policy_context(policy_context)?;
        let touch_descriptor =
            self.admit_authoritative_mutation_touch_descriptor(handoff.graph_touch_descriptor())?;
        let operating_world =
            WorthQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle();
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
        handoff: &crate::intent_admission::WorthQueryAuthoritativeMutationExecutionHandoff,
        policy_context: &crate::policy_basis::AdmittedPolicyTenantContext,
    ) -> Result<Option<WorthQueryAuthoritativeMutationObligationDispatch>, WorthQueryRuntimeError>
    {
        admit_graph_mutation_policy_context(policy_context)?;
        let touch_descriptor = self.admit_authoritative_mutation_touch_descriptor(
            WorthQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
                &WorthQueryGraphCompositionProgram::empty(),
                &WorthQueryGraphCompositionBreadth::empty(),
                std::slice::from_ref(handoff.command()),
            ),
        )?;
        let operating_world =
            WorthQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle();
        let context = WorthQueryGraphObligationDispatchContext::scalar_mutation(
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
        touch_descriptor: WorthQueryGraphTouchDescriptor,
        operating_world: WorthQueryGraphObligationOperatingWorldDescriptor,
        context: WorthQueryGraphObligationDispatchContext,
    ) -> Result<Option<WorthQueryAuthoritativeMutationObligationDispatch>, WorthQueryRuntimeError>
    {
        let selection =
            self.select_graph_obligations_for_touch(&touch_descriptor, &operating_world);
        let dispatch =
            WorthQueryAuthoritativeMutationObligationDispatch::from_selection(context, selection)
                .map_err(graph_obligation_dispatch_error)?;
        deny_blocking_graph_obligation_dispatch(&dispatch)?;
        Ok(Some(dispatch))
    }

    fn dispatch_authoritative_mutation_obligations_with_execution_context(
        &self,
        touch_descriptor: WorthQueryGraphTouchDescriptor,
        operating_world: WorthQueryGraphObligationOperatingWorldDescriptor,
        context: WorthQueryGraphObligationDispatchContext,
        execution_context: WorthQueryGraphObligationExecutionContext,
    ) -> Result<Option<WorthQueryAuthoritativeMutationObligationDispatch>, WorthQueryRuntimeError>
    {
        let selection =
            self.select_graph_obligations_for_touch(&touch_descriptor, &operating_world);
        let dispatch =
            WorthQueryAuthoritativeMutationObligationDispatch::from_selection_with_execution_context(
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
        touch_descriptor: WorthQueryGraphTouchDescriptor,
        operating_world: WorthQueryGraphObligationOperatingWorldDescriptor,
        context: WorthQueryGraphObligationDispatchContext,
        policy_context: &crate::policy_basis::AdmittedPolicyTenantContext,
    ) -> Result<Option<WorthQueryAuthoritativeMutationObligationDispatch>, WorthQueryRuntimeError>
    {
        let selection =
            self.select_graph_obligations_for_touch(&touch_descriptor, &operating_world);
        let dispatch =
            WorthQueryAuthoritativeMutationObligationDispatch::from_selection(context, selection)
                .map_err(graph_obligation_dispatch_error)?;
        let dispatch =
            attach_selected_policy_gate_evidence(dispatch, &operating_world, policy_context);
        deny_blocking_graph_obligation_dispatch(&dispatch)?;
        deny_rejected_graph_mutation_policy_gate(&dispatch)?;
        Ok(Some(dispatch))
    }

    fn admit_authoritative_mutation_touch_descriptor(
        &self,
        descriptor: Result<WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchDescriptorDenial>,
    ) -> Result<WorthQueryGraphTouchDescriptor, WorthQueryRuntimeError> {
        descriptor.map_err(graph_obligation_touch_descriptor_error)
    }
}

fn authoritative_mutation_dispatch_context(
    handoff: &crate::intent_admission::WorthQueryAuthoritativeMutationBatchExecutionHandoff,
    touch_descriptor_digest: &str,
    operating_world_digest: &str,
) -> Result<WorthQueryGraphObligationDispatchContext, WorthQueryRuntimeError> {
    let context = if handoff.graph_composition_program().is_empty() {
        WorthQueryGraphObligationDispatchContext::authoritative_command_batch(
            touch_descriptor_digest,
            operating_world_digest,
        )
    } else {
        WorthQueryGraphObligationDispatchContext::graph_composition(
            touch_descriptor_digest,
            operating_world_digest,
        )
    };
    context.map_err(graph_obligation_dispatch_error)
}

fn graph_obligation_dispatch_error(
    error: WorthQueryGraphObligationDispatchError,
) -> WorthQueryRuntimeError {
    WorthQueryRuntimeError::Workspace(WorthQueryWorkspaceError::new(error.to_string()))
}

fn graph_obligation_touch_descriptor_error(
    error: WorthQueryGraphTouchDescriptorDenial,
) -> WorthQueryRuntimeError {
    WorthQueryRuntimeError::GraphObligationTouchDescriptorDenied(error)
}

fn selection_contains_operating_context_gate(
    selection: &WorthQueryGraphObligationSelection,
) -> bool {
    selection
        .matched_registrations()
        .iter()
        .any(|registration| {
            registration.kind() == WorthQueryGraphObligationKind::OperatingContextGate
        })
}

fn attach_selected_policy_gate_evidence(
    dispatch: WorthQueryAuthoritativeMutationObligationDispatch,
    operating_world: &WorthQueryGraphObligationOperatingWorldDescriptor,
    policy_context: &crate::policy_basis::AdmittedPolicyTenantContext,
) -> WorthQueryAuthoritativeMutationObligationDispatch {
    if !selection_contains_operating_context_gate(dispatch.selection()) {
        return dispatch;
    }
    let policy_gate =
        WorthQueryGraphMutationPolicyGateEvidence::from_admitted_context_and_selection(
            policy_context,
            operating_world,
            dispatch.selection(),
        );
    dispatch.with_policy_gate(policy_gate)
}

fn deny_blocking_graph_obligation_dispatch(
    dispatch: &WorthQueryAuthoritativeMutationObligationDispatch,
) -> Result<(), WorthQueryRuntimeError> {
    if let Some(denial) = crate::runtime::WorthQueryGraphObligationDenial::from_dispatch(dispatch) {
        return Err(WorthQueryRuntimeError::GraphObligationDenied(denial));
    }
    Ok(())
}

fn deny_rejected_graph_mutation_policy_gate(
    dispatch: &WorthQueryAuthoritativeMutationObligationDispatch,
) -> Result<(), WorthQueryRuntimeError> {
    if let Some(policy_gate) = dispatch.policy_gate() {
        if policy_gate.verdict() == WorthQueryGraphMutationPolicyGateVerdict::Deny {
            return Err(WorthQueryRuntimeError::GraphMutationPolicyGateDenied(
                policy_gate.clone(),
            ));
        }
    }
    Ok(())
}

fn admit_graph_mutation_policy_context(
    policy_context: &crate::policy_basis::AdmittedPolicyTenantContext,
) -> Result<(), WorthQueryRuntimeError> {
    let expected = crate::policy_basis::PolicyExecutionModeRequest::GraphMutation;
    let actual = policy_context.bundle().execution_mode();
    if actual == expected {
        return Ok(());
    }
    Err(WorthQueryRuntimeError::GraphMutationPolicyContextDenied {
        expected,
        actual,
        policy_tenant_admission_digest: policy_context.bundle().digest().as_str().to_string(),
    })
}

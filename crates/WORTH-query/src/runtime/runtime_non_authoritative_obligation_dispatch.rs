use super::{
    WorthQueryAuthoritativeMutationObligationDispatch, WorthQueryGraphObligationDispatchContext,
    WorthQueryGraphObligationDispatchError, WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchDescriptorDenial,
    WorthQueryIntentDeclaration, WorthQueryRuntime, WorthQueryRuntimeError,
    WorthQueryWorkspaceError, WorthQueryWriteCommand,
};

impl WorthQueryRuntime {
    pub(crate) fn preview_mutation_obligation_dispatch(
        &self,
        command: &WorthQueryWriteCommand,
    ) -> Result<Option<WorthQueryAuthoritativeMutationObligationDispatch>, WorthQueryRuntimeError>
    {
        let touch_descriptor = self.admit_non_authoritative_touch_descriptor(
            WorthQueryGraphTouchDescriptor::from_mutation_command_batch(std::slice::from_ref(
                command,
            )),
        )?;
        let operating_world = WorthQueryGraphObligationOperatingWorldDescriptor::preview();
        let context = WorthQueryGraphObligationDispatchContext::preview_mutation(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )
        .map_err(graph_obligation_dispatch_error)?;
        self.dispatch_non_authoritative_obligations(touch_descriptor, operating_world, context)
    }

    pub(crate) fn preview_mutation_batch_obligation_dispatch(
        &self,
        commands: &[WorthQueryWriteCommand],
    ) -> Result<Option<WorthQueryAuthoritativeMutationObligationDispatch>, WorthQueryRuntimeError>
    {
        let touch_descriptor = self.admit_non_authoritative_touch_descriptor(
            WorthQueryGraphTouchDescriptor::from_mutation_command_batch(commands),
        )?;
        let operating_world = WorthQueryGraphObligationOperatingWorldDescriptor::preview();
        let context = WorthQueryGraphObligationDispatchContext::preview_mutation(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )
        .map_err(graph_obligation_dispatch_error)?;
        self.dispatch_non_authoritative_obligations(touch_descriptor, operating_world, context)
    }

    pub(crate) fn preview_intent_obligation_dispatch(
        &self,
        declaration: &WorthQueryIntentDeclaration,
    ) -> Result<Option<WorthQueryAuthoritativeMutationObligationDispatch>, WorthQueryRuntimeError>
    {
        let Some(touch_descriptor) = declaration.graph_touch_descriptor().cloned() else {
            return self.require_intent_touch_descriptor_when_graph_obligations_exist(declaration);
        };
        let operating_world = WorthQueryGraphObligationOperatingWorldDescriptor::preview();
        let context = WorthQueryGraphObligationDispatchContext::preview_intent(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )
        .map_err(graph_obligation_dispatch_error)?;
        self.dispatch_non_authoritative_obligations(touch_descriptor, operating_world, context)
    }

    pub(crate) fn branch_intent_obligation_dispatch(
        &self,
        declaration: &WorthQueryIntentDeclaration,
    ) -> Result<Option<WorthQueryAuthoritativeMutationObligationDispatch>, WorthQueryRuntimeError>
    {
        let Some(touch_descriptor) = declaration.graph_touch_descriptor().cloned() else {
            return self.require_intent_touch_descriptor_when_graph_obligations_exist(declaration);
        };
        let operating_world = WorthQueryGraphObligationOperatingWorldDescriptor::branch();
        let context = WorthQueryGraphObligationDispatchContext::branch_intent(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )
        .map_err(graph_obligation_dispatch_error)?;
        self.dispatch_non_authoritative_obligations(touch_descriptor, operating_world, context)
    }

    fn dispatch_non_authoritative_obligations(
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
        if let Some(denial) =
            crate::runtime::WorthQueryGraphObligationDenial::from_dispatch(&dispatch)
        {
            return Err(WorthQueryRuntimeError::GraphObligationDenied(denial));
        }
        Ok(Some(dispatch))
    }

    fn admit_non_authoritative_touch_descriptor(
        &self,
        descriptor: Result<WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchDescriptorDenial>,
    ) -> Result<WorthQueryGraphTouchDescriptor, WorthQueryRuntimeError> {
        descriptor.map_err(WorthQueryRuntimeError::GraphObligationTouchDescriptorDenied)
    }

    fn require_intent_touch_descriptor_when_graph_obligations_exist(
        &self,
        declaration: &WorthQueryIntentDeclaration,
    ) -> Result<Option<WorthQueryAuthoritativeMutationObligationDispatch>, WorthQueryRuntimeError>
    {
        if self
            .graph_obligation_registration_catalog()
            .registration_count()
            == 0
        {
            return Ok(None);
        }
        Err(
            WorthQueryRuntimeError::GraphObligationIntentTouchDescriptorMissing {
                intent_name: declaration.name().to_string(),
            },
        )
    }
}

fn graph_obligation_dispatch_error(
    error: WorthQueryGraphObligationDispatchError,
) -> WorthQueryRuntimeError {
    WorthQueryRuntimeError::Workspace(WorthQueryWorkspaceError::new(error.to_string()))
}

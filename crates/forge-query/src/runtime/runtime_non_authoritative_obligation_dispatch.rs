use super::{
    ForgeQueryAuthoritativeMutationObligationDispatch, ForgeQueryGraphObligationDispatchContext,
    ForgeQueryGraphObligationDispatchError, ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial,
    ForgeQueryIntentDeclaration, ForgeQueryRuntime, ForgeQueryRuntimeError,
    ForgeQueryWorkspaceError, ForgeQueryWriteCommand,
};

impl ForgeQueryRuntime {
    pub(crate) fn preview_mutation_obligation_dispatch(
        &self,
        command: &ForgeQueryWriteCommand,
    ) -> Result<Option<ForgeQueryAuthoritativeMutationObligationDispatch>, ForgeQueryRuntimeError>
    {
        let touch_descriptor = self.admit_non_authoritative_touch_descriptor(
            ForgeQueryGraphTouchDescriptor::from_mutation_command_batch(std::slice::from_ref(
                command,
            )),
        )?;
        let operating_world = ForgeQueryGraphObligationOperatingWorldDescriptor::preview();
        let context = ForgeQueryGraphObligationDispatchContext::preview_mutation(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )
        .map_err(graph_obligation_dispatch_error)?;
        self.dispatch_non_authoritative_obligations(touch_descriptor, operating_world, context)
    }

    pub(crate) fn preview_mutation_batch_obligation_dispatch(
        &self,
        commands: &[ForgeQueryWriteCommand],
    ) -> Result<Option<ForgeQueryAuthoritativeMutationObligationDispatch>, ForgeQueryRuntimeError>
    {
        let touch_descriptor = self.admit_non_authoritative_touch_descriptor(
            ForgeQueryGraphTouchDescriptor::from_mutation_command_batch(commands),
        )?;
        let operating_world = ForgeQueryGraphObligationOperatingWorldDescriptor::preview();
        let context = ForgeQueryGraphObligationDispatchContext::preview_mutation(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )
        .map_err(graph_obligation_dispatch_error)?;
        self.dispatch_non_authoritative_obligations(touch_descriptor, operating_world, context)
    }

    pub(crate) fn preview_intent_obligation_dispatch(
        &self,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<Option<ForgeQueryAuthoritativeMutationObligationDispatch>, ForgeQueryRuntimeError>
    {
        let Some(touch_descriptor) = declaration.graph_touch_descriptor().cloned() else {
            return self.require_intent_touch_descriptor_when_graph_obligations_exist(declaration);
        };
        let operating_world = ForgeQueryGraphObligationOperatingWorldDescriptor::preview();
        let context = ForgeQueryGraphObligationDispatchContext::preview_intent(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )
        .map_err(graph_obligation_dispatch_error)?;
        self.dispatch_non_authoritative_obligations(touch_descriptor, operating_world, context)
    }

    pub(crate) fn branch_intent_obligation_dispatch(
        &self,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<Option<ForgeQueryAuthoritativeMutationObligationDispatch>, ForgeQueryRuntimeError>
    {
        let Some(touch_descriptor) = declaration.graph_touch_descriptor().cloned() else {
            return self.require_intent_touch_descriptor_when_graph_obligations_exist(declaration);
        };
        let operating_world = ForgeQueryGraphObligationOperatingWorldDescriptor::branch();
        let context = ForgeQueryGraphObligationDispatchContext::branch_intent(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )
        .map_err(graph_obligation_dispatch_error)?;
        self.dispatch_non_authoritative_obligations(touch_descriptor, operating_world, context)
    }

    fn dispatch_non_authoritative_obligations(
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
        if let Some(denial) =
            crate::runtime::ForgeQueryGraphObligationDenial::from_dispatch(&dispatch)
        {
            return Err(ForgeQueryRuntimeError::GraphObligationDenied(denial));
        }
        Ok(Some(dispatch))
    }

    fn admit_non_authoritative_touch_descriptor(
        &self,
        descriptor: Result<ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial>,
    ) -> Result<ForgeQueryGraphTouchDescriptor, ForgeQueryRuntimeError> {
        descriptor.map_err(ForgeQueryRuntimeError::GraphObligationTouchDescriptorDenied)
    }

    fn require_intent_touch_descriptor_when_graph_obligations_exist(
        &self,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<Option<ForgeQueryAuthoritativeMutationObligationDispatch>, ForgeQueryRuntimeError>
    {
        if self
            .graph_obligation_registration_catalog()
            .registration_count()
            == 0
        {
            return Ok(None);
        }
        Err(
            ForgeQueryRuntimeError::GraphObligationIntentTouchDescriptorMissing {
                intent_name: declaration.name().to_string(),
            },
        )
    }
}

fn graph_obligation_dispatch_error(
    error: ForgeQueryGraphObligationDispatchError,
) -> ForgeQueryRuntimeError {
    ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(error.to_string()))
}

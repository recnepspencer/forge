pub(crate) struct UiIntentInputBasisView<'state> {
    generation: &'state crate::facade::prepared_application_authority::
        WorthUiPreparedApplicationGenerationIdentity,
    publication_frame: worth_ui_host_contract::UiMountedFrameIdentity,
    target: crate::runtime::interaction::UiPresentedInteractionTargetView,
    mounted: &'state crate::mounting::WorthUiMountedSessionState,
    application_facts: &'state super::super::UiIntentApplicationFactState,
}

impl<'state> UiIntentInputBasisView<'state> {
    pub(crate) fn observe(
        interaction: &crate::runtime::interaction::UiSemanticInteraction,
        generation: &'state crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
        mounted: &'state crate::mounting::WorthUiMountedSessionState,
        application_facts: &'state super::super::UiIntentApplicationFactState,
    ) -> Result<Self, super::super::UiIntentPayloadStop> {
        if interaction.generation() != generation {
            return Err(super::super::UiIntentPayloadStop::ApplicationGenerationChanged);
        }
        if mounted.has_active_presentation_attempt() {
            return Err(super::super::UiIntentPayloadStop::PublicationTransitionInFlight);
        }
        crate::runtime::interaction::targeting::require_current_target(
            mounted,
            interaction.target(),
        )
        .map_err(super::super::UiIntentPayloadStop::Targeting)?;
        let publication_frame = mounted
            .view()
            .current_frame()
            .ok_or(super::super::UiIntentPayloadStop::NoCurrentPublication)?;
        Ok(Self {
            generation,
            publication_frame,
            target: interaction.target(),
            mounted,
            application_facts,
        })
    }

    pub(crate) const fn generation(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        self.generation
    }

    pub(crate) fn projection(
        &self,
        slot: worth_ui_query_binding::UiProjectionInputSlot,
    ) -> Option<worth_ui_query_binding::UiProjectionInputFactReference> {
        self.mounted.current_projection_input(slot)
    }

    pub(crate) fn application(
        &self,
        slot: crate::declaration::UiIntentApplicationFactSlot,
    ) -> Option<super::super::UiIntentApplicationInputReference> {
        self.application_facts.input_reference(slot)
    }

    pub(crate) fn seal(
        self,
        interaction: crate::runtime::interaction::UiSemanticInteraction,
        query_inputs: Vec<worth_ui_query_binding::UiProjectionInputFactReference>,
        application_inputs: Vec<super::super::UiIntentApplicationInputReference>,
        owner_revisions: Vec<super::UiIntentInputOwnerRevision>,
        cost: super::UiIntentPayloadProjectionCost,
    ) -> super::UiIntentInputBasis {
        super::UiIntentInputBasis::seal(super::UiIntentInputBasisInput {
            generation: self.generation.clone(),
            publication_frame: self.publication_frame,
            target: self.target,
            interaction,
            query_inputs,
            application_inputs,
            owner_revisions,
            cost,
        })
    }
}

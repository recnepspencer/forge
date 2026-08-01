use crate::capability::{
    UiApplicationEffectDestination, UiIntent, UiIntentDefinition,
    UiIntentDefinitionRegistrationError, UiRuntimeServiceDefinitionDestination,
    UiTransitionDefinitionDestination,
};

use super::{UiIntentProviderRequired, UiIntentWiringSatisfied, WorthUiApplicationBuilder};

impl<ChangeProfileState> WorthUiApplicationBuilder<ChangeProfileState, UiIntentWiringSatisfied> {
    pub fn register_intent_definition<I: UiIntent>(
        mut self,
        definition: UiIntentDefinition<I, UiApplicationEffectDestination>,
    ) -> Result<
        WorthUiApplicationBuilder<ChangeProfileState, UiIntentProviderRequired<I>>,
        UiIntentDefinitionRegistrationError,
    > {
        self.inner = self.inner.register_intent_definition(definition)?;
        Ok(self.transition_intent_wiring(UiIntentProviderRequired { definition }))
    }

    pub fn register_intent_transition_definition<I: UiIntent>(
        mut self,
        definition: UiIntentDefinition<I, UiTransitionDefinitionDestination>,
    ) -> Result<Self, UiIntentDefinitionRegistrationError>
    where
        I::ProductOutcome: crate::capability::UiIntentTransitionOutcome,
    {
        self.inner = self.inner.register_intent_definition(definition)?;
        self.intent_execution_bindings
            .register_transition(definition)
            .expect("accepted transition definition has one fresh execution binding");
        Ok(self)
    }

    pub fn register_unsupported_intent_definition<I: UiIntent>(
        mut self,
        definition: UiIntentDefinition<I, UiRuntimeServiceDefinitionDestination>,
    ) -> Result<Self, UiIntentDefinitionRegistrationError> {
        self.inner = self.inner.register_intent_definition(definition)?;
        self.intent_execution_bindings
            .register_unsupported_service(definition)
            .expect("accepted service definition has one fresh unsupported binding");
        Ok(self)
    }
}

impl<ChangeProfileState, I: UiIntent>
    WorthUiApplicationBuilder<ChangeProfileState, UiIntentProviderRequired<I>>
{
    pub fn register_intent_provider<Provider>(
        mut self,
        provider: Provider,
    ) -> Result<
        WorthUiApplicationBuilder<ChangeProfileState, UiIntentWiringSatisfied>,
        crate::runtime::intent_execution::UiIntentExecutionBindingPreparationDenial,
    >
    where
        Provider: crate::runtime::intent_execution::UiIntentExecutionProvider<I>,
    {
        self.intent_execution_bindings
            .register_application(self.intent_wiring.definition, provider)?;
        Ok(self.transition_intent_wiring(UiIntentWiringSatisfied { _sealed: () }))
    }
}

impl<ChangeProfileState, IntentWiringState>
    WorthUiApplicationBuilder<ChangeProfileState, IntentWiringState>
{
    fn transition_intent_wiring<NextIntentWiringState>(
        self,
        intent_wiring: NextIntentWiringState,
    ) -> WorthUiApplicationBuilder<ChangeProfileState, NextIntentWiringState> {
        WorthUiApplicationBuilder {
            inner: self.inner,
            preparation_source: self.preparation_source,
            host_session_plan: self.host_session_plan,
            visual_inspection_policy: self.visual_inspection_policy,
            graph_world_profile: self.graph_world_profile,
            runtime_instance_basis_admissions: self.runtime_instance_basis_admissions,
            measurement_inspection_evidence: self.measurement_inspection_evidence,
            query_binding_plan: self.query_binding_plan,
            intent_application_facts: self.intent_application_facts,
            intent_execution_bindings: self.intent_execution_bindings,
            change_profile: self.change_profile,
            intent_wiring,
        }
    }
}

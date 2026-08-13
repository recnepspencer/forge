use std::marker::PhantomData;

use crate::application_aftermath::PortableApplicationAftermathContract;

use super::contract_slots::DeclaredExternalEffectSlot;

/// One operation declaration with every statically knowable singleton contract
/// selected exactly once.
///
/// Values are created only by a completed
/// [`ApplicationOperationDefinitionBuilder`](super::ApplicationOperationDefinitionBuilder).
/// This makes absence explicit and makes duplicate external-effect or aftermath
/// selection unrepresentable on the ordinary authoring path.
pub struct ApplicationOperationDefinition<Schema, Operation, Input> {
    pub(super) operation: &'static str,
    pub(super) input_type: &'static str,
    pub(super) external_effect: Option<DeclaredExternalEffectSlot>,
    pub(super) aftermath: Option<PortableApplicationAftermathContract>,
    pub(super) marker: PhantomData<fn(Input) -> (Schema, Operation)>,
}

impl<Schema, Operation, Input> std::fmt::Debug
    for ApplicationOperationDefinition<Schema, Operation, Input>
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ApplicationOperationDefinition")
            .field("operation", &self.operation)
            .field("input_type", &self.input_type)
            .field("has_external_effect", &self.external_effect.is_some())
            .field("has_aftermath", &self.aftermath.is_some())
            .finish()
    }
}

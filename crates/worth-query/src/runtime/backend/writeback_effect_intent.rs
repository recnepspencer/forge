use crate::memory_workspace::WorthQueryWorkspaceError;
use crate::runtime::WorthQueryBackendAdmissibleMutation;
use worth_runtime_bridge::facade::{
    BridgeWritebackEffectClass, BridgeWritebackEffectIntent, BridgeWritebackEffectIntentError,
};

pub(crate) struct WorthQueryBridgeWritebackEffectIntent {
    intent: BridgeWritebackEffectIntent,
}

impl WorthQueryBridgeWritebackEffectIntent {
    pub(crate) fn from_admitted_mutation(
        effect_class: BridgeWritebackEffectClass,
        mutation: &WorthQueryBackendAdmissibleMutation,
    ) -> Result<Self, WorthQueryWorkspaceError> {
        let intent = BridgeWritebackEffectIntent::from_authoritative_patch(
            effect_class,
            mutation.authoritative_patch().clone(),
        )
        .map_err(writeback_intent_error)?;
        Ok(Self { intent })
    }

    pub(crate) fn into_bridge_intent(self) -> BridgeWritebackEffectIntent {
        self.intent
    }
}

fn writeback_intent_error(error: BridgeWritebackEffectIntentError) -> WorthQueryWorkspaceError {
    WorthQueryWorkspaceError::new(format!("{error:?}"))
}

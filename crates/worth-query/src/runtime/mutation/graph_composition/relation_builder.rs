use super::symbols::WorthQueryGraphEntitySymbol;
use crate::memory_workspace::WorthQueryEntityIdentity;
use crate::runtime::mutation::{
    WorthQueryAspectMutationBuilder, WorthQueryAspectTouch, WorthQueryAuthoredAspectValue,
};
use worth_runtime_bridge::facade::RelationalBridgeRecordIdentityKind;

#[derive(Clone, Debug, Default)]
pub struct WorthQueryGraphRelationMutationBuilder {
    inner: WorthQueryAspectMutationBuilder,
}

impl WorthQueryGraphRelationMutationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_aspect(
        mut self,
        aspect_touch: WorthQueryAspectTouch,
        value: WorthQueryAuthoredAspectValue,
    ) -> Self {
        self.inner = self.inner.set_aspect(aspect_touch, value);
        self
    }

    pub fn existing_entity_identity(
        mut self,
        aspect_touch: WorthQueryAspectTouch,
        entity_identity: WorthQueryEntityIdentity,
    ) -> Self {
        self.inner = self.inner.set_aspect(
            aspect_touch,
            WorthQueryAuthoredAspectValue::string(endpoint_identity_label(&entity_identity)),
        );
        self
    }

    pub fn symbolic_entity_identity(
        mut self,
        aspect_touch: WorthQueryAspectTouch,
        symbol: &WorthQueryGraphEntitySymbol,
    ) -> Self {
        self.inner = self
            .inner
            .symbolic_entity_identity(aspect_touch, symbol.reference());
        self
    }

    pub(crate) fn into_inner(self) -> WorthQueryAspectMutationBuilder {
        self.inner
    }
}

fn endpoint_identity_label(identity: &WorthQueryEntityIdentity) -> String {
    let parts = identity
        .relational_record_parts()
        .expect("existing graph relation endpoints must carry relational record authority");
    let kind = match parts.kind() {
        RelationalBridgeRecordIdentityKind::Entity => "entity",
        RelationalBridgeRecordIdentityKind::Relation => "relation",
    };
    format!(
        "{kind}:{}:{}:{}",
        parts.partition_id(),
        parts.local_slot(),
        parts.generation()
    )
}

use super::symbols::ForgeQueryGraphEntitySymbol;
use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::runtime::mutation::{
    ForgeQueryAspectMutationBuilder, ForgeQueryAspectTouch, ForgeQueryAuthoredAspectValue,
};
use forge_runtime_bridge::facade::RelationalBridgeRecordIdentityKind;

#[derive(Clone, Debug, Default)]
pub struct ForgeQueryGraphRelationMutationBuilder {
    inner: ForgeQueryAspectMutationBuilder,
}

impl ForgeQueryGraphRelationMutationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_aspect(
        mut self,
        aspect_touch: ForgeQueryAspectTouch,
        value: ForgeQueryAuthoredAspectValue,
    ) -> Self {
        self.inner = self.inner.set_aspect(aspect_touch, value);
        self
    }

    pub fn existing_entity_identity(
        mut self,
        aspect_touch: ForgeQueryAspectTouch,
        entity_identity: ForgeQueryEntityIdentity,
    ) -> Self {
        self.inner = self.inner.set_aspect(
            aspect_touch,
            ForgeQueryAuthoredAspectValue::string(endpoint_identity_label(&entity_identity)),
        );
        self
    }

    pub fn symbolic_entity_identity(
        mut self,
        aspect_touch: ForgeQueryAspectTouch,
        symbol: &ForgeQueryGraphEntitySymbol,
    ) -> Self {
        self.inner = self
            .inner
            .symbolic_entity_identity(aspect_touch, symbol.reference());
        self
    }

    pub(crate) fn into_inner(self) -> ForgeQueryAspectMutationBuilder {
        self.inner
    }
}

fn endpoint_identity_label(identity: &ForgeQueryEntityIdentity) -> String {
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

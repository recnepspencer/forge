use crate::facade::ForgeStoreBuilder;

use super::ExternalArtifactIntakeCapabilityProof;

#[derive(Debug)]
pub(crate) struct EmbeddedModeConstructionPlan {
    store_builder: ForgeStoreBuilder,
    capability: ExternalArtifactIntakeCapabilityProof,
}

impl EmbeddedModeConstructionPlan {
    pub(crate) fn new(store_builder: ForgeStoreBuilder) -> Self {
        Self {
            store_builder,
            capability: ExternalArtifactIntakeCapabilityProof::issue(),
        }
    }

    pub(crate) fn into_parts(self) -> (ForgeStoreBuilder, ExternalArtifactIntakeCapabilityProof) {
        (self.store_builder, self.capability)
    }
}

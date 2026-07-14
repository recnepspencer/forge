use crate::facade::WORTHStoreBuilder;

use super::ExternalArtifactIntakeCapabilityProof;

#[derive(Debug)]
pub(crate) struct EmbeddedModeConstructionPlan {
    store_builder: WORTHStoreBuilder,
    capability: ExternalArtifactIntakeCapabilityProof,
}

impl EmbeddedModeConstructionPlan {
    pub(crate) fn new(store_builder: WORTHStoreBuilder) -> Self {
        Self {
            store_builder,
            capability: ExternalArtifactIntakeCapabilityProof::issue(),
        }
    }

    pub(crate) fn into_parts(self) -> (WORTHStoreBuilder, ExternalArtifactIntakeCapabilityProof) {
        (self.store_builder, self.capability)
    }
}

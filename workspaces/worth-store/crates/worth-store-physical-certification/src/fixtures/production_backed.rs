use super::{
    FixtureAuthorityReceipt, FixtureMutationBoundary, PersistedStoreFixtureManifest,
    SyntheticFixtureAuthorityDenied,
};
use worth_store_physical_format::InMemoryPhysicalFormatReplayArtifact;

#[derive(Debug, PartialEq, Eq)]
pub struct ProductionBackedPhysicalFixture {
    manifest: PersistedStoreFixtureManifest,
    authority_receipt: FixtureAuthorityReceipt,
    reopened_layout: worth_store_physical_format::PersistedPhysicalLayout,
    replay_artifact: Option<InMemoryPhysicalFormatReplayArtifact>,
}

impl ProductionBackedPhysicalFixture {
    pub(crate) const fn from_manifest_and_receipt(
        manifest: PersistedStoreFixtureManifest,
        authority_receipt: FixtureAuthorityReceipt,
        reopened_layout: worth_store_physical_format::PersistedPhysicalLayout,
        replay_artifact: Option<InMemoryPhysicalFormatReplayArtifact>,
    ) -> Self {
        Self {
            manifest,
            authority_receipt,
            reopened_layout,
            replay_artifact,
        }
    }

    pub const fn manifest(&self) -> &PersistedStoreFixtureManifest {
        &self.manifest
    }

    pub const fn authority_receipt(&self) -> &FixtureAuthorityReceipt {
        &self.authority_receipt
    }

    pub const fn reopened_persisted_layout(
        &self,
    ) -> &worth_store_physical_format::PersistedPhysicalLayout {
        &self.reopened_layout
    }

    pub const fn replay_artifact(&self) -> Option<&InMemoryPhysicalFormatReplayArtifact> {
        self.replay_artifact.as_ref()
    }

    pub fn require_mutation_boundary(
        &self,
        boundary: FixtureMutationBoundary,
    ) -> Result<(), SyntheticFixtureAuthorityDenied> {
        if self.manifest.mutation_boundaries().contains(boundary) {
            Ok(())
        } else {
            Err(SyntheticFixtureAuthorityDenied::UndeclaredMutationBoundary(
                boundary,
            ))
        }
    }
}

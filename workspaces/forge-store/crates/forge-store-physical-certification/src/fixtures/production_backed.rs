use super::{
    FixtureAuthorityReceipt, FixtureMutationBoundary, PersistedStoreFixtureManifest,
    SyntheticFixtureAuthorityDenied,
};

#[derive(Debug, PartialEq, Eq)]
pub struct ProductionBackedPhysicalFixture {
    manifest: PersistedStoreFixtureManifest,
    authority_receipt: FixtureAuthorityReceipt,
}

impl ProductionBackedPhysicalFixture {
    pub(crate) const fn from_manifest_and_receipt(
        manifest: PersistedStoreFixtureManifest,
        authority_receipt: FixtureAuthorityReceipt,
    ) -> Self {
        Self {
            manifest,
            authority_receipt,
        }
    }

    pub const fn manifest(&self) -> &PersistedStoreFixtureManifest {
        &self.manifest
    }

    pub const fn authority_receipt(&self) -> &FixtureAuthorityReceipt {
        &self.authority_receipt
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

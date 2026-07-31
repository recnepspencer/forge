use worth_query_declaration::facade::application_capability::ErasedApplicationCapabilityContract;

use crate::application_schema::WorthQueryInstalledApplicationSchema;

use super::{
    WorthQueryCapabilityCanonicalArtifact, WorthQueryInstalledApplicationCapabilityIdentity,
};

/// Read-only, non-authority input for cold execution-plan compilation.
///
/// The source is obtainable only from an installed application schema. It
/// exposes retained installed meaning without permitting capability lookup,
/// validation, or execution-authority construction.
#[derive(Clone, Copy)]
pub struct WorthQueryInstalledApplicationCapabilityPlanSource<'a> {
    identity: &'a WorthQueryInstalledApplicationCapabilityIdentity,
    authority_identity: &'a str,
    contract: &'a ErasedApplicationCapabilityContract,
    canonical: &'a WorthQueryCapabilityCanonicalArtifact,
}

impl<'a> WorthQueryInstalledApplicationCapabilityPlanSource<'a> {
    pub(super) const fn new(
        identity: &'a WorthQueryInstalledApplicationCapabilityIdentity,
        authority_identity: &'a str,
        contract: &'a ErasedApplicationCapabilityContract,
        canonical: &'a WorthQueryCapabilityCanonicalArtifact,
    ) -> Self {
        Self {
            identity,
            authority_identity,
            contract,
            canonical,
        }
    }

    pub const fn identity(&self) -> &'a WorthQueryInstalledApplicationCapabilityIdentity {
        self.identity
    }

    pub const fn contract(&self) -> &'a ErasedApplicationCapabilityContract {
        self.contract
    }

    pub const fn authority_identity(&self) -> &'a str {
        self.authority_identity
    }

    pub const fn canonical(&self) -> &'a WorthQueryCapabilityCanonicalArtifact {
        self.canonical
    }
}

impl<Schema> WorthQueryInstalledApplicationSchema<Schema> {
    pub fn capability_plan_sources(
        &self,
    ) -> impl ExactSizeIterator<Item = WorthQueryInstalledApplicationCapabilityPlanSource<'_>> {
        self.capability_registry.values().map(|compiled| {
            WorthQueryInstalledApplicationCapabilityPlanSource::new(
                compiled.identity(),
                compiled.authority_identity().as_str(),
                compiled.contract(),
                compiled.canonical(),
            )
        })
    }
}

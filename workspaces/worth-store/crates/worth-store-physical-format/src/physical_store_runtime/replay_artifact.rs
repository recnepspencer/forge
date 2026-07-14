use crate::{
    PersistedPhysicalLayout, PhysicalBootstrapCatalogDenial, PhysicalBootstrapCatalogOpenWitness,
    PhysicalHeaderAuthority, PhysicalStoreIdentity,
};
use worth_store_contracts::AcceptedHandoffReadiness;

use super::{
    operation::reopen, PhysicalStoreRuntime, PhysicalStoreRuntimeDenial,
    PlatformPhysicalOpenRequest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPhysicalReplayArtifact {
    headers: PhysicalHeaderAuthority,
    layout: PersistedPhysicalLayout,
    store_identity: PhysicalStoreIdentity,
}

impl PlatformPhysicalReplayArtifact {
    pub(crate) fn from_persisted_layout(
        headers: PhysicalHeaderAuthority,
        layout: PersistedPhysicalLayout,
        store_identity: PhysicalStoreIdentity,
    ) -> Self {
        Self {
            headers,
            layout,
            store_identity,
        }
    }

    pub fn reopen_physical_format(
        &self,
        readiness: AcceptedHandoffReadiness,
        request: PlatformPhysicalOpenRequest,
    ) -> Result<PhysicalStoreRuntime, PhysicalStoreRuntimeDenial> {
        reopen::reopen_from_verified_layout(
            readiness,
            request,
            self.headers.clone(),
            self.layout.clone(),
            self.store_identity.clone(),
        )
    }

    pub fn admit_bootstrap_open_witness(
        &self,
    ) -> Result<PhysicalBootstrapCatalogOpenWitness, PhysicalBootstrapCatalogDenial> {
        PhysicalBootstrapCatalogOpenWitness::admit_persisted_layout(&self.headers, &self.layout)
    }

    pub const fn persisted_layout(&self) -> &PersistedPhysicalLayout {
        &self.layout
    }

    pub const fn store_identity(&self) -> &PhysicalStoreIdentity {
        &self.store_identity
    }
}

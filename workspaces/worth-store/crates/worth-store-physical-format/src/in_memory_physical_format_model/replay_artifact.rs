use crate::{
    PersistedPhysicalLayout, PhysicalBootstrapCatalogDenial, PhysicalBootstrapCatalogOpenWitness,
    PhysicalHeaderAuthority, PhysicalStoreIdentity,
};
use worth_store_contracts::AcceptedHandoffReadiness;

use super::{
    operation::restore, InMemoryPhysicalFormatModel, InMemoryPhysicalFormatModelDenial,
    InMemoryPhysicalFormatModelRequest,
};

/// Supplied model state for offline reconstruction tests and algorithms.
///
/// It is not filesystem-discovered replay and cannot satisfy Store runtime
/// admission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InMemoryPhysicalFormatReplayArtifact {
    headers: PhysicalHeaderAuthority,
    layout: PersistedPhysicalLayout,
    store_identity: PhysicalStoreIdentity,
}

impl InMemoryPhysicalFormatReplayArtifact {
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

    pub fn restore_model(
        &self,
        readiness: AcceptedHandoffReadiness,
        request: InMemoryPhysicalFormatModelRequest,
    ) -> Result<InMemoryPhysicalFormatModel, InMemoryPhysicalFormatModelDenial> {
        restore::restore_from_verified_layout(
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

use crate::{
    PersistedPhysicalLayout, PhysicalBootstrapCatalogDenial, PhysicalBootstrapCatalogOpenWitness,
    PhysicalHeaderAuthority,
};
use forge_store_contracts::AcceptedHandoffReadiness;

use super::{reopen, PlatformPhysicalFacade, PlatformPhysicalFacadeDenial, PlatformPhysicalOpenRequest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPhysicalReplayArtifact {
    headers: PhysicalHeaderAuthority,
    layout: PersistedPhysicalLayout,
}

impl PlatformPhysicalReplayArtifact {
    pub(crate) fn from_persisted_layout(
        headers: PhysicalHeaderAuthority,
        layout: PersistedPhysicalLayout,
    ) -> Self {
        Self { headers, layout }
    }

    pub fn reopen_s1(
        &self,
        readiness: AcceptedHandoffReadiness,
        request: PlatformPhysicalOpenRequest,
    ) -> Result<PlatformPhysicalFacade, PlatformPhysicalFacadeDenial> {
        reopen::reopen_s1(
            readiness,
            request,
            self.headers.clone(),
            self.layout.clone(),
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
}

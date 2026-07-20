use super::InMemoryPhysicalFormatModel;
use crate::{InMemoryPhysicalFormatModelDenial, PhysicalRootManifestRebuildSource};

impl InMemoryPhysicalFormatModel {
    /// Readmit the current canonical root manifest and bind it to this opened Store instance.
    pub fn root_manifest_rebuild_source(
        &self,
    ) -> Result<PhysicalRootManifestRebuildSource, InMemoryPhysicalFormatModelDenial> {
        let access = crate::access::manifest::root_discovery::canonical_root_manifest(self)?;
        Ok(PhysicalRootManifestRebuildSource::issue(
            access.root().clone(),
            self.store_identity().clone(),
        ))
    }

    /// Issue the current physical allocation inventory for reachability
    /// consumers without widening the index-rebuild capability.
    pub fn current_physical_reachability_source(
        &self,
    ) -> Result<crate::PhysicalCurrentReachabilitySource, InMemoryPhysicalFormatModelDenial> {
        let access = crate::access::manifest::root_discovery::canonical_root_manifest(self)?;
        Ok(crate::PhysicalCurrentReachabilitySource::issue(
            access.root().clone(),
            self.store_identity().clone(),
            self.storage_ref().page_cells(),
            self.storage_ref().extent_cells().to_vec(),
        ))
    }
}

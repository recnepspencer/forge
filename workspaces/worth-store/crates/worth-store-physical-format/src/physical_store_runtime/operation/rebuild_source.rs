use super::PhysicalStoreRuntime;
use crate::{PhysicalRootManifestRebuildSource, PhysicalStoreRuntimeDenial};

impl PhysicalStoreRuntime {
    /// Readmit the current canonical root manifest and bind it to this opened Store instance.
    pub fn root_manifest_rebuild_source(
        &self,
    ) -> Result<PhysicalRootManifestRebuildSource, PhysicalStoreRuntimeDenial> {
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
    ) -> Result<crate::PhysicalCurrentReachabilitySource, PhysicalStoreRuntimeDenial> {
        let access = crate::access::manifest::root_discovery::canonical_root_manifest(self)?;
        Ok(crate::PhysicalCurrentReachabilitySource::issue(
            access.root().clone(),
            self.store_identity().clone(),
            self.storage_ref().page_cells(),
            self.storage_ref().extent_cells().to_vec(),
        ))
    }
}

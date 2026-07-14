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
}

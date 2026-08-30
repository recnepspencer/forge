use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_format::PhysicalRecordFormatDeclaration;

use super::{PhysicalArtifactScope, PhysicalArtifactScopeIdentity};
use crate::localization::PhysicalByteRange;

impl PhysicalArtifactScope {
    pub const fn bootstrap_catalog(
        store: StableStoreIdentity,
        record_format: PhysicalRecordFormatDeclaration,
        range: PhysicalByteRange,
    ) -> Self {
        Self::new(
            store,
            PhysicalArtifactScopeIdentity::BootstrapCatalog(record_format),
            range,
        )
    }
}

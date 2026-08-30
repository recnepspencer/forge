use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_format::{PageGenerationCell, PhysicalRecordFormatDeclaration};

use super::{PhysicalArtifactScope, PhysicalArtifactScopeIdentity};
use crate::localization::PhysicalByteRange;

impl PhysicalArtifactScope {
    pub const fn inline_page(
        store: StableStoreIdentity,
        record_format: PhysicalRecordFormatDeclaration,
        page: PageGenerationCell,
        range: PhysicalByteRange,
    ) -> Self {
        Self::new(
            store,
            PhysicalArtifactScopeIdentity::InlinePage {
                record_format,
                page,
            },
            range,
        )
    }

    pub const fn page_identity(self) -> Option<PageGenerationCell> {
        match self.identity {
            PhysicalArtifactScopeIdentity::InlinePage { page, .. } => Some(page),
            _ => None,
        }
    }
}

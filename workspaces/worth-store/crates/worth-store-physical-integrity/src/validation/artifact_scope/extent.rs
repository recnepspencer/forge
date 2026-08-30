use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_format::{
    DurableExtentRecordPlacement, ExtentChunkCoordinate, PhysicalRecordFormatDeclaration,
};

use super::{PhysicalArtifactScope, PhysicalArtifactScopeIdentity};
use crate::localization::PhysicalByteRange;

impl PhysicalArtifactScope {
    pub const fn extent_manifest(
        store: StableStoreIdentity,
        record_format: PhysicalRecordFormatDeclaration,
        placement: DurableExtentRecordPlacement,
        range: PhysicalByteRange,
    ) -> Self {
        Self::new(
            store,
            PhysicalArtifactScopeIdentity::ExtentManifest {
                record_format,
                placement,
            },
            range,
        )
    }

    pub const fn extent_chunk(
        store: StableStoreIdentity,
        record_format: PhysicalRecordFormatDeclaration,
        coordinate: ExtentChunkCoordinate,
        range: PhysicalByteRange,
    ) -> Self {
        Self::new(
            store,
            PhysicalArtifactScopeIdentity::ExtentChunk {
                record_format,
                coordinate,
            },
            range,
        )
    }

    pub const fn extent_manifest_placement(self) -> Option<DurableExtentRecordPlacement> {
        match self.identity {
            PhysicalArtifactScopeIdentity::ExtentManifest { placement, .. } => Some(placement),
            _ => None,
        }
    }

    pub const fn extent_chunk_coordinate(self) -> Option<ExtentChunkCoordinate> {
        match self.identity {
            PhysicalArtifactScopeIdentity::ExtentChunk { coordinate, .. } => Some(coordinate),
            _ => None,
        }
    }
}

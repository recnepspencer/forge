pub(super) const fn offline_family(
    family: worth_store_physical_format::BackupBundleArtifactFamily,
) -> worth_store_physical_format::OfflinePhysicalArtifactFamily {
    use worth_store_physical_format::{
        BackupBundleArtifactFamily as Bundle, OfflinePhysicalArtifactFamily as Offline,
    };

    match family {
        Bundle::RootManifest | Bundle::CheckpointManifest | Bundle::SecondaryRoot => {
            Offline::Manifest
        }
        Bundle::WalSegment => Offline::Wal,
        Bundle::Page => Offline::Page,
        Bundle::Extent => Offline::Extent,
        Bundle::Index => Offline::Index,
        Bundle::BlobChunk => Offline::BlobChunk,
    }
}

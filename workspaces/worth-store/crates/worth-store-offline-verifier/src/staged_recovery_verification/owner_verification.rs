use sha2::{Digest, Sha256};
use worth_store_physical_format::{BackupBundleArtifactFamily, BackupBundleManifest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StagedRecoveryOwnerVerificationSet {
    physical_integrity: [u8; 32],
    layout_indexes: [u8; 32],
    blob_chunks: [u8; 32],
}

impl StagedRecoveryOwnerVerificationSet {
    pub fn for_manifest(
        manifest: &BackupBundleManifest,
        manifest_digest: [u8; 32],
    ) -> Option<Self> {
        if manifest_digest == [0; 32] {
            return None;
        }
        Some(Self {
            physical_integrity: owner_identity(
                b"worth-store-staged-physical-integrity-verification-v1",
                manifest,
                manifest_digest,
                |_| true,
            ),
            layout_indexes: owner_identity(
                b"worth-store-staged-layout-index-verification-v1",
                manifest,
                manifest_digest,
                |family| family == BackupBundleArtifactFamily::Index,
            ),
            blob_chunks: owner_identity(
                b"worth-store-staged-blob-chunk-verification-v1",
                manifest,
                manifest_digest,
                |family| family == BackupBundleArtifactFamily::BlobChunk,
            ),
        })
    }

    pub const fn physical_integrity(self) -> [u8; 32] {
        self.physical_integrity
    }

    pub const fn layout_indexes(self) -> [u8; 32] {
        self.layout_indexes
    }

    pub const fn blob_chunks(self) -> [u8; 32] {
        self.blob_chunks
    }
}

fn owner_identity(
    domain: &[u8],
    manifest: &BackupBundleManifest,
    manifest_digest: [u8; 32],
    includes: impl Fn(BackupBundleArtifactFamily) -> bool,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update(manifest_digest);
    for row in manifest
        .artifacts()
        .iter()
        .filter(|row| includes(row.family()))
    {
        digest.update((row.output_name().len() as u64).to_be_bytes());
        digest.update(row.output_name().as_bytes());
        digest.update(row.content_digest());
        digest.update(row.bytes().to_be_bytes());
        digest.update(row.generation().to_be_bytes());
    }
    digest.finalize().into()
}

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use super::{
    manifest_decoding::decode_manifest,
    manifest_encoding::{encode_manifest, MAXIMUM_MANIFEST_BYTES},
    DisasterRecoveryBundleDenial, DisasterRecoveryComponent, DisasterRecoverySecurityBinding,
    MaterializedDisasterRecoveryBundle,
};
use crate::{ReplicaRecoveryFrontier, ReplicationLineageIdentity};

pub const DISASTER_RECOVERY_MANIFEST_NAME: &str = "disaster-recovery.manifest";

#[derive(Debug, Clone, Copy)]
pub struct DisasterRecoveryManifestFormat;

impl DisasterRecoveryManifestFormat {
    pub fn open_materialized(
        root: impl Into<PathBuf>,
        maximum_manifest_bytes: usize,
    ) -> Result<MaterializedDisasterRecoveryBundle, DisasterRecoveryBundleDenial> {
        if maximum_manifest_bytes == 0 || maximum_manifest_bytes > MAXIMUM_MANIFEST_BYTES {
            return Err(DisasterRecoveryBundleDenial::ManifestTooLarge);
        }
        let root = std::fs::canonicalize(root.into())
            .map_err(|_| DisasterRecoveryBundleDenial::BundleRootUnavailable)?;
        let bytes = read_bounded_manifest(&root, maximum_manifest_bytes)?;
        let identity = Sha256::digest(&bytes).into();
        let decoded = decode_manifest(&bytes)?;
        MaterializedDisasterRecoveryBundle::from_manifest(
            root,
            decoded.source_lineage,
            decoded.frontier,
            decoded.security,
            decoded.expected_rpo_lsn,
            decoded.components,
            identity,
        )
    }

    pub(super) fn persist(
        root: &Path,
        lineage: &ReplicationLineageIdentity,
        frontier: ReplicaRecoveryFrontier,
        security: DisasterRecoverySecurityBinding,
        expected_rpo_lsn: u64,
        components: &[DisasterRecoveryComponent],
    ) -> Result<[u8; 32], DisasterRecoveryBundleDenial> {
        let bytes = encode_manifest(lineage, frontier, security, expected_rpo_lsn, components)?;
        let identity: [u8; 32] = Sha256::digest(&bytes).into();
        let final_path = root.join(DISASTER_RECOVERY_MANIFEST_NAME);
        if final_path.exists() {
            return existing_manifest_identity(&final_path, &bytes, identity);
        }
        let pending_path = root.join(pending_name(identity));
        if pending_path.exists() {
            require_exact_bytes(&pending_path, &bytes)?;
        } else {
            let mut file = std::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&pending_path)
                .map_err(|_| DisasterRecoveryBundleDenial::ManifestUnavailable)?;
            file.write_all(&bytes)
                .map_err(|_| DisasterRecoveryBundleDenial::ManifestUnavailable)?;
            file.sync_all()
                .map_err(|_| DisasterRecoveryBundleDenial::ManifestUnavailable)?;
        }
        std::fs::rename(&pending_path, &final_path)
            .map_err(|_| DisasterRecoveryBundleDenial::ManifestUnavailable)?;
        sync_directory(root)?;
        Ok(identity)
    }
}

fn read_bounded_manifest(
    root: &Path,
    maximum_manifest_bytes: usize,
) -> Result<Vec<u8>, DisasterRecoveryBundleDenial> {
    let path = root.join(DISASTER_RECOVERY_MANIFEST_NAME);
    let metadata = std::fs::symlink_metadata(&path)
        .map_err(|_| DisasterRecoveryBundleDenial::ManifestUnavailable)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(DisasterRecoveryBundleDenial::ManifestMalformed);
    }
    let length = usize::try_from(metadata.len())
        .map_err(|_| DisasterRecoveryBundleDenial::ManifestTooLarge)?;
    if length == 0 || length > maximum_manifest_bytes {
        return Err(DisasterRecoveryBundleDenial::ManifestTooLarge);
    }
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(length)
        .map_err(|_| DisasterRecoveryBundleDenial::AllocationFailed)?;
    bytes.resize(length, 0);
    let mut file =
        std::fs::File::open(path).map_err(|_| DisasterRecoveryBundleDenial::ManifestUnavailable)?;
    file.read_exact(&mut bytes)
        .map_err(|_| DisasterRecoveryBundleDenial::ManifestMalformed)?;
    let mut extra = [0_u8; 1];
    if file
        .read(&mut extra)
        .map_err(|_| DisasterRecoveryBundleDenial::ManifestUnavailable)?
        != 0
    {
        return Err(DisasterRecoveryBundleDenial::ManifestTooLarge);
    }
    Ok(bytes)
}

fn existing_manifest_identity(
    path: &Path,
    expected: &[u8],
    identity: [u8; 32],
) -> Result<[u8; 32], DisasterRecoveryBundleDenial> {
    require_exact_bytes(path, expected)?;
    Ok(identity)
}

fn require_exact_bytes(path: &Path, expected: &[u8]) -> Result<(), DisasterRecoveryBundleDenial> {
    let metadata = std::fs::symlink_metadata(path)
        .map_err(|_| DisasterRecoveryBundleDenial::ManifestUnavailable)?;
    if metadata.file_type().is_symlink()
        || !metadata.is_file()
        || metadata.len() != expected.len() as u64
    {
        return Err(DisasterRecoveryBundleDenial::ManifestAlreadyExists);
    }
    let mut file =
        std::fs::File::open(path).map_err(|_| DisasterRecoveryBundleDenial::ManifestUnavailable)?;
    let mut buffer = [0_u8; 4096];
    for expected_chunk in expected.chunks(buffer.len()) {
        let actual = &mut buffer[..expected_chunk.len()];
        file.read_exact(actual)
            .map_err(|_| DisasterRecoveryBundleDenial::ManifestUnavailable)?;
        if actual != expected_chunk {
            return Err(DisasterRecoveryBundleDenial::ManifestAlreadyExists);
        }
    }
    let mut extra = [0_u8; 1];
    if file
        .read(&mut extra)
        .map_err(|_| DisasterRecoveryBundleDenial::ManifestUnavailable)?
        != 0
    {
        return Err(DisasterRecoveryBundleDenial::ManifestAlreadyExists);
    }
    Ok(())
}

fn pending_name(identity: [u8; 32]) -> String {
    let mut name = String::from(".disaster-recovery.manifest.");
    for byte in identity {
        use std::fmt::Write;
        write!(name, "{byte:02x}").expect("writing to a String cannot fail");
    }
    name.push_str(".pending");
    name
}

#[cfg(windows)]
fn sync_directory(path: &Path) -> Result<(), DisasterRecoveryBundleDenial> {
    use std::os::windows::fs::OpenOptionsExt;

    std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(0x0200_0000)
        .open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| DisasterRecoveryBundleDenial::ManifestUnavailable)
}

#[cfg(not(windows))]
fn sync_directory(path: &Path) -> Result<(), DisasterRecoveryBundleDenial> {
    std::fs::File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| DisasterRecoveryBundleDenial::ManifestUnavailable)
}

use std::fs::{File, Metadata, OpenOptions};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use sha2::{Digest, Sha256};

use super::{
    OfflineMediaFileIdentity, OfflineMediaReadDenial, ReadOnlyOfflineMediaCapability,
    StableReadOnlyFile,
};
use crate::OfflineMediaConsistencyBasis;

pub(crate) fn physical_file_identity(path: &Path) -> Result<[u8; 32], OfflineMediaReadDenial> {
    let capability = ReadOnlyOfflineMediaCapability::open(
        [path.to_path_buf()],
        OfflineMediaConsistencyBasis::single_artifact_mutation_detection(),
    )?;
    capability
        .file(0)
        .map(OfflineMediaFileIdentity::physical_key_fingerprint)
        .ok_or(OfflineMediaReadDenial::InvalidFileIndex)
}

pub(super) fn identity(
    path: PathBuf,
    metadata: &Metadata,
    physical_key: file_id::FileId,
    physical_alias_group: u64,
) -> OfflineMediaFileIdentity {
    let mut digest = Sha256::new();
    let mut physical_key_digest = Sha256::new();
    update_file_id_digest(&mut physical_key_digest, physical_key);
    let physical_key_fingerprint = physical_key_digest.finalize().into();
    update_path_digest(&mut digest, &path);
    digest.update(metadata.len().to_le_bytes());
    update_file_id_digest(&mut digest, physical_key);
    if let Ok(modified) = metadata.modified().and_then(|time| {
        time.duration_since(UNIX_EPOCH)
            .map_err(std::io::Error::other)
    }) {
        digest.update(modified.as_nanos().to_le_bytes());
    }
    OfflineMediaFileIdentity {
        path,
        length: metadata.len(),
        metadata_fingerprint: digest.finalize().into(),
        physical_alias_group,
        physical_key,
        physical_key_fingerprint,
    }
}

fn update_file_id_digest(digest: &mut Sha256, physical_key: file_id::FileId) {
    match physical_key {
        file_id::FileId::Inode {
            device_id,
            inode_number,
        } => {
            digest.update([1]);
            digest.update(device_id.to_le_bytes());
            digest.update(inode_number.to_le_bytes());
        }
        file_id::FileId::LowRes {
            volume_serial_number,
            file_index,
        } => {
            digest.update([2]);
            digest.update(volume_serial_number.to_le_bytes());
            digest.update(file_index.to_le_bytes());
        }
        file_id::FileId::HighRes {
            volume_serial_number,
            file_id,
        } => {
            digest.update([3]);
            digest.update(volume_serial_number.to_le_bytes());
            digest.update(file_id.to_le_bytes());
        }
    }
}

#[cfg(unix)]
fn update_path_digest(digest: &mut Sha256, path: &Path) {
    use std::os::unix::ffi::OsStrExt;
    digest.update(path.as_os_str().as_bytes());
}

#[cfg(windows)]
fn update_path_digest(digest: &mut Sha256, path: &Path) {
    use std::os::windows::ffi::OsStrExt;
    for unit in path.as_os_str().encode_wide() {
        digest.update(unit.to_le_bytes());
    }
}

#[cfg(not(any(unix, windows)))]
fn update_path_digest(digest: &mut Sha256, path: &Path) {
    digest.update(path.to_string_lossy().as_bytes());
}

pub(super) fn revalidate(file: &StableReadOnlyFile) -> Result<(), OfflineMediaReadDenial> {
    let opened = open_revalidated(file)?;
    revalidate_open_file(file, &opened)
}

pub(super) fn open_revalidated(file: &StableReadOnlyFile) -> Result<File, OfflineMediaReadDenial> {
    let key_before =
        file_id::get_file_id(&file.identity.path).map_err(|source| OfflineMediaReadDenial::Io {
            path: file.identity.path.clone(),
            source,
        })?;
    let opened = OpenOptions::new()
        .read(true)
        .open(&file.identity.path)
        .map_err(|source| OfflineMediaReadDenial::Io {
            path: file.identity.path.clone(),
            source,
        })?;
    if key_before != file.identity.physical_key {
        return Err(OfflineMediaReadDenial::ConcurrentMutationIndeterminate {
            path: file.identity.path.clone(),
        });
    }
    revalidate_open_file(file, &opened)?;
    Ok(opened)
}

pub(super) fn revalidate_open_file(
    file: &StableReadOnlyFile,
    opened: &File,
) -> Result<(), OfflineMediaReadDenial> {
    let metadata = opened
        .metadata()
        .map_err(|source| OfflineMediaReadDenial::Io {
            path: file.identity.path.clone(),
            source,
        })?;
    let key =
        file_id::get_file_id(&file.identity.path).map_err(|source| OfflineMediaReadDenial::Io {
            path: file.identity.path.clone(),
            source,
        })?;
    if identity(
        file.identity.path.clone(),
        &metadata,
        key,
        file.identity.physical_alias_group,
    ) != file.identity
    {
        return Err(OfflineMediaReadDenial::ConcurrentMutationIndeterminate {
            path: file.identity.path.clone(),
        });
    }
    Ok(())
}

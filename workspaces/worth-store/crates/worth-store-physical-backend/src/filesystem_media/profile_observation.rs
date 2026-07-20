use std::num::NonZeroU64;
use std::path::Path;

use sha2::{Digest, Sha256};

use super::capability_profile::ObservedFilesystemProfile;
use super::{
    qualification_basis::RootProfileBinding, CapabilitySupport, FilesystemBackendProfile,
    FilesystemLocation, MediaCapability, MediaQualificationDenial,
};

pub(super) fn observe_profile(
    root: &Path,
    boundary: &super::fault_interposition::MediaFaultInterposer,
) -> Result<FilesystemBackendProfile, std::io::ErrorKind> {
    let attempt = boundary.begin(super::MediaOperationRole::ObserveRootProfile, 0);
    if let Some(error) = attempt.fail_before_error() {
        attempt.denied();
        return Err(error.kind());
    }
    match observe_profile_at_root(root) {
        Ok(profile) => {
            attempt.completed(0);
            Ok(profile)
        }
        Err(kind) => {
            attempt.denied();
            Err(kind)
        }
    }
}

pub(super) fn observe_admission_profile(
    root: &Path,
    boundary: &super::fault_interposition::MediaFaultInterposer,
) -> Result<FilesystemBackendProfile, std::io::ErrorKind> {
    let target = if root.exists() {
        root
    } else {
        root.parent()
            .filter(|parent| parent.exists())
            .unwrap_or(root)
    };
    observe_profile(target, boundary)
}

fn observe_profile_at_root(root: &Path) -> Result<FilesystemBackendProfile, std::io::ErrorKind> {
    let canonical = root.canonicalize().map_err(|error| error.kind())?;
    let disks = sysinfo::Disks::new_with_refreshed_list();
    let disk = disks
        .list()
        .iter()
        .filter(|disk| path_is_on_mount(&canonical, disk.mount_point()))
        .max_by_key(|disk| disk.mount_point().components().count())
        .ok_or(std::io::ErrorKind::NotFound)?;
    let filesystem_type = disk.file_system().to_string_lossy().into_owned();
    let root_identity = digest_file_id(file_id::get_file_id(&canonical).map_err(|e| e.kind())?);
    let mount_identity = file_id::get_file_id(disk.mount_point()).map_err(|error| error.kind())?;
    let volume_identity = digest_file_id(mount_identity);
    let allocation_granularity =
        NonZeroU64::new(fs4::allocation_granularity(&canonical).map_err(|error| error.kind())?)
            .ok_or(std::io::ErrorKind::InvalidData)?;
    let support = MediaCapability::ALL.map(|capability| match capability {
        MediaCapability::MemoryMap
        | MediaCapability::DirectIo
        | MediaCapability::SparseAllocation
        | MediaCapability::EagerAllocation => CapabilitySupport::Unsupported,
        _ => CapabilitySupport::Supported,
    });
    let location = if is_remote_path(&canonical, &filesystem_type) {
        FilesystemLocation::Remote
    } else {
        FilesystemLocation::Local
    };
    Ok(FilesystemBackendProfile::from_root_observation(
        ObservedFilesystemProfile {
            support_by_capability: support,
            root_identity,
            volume_identity,
            filesystem_type: filesystem_type.into_boxed_str(),
            allocation_granularity,
            location,
            removable: disk.is_removable(),
            read_only: disk.is_read_only(),
        },
    ))
}

pub(super) fn deny_profile(
    profile: &FilesystemBackendProfile,
    counters: super::MediaCounterSnapshot,
) -> Option<MediaQualificationDenial> {
    let counters = || Box::new(counters);
    if profile.location() == FilesystemLocation::Remote {
        return Some(MediaQualificationDenial::RemoteFilesystem {
            counters: counters(),
        });
    }
    if profile.location() == FilesystemLocation::Unknown {
        return Some(MediaQualificationDenial::UnknownFilesystem {
            counters: counters(),
        });
    }
    if profile.is_removable() {
        return Some(MediaQualificationDenial::RemovableFilesystem {
            counters: counters(),
        });
    }
    if profile.is_read_only() {
        return Some(MediaQualificationDenial::ReadOnlyFilesystem {
            counters: counters(),
        });
    }
    let filesystem = profile.filesystem_type().to_ascii_lowercase();
    if filesystem.is_empty() {
        return Some(MediaQualificationDenial::UnknownFilesystem {
            counters: counters(),
        });
    }
    if filesystem.contains("fuse") || filesystem == "9p" {
        return Some(MediaQualificationDenial::UserspaceFilesystem {
            filesystem: filesystem.into_boxed_str(),
            counters: counters(),
        });
    }
    None
}

pub(super) fn profile_binding(
    profile: &FilesystemBackendProfile,
    access_contract: super::FilesystemAccessContract,
) -> RootProfileBinding {
    let support = super::MediaCapability::ALL.map(|capability| match profile.support(capability) {
        CapabilitySupport::Supported => 1,
        CapabilitySupport::Unsupported => 2,
        CapabilitySupport::Indeterminate => 3,
    });
    let location = match profile.location() {
        FilesystemLocation::Local => 1,
        FilesystemLocation::Remote => 2,
        FilesystemLocation::Unknown => 3,
    };
    let access = match access_contract {
        super::FilesystemAccessContract::CoordinatedServiceAccount => 1,
    };
    let profile_digest = digest_parts(&[
        profile.filesystem_type().as_bytes(),
        &profile.allocation_granularity().get().to_le_bytes(),
        &[
            location,
            profile.is_removable() as u8,
            profile.is_read_only() as u8,
            access,
        ],
        &support,
    ]);
    RootProfileBinding {
        contract_version: super::qualification_basis::qualification_contract_version(),
        root_identity: profile.root_identity(),
        volume_identity: profile.volume_identity(),
        profile_digest,
        backend_build_identity: filesystem_media_build_identity(),
        access_contract,
    }
}

/// Digest of the concrete media implementation sources and build posture.
/// It is rerun-binding evidence only and grants no operational authority.
pub fn filesystem_media_build_identity() -> [u8; 32] {
    let encoded = env!("WORTH_STORE_MEDIA_BUILD_ID").as_bytes();
    let mut identity = [0_u8; 32];
    for (index, pair) in encoded.chunks_exact(2).enumerate() {
        identity[index] = decode_hex(pair[0]) << 4 | decode_hex(pair[1]);
    }
    identity
}

fn decode_hex(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        _ => unreachable!("build identity is emitted as lowercase hexadecimal"),
    }
}

fn path_is_on_mount(path: &Path, mount: &Path) -> bool {
    #[cfg(windows)]
    {
        let normalize = |value: &Path| {
            value
                .as_os_str()
                .to_string_lossy()
                .trim_start_matches(r"\\?\")
                .replace('/', "\\")
                .to_ascii_lowercase()
        };
        normalize(path).starts_with(&normalize(mount))
    }
    #[cfg(not(windows))]
    path.starts_with(mount)
}

fn digest_file_id(identity: file_id::FileId) -> [u8; 32] {
    digest_parts(&[format!("{identity:?}").as_bytes()])
}

fn digest_parts(parts: &[&[u8]]) -> [u8; 32] {
    let mut digest = Sha256::new();
    for part in parts {
        digest.update((part.len() as u64).to_le_bytes());
        digest.update(part);
    }
    digest.finalize().into()
}

fn is_remote_path(path: &Path, filesystem_type: &str) -> bool {
    let fs = filesystem_type.to_ascii_lowercase();
    if matches!(fs.as_str(), "nfs" | "nfs4" | "cifs" | "smbfs") {
        return true;
    }
    #[cfg(windows)]
    {
        use std::path::{Component, Prefix};
        matches!(
            path.components().next(),
            Some(Component::Prefix(prefix))
                if matches!(prefix.kind(), Prefix::UNC(_, _) | Prefix::VerbatimUNC(_, _))
        )
    }
    #[cfg(not(windows))]
    false
}

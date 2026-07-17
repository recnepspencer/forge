use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use sha2::{Digest, Sha256};

use super::RecoveryPublicationDenial;
use crate::CurrentPhysicalRoot;

const MAGIC: &[u8; 8] = b"WORTHRPL";
const VERSION: u16 = 1;
const FIXED_BODY_BYTES: usize = 193;
const DIGEST_BYTES: usize = 32;
const MAX_PATH_BYTES: usize = 32 * 1024;
static NEXT_PENDING: AtomicU64 = AtomicU64::new(1);

pub(super) struct DurableRecoveryPublicationLocator;

pub(super) struct ReopenedRecoveryPublicationLocator {
    pub(super) plan_fingerprint: [u8; 32],
    pub(super) media_identity: [u8; 32],
    pub(super) staging_plan_fingerprint: [u8; 32],
    pub(super) media_root: std::path::PathBuf,
    pub(super) candidate_root: CurrentPhysicalRoot,
}

impl DurableRecoveryPublicationLocator {
    pub(super) fn binding_exists(
        publication_directory: &Path,
        binding: [u8; 32],
    ) -> Result<bool, RecoveryPublicationDenial> {
        let path = publication_directory
            .join("recovery-publication-locators")
            .join(format!("{}.locator", hex(binding)));
        match std::fs::metadata(path) {
            Ok(metadata) => Ok(metadata.is_file()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(_) => Err(RecoveryPublicationDenial::PublicationLocatorIo),
        }
    }

    pub(super) fn admit_or_persist(
        publication_directory: &Path,
        binding: [u8; 32],
        plan_fingerprint: [u8; 32],
        media_identity: [u8; 32],
        staging_plan_fingerprint: [u8; 32],
        candidate_root: CurrentPhysicalRoot,
        media_root: &Path,
    ) -> Result<(), RecoveryPublicationDenial> {
        let directory = publication_directory.join("recovery-publication-locators");
        std::fs::create_dir_all(&directory)
            .map_err(|_| RecoveryPublicationDenial::PublicationLocatorIo)?;
        let record = encode_record(
            binding,
            plan_fingerprint,
            media_identity,
            staging_plan_fingerprint,
            candidate_root,
            media_root,
        )?;
        let final_path = directory.join(format!("{}.locator", hex(binding)));
        if final_path.exists() {
            return verify_record(&final_path, &record);
        }
        let pending_path = directory.join(format!(
            ".pending-{}-{}",
            std::process::id(),
            NEXT_PENDING.fetch_add(1, Ordering::Relaxed),
        ));
        let mut pending = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&pending_path)
            .map_err(|_| RecoveryPublicationDenial::PublicationLocatorIo)?;
        pending
            .write_all(&record)
            .and_then(|()| pending.sync_all())
            .map_err(|_| RecoveryPublicationDenial::PublicationLocatorIo)?;
        match std::fs::hard_link(&pending_path, &final_path) {
            Ok(()) => {
                sync_directory(&directory)?;
                std::fs::remove_file(&pending_path)
                    .map_err(|_| RecoveryPublicationDenial::PublicationLocatorIo)?;
                sync_directory(&directory)
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                std::fs::remove_file(&pending_path)
                    .map_err(|_| RecoveryPublicationDenial::PublicationLocatorIo)?;
                verify_record(&final_path, &record)
            }
            Err(_) => Err(RecoveryPublicationDenial::PublicationLocatorIo),
        }
    }

    pub(super) fn reopen(
        publication_directory: &Path,
        binding: [u8; 32],
        candidate_root: CurrentPhysicalRoot,
    ) -> Result<ReopenedRecoveryPublicationLocator, RecoveryPublicationDenial> {
        let path = publication_directory
            .join("recovery-publication-locators")
            .join(format!("{}.locator", hex(binding)));
        let record =
            std::fs::read(path).map_err(|_| RecoveryPublicationDenial::PublicationLocatorIo)?;
        decode_record(&record, binding, candidate_root)
    }

    pub(super) fn reopen_by_binding(
        publication_directory: &Path,
        binding: [u8; 32],
    ) -> Result<ReopenedRecoveryPublicationLocator, RecoveryPublicationDenial> {
        let path = publication_directory
            .join("recovery-publication-locators")
            .join(format!("{}.locator", hex(binding)));
        let record =
            std::fs::read(path).map_err(|_| RecoveryPublicationDenial::PublicationLocatorIo)?;
        let candidate_root = root_from_record(&record, binding)?;
        decode_record(&record, binding, candidate_root)
    }
}

fn encode_record(
    binding: [u8; 32],
    plan_fingerprint: [u8; 32],
    media_identity: [u8; 32],
    staging_plan_fingerprint: [u8; 32],
    candidate_root: CurrentPhysicalRoot,
    media_root: &Path,
) -> Result<Vec<u8>, RecoveryPublicationDenial> {
    let (platform, path) = encode_path(media_root);
    if path.len() > MAX_PATH_BYTES {
        return Err(RecoveryPublicationDenial::PublicationLocatorPathTooLong);
    }
    let path_len = u32::try_from(path.len())
        .map_err(|_| RecoveryPublicationDenial::PublicationLocatorPathTooLong)?;
    let mut body = Vec::with_capacity(FIXED_BODY_BYTES + path.len() + DIGEST_BYTES);
    body.extend_from_slice(MAGIC);
    body.extend_from_slice(&VERSION.to_le_bytes());
    body.extend_from_slice(&binding);
    body.extend_from_slice(&plan_fingerprint);
    body.extend_from_slice(&media_identity);
    body.extend_from_slice(&staging_plan_fingerprint);
    body.extend_from_slice(&candidate_root.epoch().get().to_le_bytes());
    body.extend_from_slice(&candidate_root.manifest_epoch().get().to_le_bytes());
    body.extend_from_slice(&candidate_root.store_authority_identity().fingerprint());
    body.extend_from_slice(&ordering_identity(candidate_root));
    body.push(platform);
    body.extend_from_slice(&path_len.to_le_bytes());
    body.extend_from_slice(&path);
    let digest = Sha256::digest(&body);
    body.extend_from_slice(&digest);
    Ok(body)
}

fn verify_record(path: &Path, expected: &[u8]) -> Result<(), RecoveryPublicationDenial> {
    let observed =
        std::fs::read(path).map_err(|_| RecoveryPublicationDenial::PublicationLocatorIo)?;
    if observed == expected && valid_record(&observed) {
        Ok(())
    } else {
        Err(RecoveryPublicationDenial::PublicationLocatorConflict)
    }
}

fn valid_record(record: &[u8]) -> bool {
    if record.len() < FIXED_BODY_BYTES + DIGEST_BYTES
        || &record[..8] != MAGIC
        || u16::from_le_bytes([record[8], record[9]]) != VERSION
    {
        return false;
    }
    let path_len = u32::from_le_bytes(record[189..193].try_into().expect("fixed slice"));
    let Ok(path_len) = usize::try_from(path_len) else {
        return false;
    };
    let Some(body_len) = FIXED_BODY_BYTES.checked_add(path_len) else {
        return false;
    };
    record.len() == body_len + DIGEST_BYTES
        && Sha256::digest(&record[..body_len])[..] == record[body_len..]
}

fn decode_record(
    record: &[u8],
    expected_binding: [u8; 32],
    expected_root: CurrentPhysicalRoot,
) -> Result<ReopenedRecoveryPublicationLocator, RecoveryPublicationDenial> {
    if !valid_record(record)
        || record[10..42] != expected_binding
        || record[138..146] != expected_root.epoch().get().to_le_bytes()
        || record[146..154] != expected_root.manifest_epoch().get().to_le_bytes()
        || record[154..186] != expected_root.store_authority_identity().fingerprint()
        || record[186..188] != ordering_identity(expected_root)
    {
        return Err(RecoveryPublicationDenial::PublicationLocatorConflict);
    }
    let path_len = usize::try_from(u32::from_le_bytes(
        record[189..193].try_into().expect("fixed slice"),
    ))
    .map_err(|_| RecoveryPublicationDenial::PublicationLocatorConflict)?;
    let path = decode_path(record[188], &record[193..193 + path_len])?;
    Ok(ReopenedRecoveryPublicationLocator {
        plan_fingerprint: record[42..74].try_into().expect("fixed slice"),
        media_identity: record[74..106].try_into().expect("fixed slice"),
        staging_plan_fingerprint: record[106..138].try_into().expect("fixed slice"),
        media_root: path,
        candidate_root: expected_root,
    })
}

fn root_from_record(
    record: &[u8],
    expected_binding: [u8; 32],
) -> Result<CurrentPhysicalRoot, RecoveryPublicationDenial> {
    if !valid_record(record) || record[10..42] != expected_binding {
        return Err(RecoveryPublicationDenial::PublicationLocatorConflict);
    }
    let epoch = u64::from_le_bytes(record[138..146].try_into().expect("fixed slice"));
    let manifest = u64::from_le_bytes(record[146..154].try_into().expect("fixed slice"));
    let authority =
        worth_store_authority::StoreCurrentAuthorityIdentity::from_persisted_fingerprint(
            record[154..186].try_into().expect("fixed slice"),
        );
    let root = CurrentPhysicalRoot::from_physical_isolation_entry(
        crate::CurrentPhysicalRootBasis::new(
            crate::RootEpoch::from_admitted_physical_basis(epoch),
            crate::ManifestEpoch::from_admitted_physical_basis(manifest),
            authority,
        ),
        crate::PhysicalOrderingContract::root_swap_acquire_release(),
    )
    .map_err(|_| RecoveryPublicationDenial::PublicationLocatorConflict)?;
    if ordering_identity(root) != record[186..188] {
        return Err(RecoveryPublicationDenial::PublicationLocatorConflict);
    }
    Ok(root)
}

fn ordering_identity(root: CurrentPhysicalRoot) -> [u8; 2] {
    let site = match root.ordering().site() {
        crate::PhysicalOrderingSite::RootSwap => 1,
        crate::PhysicalOrderingSite::HazardPublication => 2,
        crate::PhysicalOrderingSite::ReaderEpochPublication => 3,
        crate::PhysicalOrderingSite::GenerationAdvancement => 4,
        crate::PhysicalOrderingSite::AllocatorPublication => 5,
        crate::PhysicalOrderingSite::Validation => 6,
    };
    let strength = match root.ordering().strength() {
        crate::PhysicalOrderingStrength::AcquireRelease => 1,
        crate::PhysicalOrderingStrength::SequentiallyConsistent => 2,
    };
    [site, strength]
}

fn hex(bytes: [u8; 32]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(64);
    for byte in bytes {
        output.push(char::from(DIGITS[usize::from(byte >> 4)]));
        output.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    output
}

#[cfg(windows)]
fn encode_path(path: &Path) -> (u8, Vec<u8>) {
    use std::os::windows::ffi::OsStrExt;
    let bytes = path
        .as_os_str()
        .encode_wide()
        .flat_map(u16::to_le_bytes)
        .collect();
    (1, bytes)
}

#[cfg(windows)]
fn decode_path(
    platform: u8,
    bytes: &[u8],
) -> Result<std::path::PathBuf, RecoveryPublicationDenial> {
    use std::os::windows::ffi::OsStringExt;
    if platform != 1 || !bytes.len().is_multiple_of(2) {
        return Err(RecoveryPublicationDenial::PublicationLocatorConflict);
    }
    let units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|unit| u16::from_le_bytes([unit[0], unit[1]]))
        .collect();
    Ok(std::ffi::OsString::from_wide(&units).into())
}

#[cfg(not(windows))]
fn encode_path(path: &Path) -> (u8, Vec<u8>) {
    use std::os::unix::ffi::OsStrExt;
    (2, path.as_os_str().as_bytes().to_vec())
}

#[cfg(not(windows))]
fn decode_path(
    platform: u8,
    bytes: &[u8],
) -> Result<std::path::PathBuf, RecoveryPublicationDenial> {
    use std::os::unix::ffi::OsStringExt;
    if platform != 2 {
        return Err(RecoveryPublicationDenial::PublicationLocatorConflict);
    }
    Ok(std::ffi::OsString::from_vec(bytes.to_vec()).into())
}

fn sync_directory(path: &Path) -> Result<(), RecoveryPublicationDenial> {
    open_directory(path)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| RecoveryPublicationDenial::PublicationLocatorIo)
}

#[cfg(windows)]
fn open_directory(path: &Path) -> std::io::Result<File> {
    use std::os::windows::fs::OpenOptionsExt;
    OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(0x0200_0000)
        .open(path)
}

#[cfg(not(windows))]
fn open_directory(path: &Path) -> std::io::Result<File> {
    File::open(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        epoch::{manifest_epoch_from_entry_seed, root_epoch_from_entry_seed},
        CurrentPhysicalRootBasis, PhysicalOrderingContract,
    };

    #[test]
    fn durable_locator_is_idempotent_but_rejects_a_conflicting_media_path() {
        let directory = tempfile::tempdir().unwrap();
        let media = directory.path().join("candidate-a");
        let different_media = directory.path().join("candidate-b");
        let root = candidate_root();
        let persist = |path: &Path| {
            DurableRecoveryPublicationLocator::admit_or_persist(
                directory.path(),
                [1; 32],
                [2; 32],
                [3; 32],
                [4; 32],
                root,
                path,
            )
        };

        persist(&media).unwrap();
        persist(&media).unwrap();
        let reopened = DurableRecoveryPublicationLocator::reopen(directory.path(), [1; 32], root)
            .expect("fresh owner reopens exact locator");
        assert_eq!(reopened.plan_fingerprint, [2; 32]);
        assert_eq!(reopened.media_identity, [3; 32]);
        assert_eq!(reopened.staging_plan_fingerprint, [4; 32]);
        assert_eq!(reopened.media_root, media);
        let rebound =
            DurableRecoveryPublicationLocator::reopen_by_binding(directory.path(), [1; 32])
                .expect("locator authority reconstructs its candidate root");
        assert_eq!(rebound.candidate_root, root);
        assert_eq!(
            persist(&different_media).unwrap_err(),
            RecoveryPublicationDenial::PublicationLocatorConflict,
        );
    }

    fn candidate_root() -> CurrentPhysicalRoot {
        CurrentPhysicalRoot::from_physical_isolation_entry(
            CurrentPhysicalRootBasis::new(
                root_epoch_from_entry_seed(91),
                manifest_epoch_from_entry_seed(91),
                worth_store_physical_format::PhysicalStoreIdentity::physical_format_default()
                    .authority_identity(),
            ),
            PhysicalOrderingContract::root_swap_acquire_release(),
        )
        .unwrap()
    }
}

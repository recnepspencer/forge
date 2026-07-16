use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use fs4::FileExt;
use sha2::{Digest, Sha256};
use worth_store_physical_backend::{
    reach_storage_boundary, ProductionStorageBoundaryControl, ProductionStorageBoundarySeam,
    StorageBoundaryRegion,
};

use super::PhysicalPublicationDenial;
use crate::CurrentPhysicalRoot;

const RECORD_MAGIC: &[u8; 8] = b"WORTHROT";
const RECORD_VERSION: u16 = 3;
const RECORD_BODY_BYTES: usize = 92;
const RECORD_BYTES: usize = RECORD_BODY_BYTES + 32;
const NO_RECOVERY_BINDING: [u8; 32] = [0; 32];

#[derive(Debug)]
pub(super) struct PhysicalRootPublicationStore {
    log_path: PathBuf,
    directory: File,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PersistedRootIdentity {
    root_epoch: u64,
    manifest_epoch: u64,
    authority: [u8; 32],
    ordering: [u8; 2],
    recovery_binding: [u8; 32],
}

impl PhysicalRootPublicationStore {
    pub(super) fn open(
        publication_directory: &Path,
        expected_current: CurrentPhysicalRoot,
    ) -> Result<Self, PhysicalPublicationDenial> {
        std::fs::create_dir_all(publication_directory)
            .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
        let directory = open_directory(publication_directory)
            .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
        let store = Self {
            log_path: publication_directory.join("root-publications.log"),
            directory,
        };
        store.admit_or_initialize(expected_current)?;
        Ok(store)
    }

    #[cfg(test)]
    pub(super) fn publish(
        &self,
        expected_current: CurrentPhysicalRoot,
        candidate: CurrentPhysicalRoot,
    ) -> Result<(), PhysicalPublicationDenial> {
        self.publish_with_boundary_control(
            expected_current,
            candidate,
            &worth_store_physical_backend::UninterruptedStorageBoundaryControl,
        )
    }

    pub(super) fn publish_with_boundary_control(
        &self,
        expected_current: CurrentPhysicalRoot,
        candidate: CurrentPhysicalRoot,
        control: &impl ProductionStorageBoundaryControl,
    ) -> Result<(), PhysicalPublicationDenial> {
        self.publish_with_binding(expected_current, candidate, NO_RECOVERY_BINDING, control)
    }

    pub(super) fn publish_recovery_with_boundary_control(
        &self,
        expected_current: CurrentPhysicalRoot,
        candidate: CurrentPhysicalRoot,
        recovery_binding: [u8; 32],
        control: &impl ProductionStorageBoundaryControl,
    ) -> Result<(), PhysicalPublicationDenial> {
        if recovery_binding == NO_RECOVERY_BINDING {
            return Err(PhysicalPublicationDenial::MissingRecoveryPublicationBinding);
        }
        self.publish_with_binding(expected_current, candidate, recovery_binding, control)
    }

    fn publish_with_binding(
        &self,
        expected_current: CurrentPhysicalRoot,
        candidate: CurrentPhysicalRoot,
        recovery_binding: [u8; 32],
        control: &impl ProductionStorageBoundaryControl,
    ) -> Result<(), PhysicalPublicationDenial> {
        let mut log = self.open_log()?;
        log.lock_exclusive()
            .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
        let (valid_bytes, current) = read_last_valid_identity(&mut log)?;
        if !current.is_some_and(|identity| identity.matches_root(expected_current)) {
            return Err(PhysicalPublicationDenial::ConcurrentRootPublication);
        }
        log.set_len(valid_bytes)
            .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
        log.seek(SeekFrom::Start(valid_bytes))
            .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
        let encoded = encode_record(candidate, recovery_binding);
        log.write_all(&encoded)
            .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
        reach_storage_boundary(
            control,
            ProductionStorageBoundarySeam::RootSwap,
            &mut log,
            StorageBoundaryRegion::new(valid_bytes, encoded.len() as u64),
        )
        .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
        log.sync_all()
            .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
        reach_storage_boundary(
            control,
            ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
            &mut log,
            StorageBoundaryRegion::new(valid_bytes, encoded.len() as u64),
        )
        .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
        Ok(())
    }

    pub(super) fn current_recovery_binding(
        &self,
    ) -> Result<Option<[u8; 32]>, PhysicalPublicationDenial> {
        let mut log = self.open_log()?;
        log.lock_shared()
            .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
        let (_, current) = read_last_valid_identity(&mut log)?;
        Ok(current.and_then(PersistedRootIdentity::recovery_binding))
    }

    fn admit_or_initialize(
        &self,
        expected_current: CurrentPhysicalRoot,
    ) -> Result<(), PhysicalPublicationDenial> {
        let mut log = self.open_log()?;
        log.lock_exclusive()
            .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
        let (valid_bytes, current) = read_last_valid_identity(&mut log)?;
        match current {
            Some(persisted) if persisted.matches_root(expected_current) => {
                if log
                    .metadata()
                    .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?
                    .len()
                    != valid_bytes
                {
                    log.set_len(valid_bytes)
                        .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
                    log.sync_all()
                        .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
                }
                Ok(())
            }
            Some(_) => Err(PhysicalPublicationDenial::PersistedRootMismatch),
            None => {
                log.set_len(valid_bytes)
                    .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
                log.seek(SeekFrom::Start(valid_bytes))
                    .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
                log.write_all(&encode_record(expected_current, NO_RECOVERY_BINDING))
                    .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
                log.sync_all()
                    .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
                self.directory
                    .sync_all()
                    .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)
            }
        }
    }

    fn open_log(&self) -> Result<File, PhysicalPublicationDenial> {
        OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(&self.log_path)
            .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)
    }
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

impl PersistedRootIdentity {
    fn from_root(root: CurrentPhysicalRoot) -> Self {
        Self {
            root_epoch: root.epoch().get(),
            manifest_epoch: root.manifest_epoch().get(),
            authority: root.store_authority_identity().fingerprint(),
            ordering: ordering_identity(root.ordering()),
            recovery_binding: NO_RECOVERY_BINDING,
        }
    }

    fn matches_root(self, root: CurrentPhysicalRoot) -> bool {
        let expected = Self::from_root(root);
        self.root_epoch == expected.root_epoch
            && self.manifest_epoch == expected.manifest_epoch
            && self.authority == expected.authority
            && self.ordering == expected.ordering
    }

    fn recovery_binding(self) -> Option<[u8; 32]> {
        (self.recovery_binding != NO_RECOVERY_BINDING).then_some(self.recovery_binding)
    }
}

fn encode_record(root: CurrentPhysicalRoot, recovery_binding: [u8; 32]) -> [u8; RECORD_BYTES] {
    let mut record = [0_u8; RECORD_BYTES];
    record[..8].copy_from_slice(RECORD_MAGIC);
    record[8..10].copy_from_slice(&RECORD_VERSION.to_le_bytes());
    record[10..18].copy_from_slice(&root.epoch().get().to_le_bytes());
    record[18..26].copy_from_slice(&root.manifest_epoch().get().to_le_bytes());
    record[26..58].copy_from_slice(&root.store_authority_identity().fingerprint());
    record[58..60].copy_from_slice(&ordering_identity(root.ordering()));
    record[60..92].copy_from_slice(&recovery_binding);
    let digest = Sha256::digest(&record[..RECORD_BODY_BYTES]);
    record[RECORD_BODY_BYTES..].copy_from_slice(&digest);
    record
}

fn read_last_valid_identity(
    log: &mut File,
) -> Result<(u64, Option<PersistedRootIdentity>), PhysicalPublicationDenial> {
    log.seek(SeekFrom::Start(0))
        .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
    let mut bytes = Vec::new();
    log.read_to_end(&mut bytes)
        .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
    let mut offset = 0;
    let mut last = None;
    while bytes.len().saturating_sub(offset) >= RECORD_BYTES {
        let record = &bytes[offset..offset + RECORD_BYTES];
        if &record[..8] != RECORD_MAGIC
            || u16::from_le_bytes(record[8..10].try_into().expect("fixed record")) != RECORD_VERSION
            || Sha256::digest(&record[..RECORD_BODY_BYTES])[..] != record[RECORD_BODY_BYTES..]
        {
            return Err(PhysicalPublicationDenial::PersistedRootMismatch);
        }
        last = Some(PersistedRootIdentity {
            root_epoch: u64::from_le_bytes(record[10..18].try_into().expect("fixed record")),
            manifest_epoch: u64::from_le_bytes(record[18..26].try_into().expect("fixed record")),
            authority: record[26..58].try_into().expect("fixed record"),
            ordering: record[58..60].try_into().expect("fixed record"),
            recovery_binding: record[60..92].try_into().expect("fixed record"),
        });
        offset += RECORD_BYTES;
    }
    Ok((offset as u64, last))
}

const fn ordering_identity(ordering: crate::PhysicalOrderingContract) -> [u8; 2] {
    let site = match ordering.site() {
        crate::PhysicalOrderingSite::RootSwap => 1,
        crate::PhysicalOrderingSite::HazardPublication => 2,
        crate::PhysicalOrderingSite::ReaderEpochPublication => 3,
        crate::PhysicalOrderingSite::GenerationAdvancement => 4,
        crate::PhysicalOrderingSite::AllocatorPublication => 5,
        crate::PhysicalOrderingSite::Validation => 6,
    };
    let strength = match ordering.strength() {
        crate::PhysicalOrderingStrength::AcquireRelease => 1,
        crate::PhysicalOrderingStrength::SequentiallyConsistent => 2,
    };
    [site, strength]
}

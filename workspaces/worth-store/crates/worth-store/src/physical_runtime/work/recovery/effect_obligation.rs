use std::sync::Mutex;

use sha2::{Digest, Sha256};
use worth_store_physical_backend::{
    ArtifactTreeDirectory, ArtifactTreeFile, QualifiedFilesystemMedia,
};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::{
    super::{PhysicalWorkIdentity, PhysicalWorkOperationFamily},
    locator::{decode_locator, encode_target, RECOVERY_RECORD_BYTES},
    PhysicalWorkRecoveryLocator, PhysicalWorkRecoveryTarget,
};

pub(in crate::physical_runtime) struct PhysicalEffectJournal {
    directory: ArtifactTreeDirectory,
    initialized: Mutex<bool>,
}

pub(in crate::physical_runtime) struct PreparedPhysicalEffect {
    artifact: ArtifactTreeFile,
}

pub(in crate::physical_runtime) struct PhysicalEffectRecoveryInventory {
    obligations: Box<[PhysicalWorkRecoveryLocator]>,
    damaged: bool,
}

impl PhysicalEffectJournal {
    pub(in crate::physical_runtime) fn new(media: &QualifiedFilesystemMedia) -> Self {
        let directory = journal_directory();
        let initialized = media
            .artifact_tree()
            .directory_exists(&directory)
            .unwrap_or(false);
        Self {
            directory,
            initialized: Mutex::new(initialized),
        }
    }

    pub(in crate::physical_runtime) fn inspect(
        media: &QualifiedFilesystemMedia,
        limit: usize,
    ) -> PhysicalEffectRecoveryInventory {
        let tree = media.artifact_tree();
        let directory = journal_directory();
        match tree.directory_exists(&directory) {
            Ok(false) => PhysicalEffectRecoveryInventory::empty(),
            Ok(true) => inspect_entries(media.store_identity(), tree, directory, limit),
            Err(_) => PhysicalEffectRecoveryInventory::damaged(),
        }
    }

    pub(in crate::physical_runtime) fn prepare(
        &self,
        media: &QualifiedFilesystemMedia,
        identity: PhysicalWorkIdentity,
        operation: PhysicalWorkOperationFamily,
        target: PhysicalWorkRecoveryTarget,
        payload_digest: Option<[u8; 32]>,
    ) -> Result<PreparedPhysicalEffect, ()> {
        self.ensure_directory(media)?;
        let artifact = self
            .directory
            .file(&format!(
                "effect-{:016x}-{:016x}-{:016x}.pending",
                identity.runtime().get(),
                identity.generation().lifecycle().get(),
                identity.operation().get(),
            ))
            .map_err(|_| ())?;
        let record = encode_record(identity, operation, target, payload_digest);
        let tree = media.artifact_tree();
        tree.write_new_obligation_record(&artifact, &record)
            .map_err(|_| ())?;
        tree.synchronize_directory(&self.directory)
            .map_err(|_| ())?;
        Ok(PreparedPhysicalEffect { artifact })
    }

    pub(in crate::physical_runtime) fn finish(
        &self,
        media: &QualifiedFilesystemMedia,
        prepared: PreparedPhysicalEffect,
    ) -> Result<(), ()> {
        media
            .artifact_tree()
            .remove_file_durably(&prepared.artifact)
            .map_err(|_| ())
    }

    fn ensure_directory(&self, media: &QualifiedFilesystemMedia) -> Result<(), ()> {
        let mut initialized = self
            .initialized
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if *initialized {
            return Ok(());
        }
        let tree = media.artifact_tree();
        if !tree.directory_exists(&self.directory).map_err(|_| ())? {
            tree.create_directory(&self.directory).map_err(|_| ())?;
            tree.synchronize_directory(&ArtifactTreeDirectory::families())
                .map_err(|_| ())?;
        }
        *initialized = true;
        Ok(())
    }
}

fn journal_directory() -> ArtifactTreeDirectory {
    ArtifactTreeDirectory::families()
        .child("physical-work")
        .expect("portable physical-work recovery path")
}

fn encode_record(
    identity: PhysicalWorkIdentity,
    operation: PhysicalWorkOperationFamily,
    target: PhysicalWorkRecoveryTarget,
    payload_digest: Option<[u8; 32]>,
) -> [u8; RECOVERY_RECORD_BYTES] {
    let mut record = [0_u8; RECOVERY_RECORD_BYTES];
    record[..8].copy_from_slice(b"WPEFFECT");
    record[8] = 4;
    record[9] = match operation {
        PhysicalWorkOperationFamily::ArtifactRangeRead => 1,
        PhysicalWorkOperationFamily::ArtifactRangeWrite => 2,
        PhysicalWorkOperationFamily::ArtifactPublication => 3,
        PhysicalWorkOperationFamily::ArtifactMetadataRead => 4,
        PhysicalWorkOperationFamily::WalAppend => 5,
        PhysicalWorkOperationFamily::DurabilityBarrier => 6,
    };
    record[16..32].copy_from_slice(&identity.store().bytes());
    record[32..40].copy_from_slice(&identity.runtime().get().to_le_bytes());
    record[40..48].copy_from_slice(&identity.generation().lifecycle().get().to_le_bytes());
    record[48..56].copy_from_slice(&identity.operation().get().to_le_bytes());
    if let Some(digest) = payload_digest {
        record[68] = 1;
        record[72..104].copy_from_slice(&digest);
    }
    encode_target(target, &mut record);
    let checksum = Sha256::digest(&record[..128]);
    record[128..].copy_from_slice(&checksum);
    record
}

fn inspect_entries(
    store: StableStoreIdentity,
    tree: worth_store_physical_backend::ArtifactTreeMedia<'_>,
    directory: ArtifactTreeDirectory,
    limit: usize,
) -> PhysicalEffectRecoveryInventory {
    let names = match tree.list_file_names_bounded(&directory, limit) {
        Ok(names) => names,
        Err(_) => return PhysicalEffectRecoveryInventory::damaged(),
    };
    let mut obligations = Vec::with_capacity(names.len());
    let mut damaged = false;
    for name in names {
        let decoded = directory
            .file(&name)
            .ok()
            .and_then(|file| tree.read_bounded(&file, RECOVERY_RECORD_BYTES as u32).ok())
            .and_then(|record| decode_locator(store, &name, &record));
        match decoded {
            Some(locator) => obligations.push(locator),
            None => damaged = true,
        }
    }
    PhysicalEffectRecoveryInventory {
        obligations: obligations.into_boxed_slice(),
        damaged,
    }
}

impl PhysicalEffectRecoveryInventory {
    fn empty() -> Self {
        Self {
            obligations: Box::new([]),
            damaged: false,
        }
    }

    fn damaged() -> Self {
        Self {
            obligations: Box::new([]),
            damaged: true,
        }
    }

    pub(in crate::physical_runtime) fn requires_inspection(&self) -> bool {
        self.damaged || !self.obligations.is_empty()
    }

    pub(in crate::physical_runtime) fn obligations(&self) -> &[PhysicalWorkRecoveryLocator] {
        &self.obligations
    }

    pub(in crate::physical_runtime) const fn evidence_damaged(&self) -> bool {
        self.damaged
    }
}

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use super::physical_abandonment::{
    abandon_physical_materialization, PhysicalBackupMaterializationAbandonment,
    PhysicalBackupMaterializationAbandonmentDenial,
};
use super::published_bundle_revalidation::{
    require_exact_file, validate_final_bundle, validate_recovered_staging_bundle,
};
use super::session_identity::{descriptor_path, PhysicalBackupSessionIdentityGuard};
use super::{
    add_counter, io_denial, reject_symbolic_link, reject_symbolic_link_if_present,
    PhysicalBackupMaterializationCounters, PhysicalBackupMaterializationDenial,
    PhysicalMaterializedBackupBundle,
};
use crate::PhysicalBackupSource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalBackupPublicationProgress {
    ExistingFinalBundleRevalidated,
    PendingManifestDurable,
    ManifestPublished,
    SessionDescriptorRemoved,
    StagingDirectoryDurable,
    FinalRootRenamed,
    ParentDirectoryDurable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PublicationPhase {
    RevalidateFinal,
    WritePending,
    SyncPending,
    RenameManifest,
    RemoveSessionDescriptor,
    SyncStaging,
    RenameRoot,
    SyncParent,
    Complete,
}

pub struct PhysicalBackupPublicationSession {
    staging_root: PathBuf,
    final_root: PathBuf,
    sources: Vec<PhysicalBackupSource>,
    buffer: Vec<u8>,
    manifest_bytes: Vec<u8>,
    counters: PhysicalBackupMaterializationCounters,
    phase: PublicationPhase,
    _session_identity_guard: PhysicalBackupSessionIdentityGuard,
}

pub(super) struct PhysicalBackupPublicationOpening {
    pub(super) staging_root: PathBuf,
    pub(super) final_root: PathBuf,
    pub(super) sources: Vec<PhysicalBackupSource>,
    pub(super) buffer: Vec<u8>,
    pub(super) manifest_bytes: Vec<u8>,
    pub(super) counters: PhysicalBackupMaterializationCounters,
    pub(super) session_identity_guard: PhysicalBackupSessionIdentityGuard,
    pub(super) recovered_publication: bool,
}

impl PhysicalBackupPublicationSession {
    pub(super) fn new(
        opening: PhysicalBackupPublicationOpening,
    ) -> Result<Self, PhysicalBackupMaterializationDenial> {
        let PhysicalBackupPublicationOpening {
            staging_root,
            final_root,
            sources,
            mut buffer,
            manifest_bytes,
            mut counters,
            session_identity_guard,
            recovered_publication,
        } = opening;
        let phase = discover_phase(
            &staging_root,
            &final_root,
            &sources,
            &manifest_bytes,
            &mut buffer,
            &mut counters,
            recovered_publication,
        )?;
        Ok(Self {
            staging_root,
            final_root,
            sources,
            buffer,
            manifest_bytes,
            counters,
            phase,
            _session_identity_guard: session_identity_guard,
        })
    }

    pub fn advance(
        &mut self,
    ) -> Result<Option<PhysicalBackupPublicationProgress>, PhysicalBackupMaterializationDenial>
    {
        self.advance_observing(None)
    }

    pub fn advance_with_cancellation(
        &mut self,
        cancellation: &crate::PhysicalBackupMaterializationCancellation,
    ) -> Result<Option<PhysicalBackupPublicationProgress>, PhysicalBackupMaterializationDenial>
    {
        self.advance_observing(Some(cancellation))
    }

    fn advance_observing(
        &mut self,
        cancellation: Option<&crate::PhysicalBackupMaterializationCancellation>,
    ) -> Result<Option<PhysicalBackupPublicationProgress>, PhysicalBackupMaterializationDenial>
    {
        if self.phase == PublicationPhase::Complete {
            return Ok(None);
        }
        if cancellation.is_some_and(crate::PhysicalBackupMaterializationCancellation::is_cancelled)
        {
            return Err(PhysicalBackupMaterializationDenial::Cancelled);
        }
        let progress = match self.phase {
            PublicationPhase::RevalidateFinal => {
                validate_final_bundle(
                    &self.final_root,
                    &self.sources,
                    &self.manifest_bytes,
                    &mut self.buffer,
                    &mut self.counters,
                )?;
                self.phase = PublicationPhase::SyncParent;
                PhysicalBackupPublicationProgress::ExistingFinalBundleRevalidated
            }
            PublicationPhase::WritePending => {
                let pending = self.staging_root.join("backup.manifest.pending");
                reject_symbolic_link_if_present(&pending)?;
                let mut file = OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .open(&pending)
                    .map_err(|source| io_denial(&pending, source))?;
                file.write_all(&self.manifest_bytes)
                    .map_err(|source| io_denial(&pending, source))?;
                self.counters.manifest_bytes_written = self.manifest_bytes.len() as u64;
                file.sync_all()
                    .map_err(|source| io_denial(&pending, source))?;
                add_counter(&mut self.counters.sync_operations, 1)?;
                self.phase = PublicationPhase::RenameManifest;
                PhysicalBackupPublicationProgress::PendingManifestDurable
            }
            PublicationPhase::SyncPending => {
                let pending = self.staging_root.join("backup.manifest.pending");
                let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(&pending)
                    .map_err(|source| io_denial(&pending, source))?;
                file.sync_all()
                    .map_err(|source| io_denial(&pending, source))?;
                add_counter(&mut self.counters.sync_operations, 1)?;
                self.phase = PublicationPhase::RenameManifest;
                PhysicalBackupPublicationProgress::PendingManifestDurable
            }
            PublicationPhase::RenameManifest => {
                let pending = self.staging_root.join("backup.manifest.pending");
                let published = self.staging_root.join("backup.manifest");
                std::fs::rename(&pending, &published)
                    .map_err(|source| io_denial(&pending, source))?;
                self.phase = PublicationPhase::RemoveSessionDescriptor;
                PhysicalBackupPublicationProgress::ManifestPublished
            }
            PublicationPhase::RemoveSessionDescriptor => {
                let descriptor = descriptor_path(&self.staging_root);
                match std::fs::remove_file(&descriptor) {
                    Ok(()) => {}
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                    Err(source) => return Err(io_denial(&descriptor, source)),
                }
                self.phase = PublicationPhase::SyncStaging;
                PhysicalBackupPublicationProgress::SessionDescriptorRemoved
            }
            PublicationPhase::SyncStaging => {
                crate::directory_durability::sync_directory(&self.staging_root)
                    .map_err(|source| io_denial(&self.staging_root, source))?;
                add_counter(&mut self.counters.sync_operations, 1)?;
                self.phase = PublicationPhase::RenameRoot;
                PhysicalBackupPublicationProgress::StagingDirectoryDurable
            }
            PublicationPhase::RenameRoot => {
                std::fs::rename(&self.staging_root, &self.final_root)
                    .map_err(|source| io_denial(&self.staging_root, source))?;
                self.phase = PublicationPhase::SyncParent;
                PhysicalBackupPublicationProgress::FinalRootRenamed
            }
            PublicationPhase::SyncParent => {
                let parent = parent(&self.final_root)?;
                crate::directory_durability::sync_directory(parent)
                    .map_err(|source| io_denial(parent, source))?;
                add_counter(&mut self.counters.sync_operations, 1)?;
                self.phase = PublicationPhase::Complete;
                PhysicalBackupPublicationProgress::ParentDirectoryDurable
            }
            PublicationPhase::Complete => return Ok(None),
        };
        Ok(Some(progress))
    }

    pub fn abandon(
        self,
    ) -> Result<
        PhysicalBackupMaterializationAbandonment,
        PhysicalBackupMaterializationAbandonmentDenial,
    > {
        let Self {
            staging_root,
            final_root,
            sources,
            buffer,
            manifest_bytes,
            _session_identity_guard,
            ..
        } = self;
        drop(sources);
        drop(buffer);
        drop(manifest_bytes);
        let result = abandon_physical_materialization(staging_root, final_root);
        drop(_session_identity_guard);
        result
    }

    pub fn finish(
        mut self,
    ) -> Result<PhysicalMaterializedBackupBundle, PhysicalBackupMaterializationDenial> {
        while self.advance()?.is_some() {}
        Ok(PhysicalMaterializedBackupBundle {
            root: self.final_root,
            counters: self.counters,
        })
    }

    pub const fn counters(&self) -> PhysicalBackupMaterializationCounters {
        self.counters
    }
}

fn discover_phase(
    staging_root: &Path,
    final_root: &Path,
    sources: &[PhysicalBackupSource],
    manifest_bytes: &[u8],
    buffer: &mut [u8],
    counters: &mut PhysicalBackupMaterializationCounters,
    recovered_publication: bool,
) -> Result<PublicationPhase, PhysicalBackupMaterializationDenial> {
    if final_root.exists() {
        reject_symbolic_link(final_root)?;
        if staging_root.exists() {
            return Err(
                PhysicalBackupMaterializationDenial::ConflictingPublicationState {
                    staging_root: staging_root.to_path_buf(),
                    final_root: final_root.to_path_buf(),
                },
            );
        }
        return Ok(PublicationPhase::RevalidateFinal);
    }
    reject_symbolic_link(staging_root)?;
    let pending = staging_root.join("backup.manifest.pending");
    let published = staging_root.join("backup.manifest");
    let descriptor = descriptor_path(staging_root);
    reject_symbolic_link_if_present(&pending)?;
    reject_symbolic_link_if_present(&published)?;
    match (pending.exists(), published.exists()) {
        (false, false) => {
            if !descriptor.exists() {
                return Err(
                    PhysicalBackupMaterializationDenial::SessionIdentityMismatch {
                        path: descriptor,
                    },
                );
            }
            Ok(PublicationPhase::WritePending)
        }
        (true, false) => {
            if !descriptor.exists() {
                return Err(
                    PhysicalBackupMaterializationDenial::SessionIdentityMismatch {
                        path: descriptor,
                    },
                );
            }
            require_exact_file(&pending, manifest_bytes, buffer, counters)?;
            if recovered_publication {
                validate_recovered_staging_bundle(staging_root, sources, buffer, counters)?;
            }
            Ok(PublicationPhase::SyncPending)
        }
        (false, true) => {
            require_exact_file(&published, manifest_bytes, buffer, counters)?;
            if recovered_publication {
                validate_recovered_staging_bundle(staging_root, sources, buffer, counters)?;
            }
            if descriptor.exists() {
                Ok(PublicationPhase::RemoveSessionDescriptor)
            } else {
                Ok(PublicationPhase::SyncStaging)
            }
        }
        (true, true) => {
            Err(PhysicalBackupMaterializationDenial::ExistingPublicationMismatch { path: pending })
        }
    }
}

fn parent(path: &Path) -> Result<&Path, PhysicalBackupMaterializationDenial> {
    path.parent().ok_or_else(|| {
        io_denial(
            path,
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "bundle has no parent"),
        )
    })
}

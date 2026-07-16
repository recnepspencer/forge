use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use super::PhysicalBackupSource;

mod materialization_counters;
mod materialization_denial;
mod materialization_progress;
mod physical_abandonment;
mod publication;
mod published_bundle_revalidation;
mod resume_validation;
mod session_identity;
mod session_open;
mod source_set_admission;

pub use materialization_counters::{
    PhysicalBackupMaterializationCounterScope, PhysicalBackupMaterializationCounters,
};
pub use materialization_denial::PhysicalBackupMaterializationDenial;
pub use materialization_progress::{
    PhysicalBackupArtifactDurabilityProgress, PhysicalBackupCopyProgress,
    PhysicalBackupMaterializationProgress,
};
use physical_abandonment::abandon_physical_materialization;
pub use physical_abandonment::{
    PendingPhysicalBackupMaterializationCleanup, PhysicalBackupMaterializationAbandonment,
    PhysicalBackupMaterializationAbandonmentDenial,
};
use publication::PhysicalBackupPublicationOpening;
pub use publication::{PhysicalBackupPublicationProgress, PhysicalBackupPublicationSession};
use resume_validation::{validate_resume_prefix, validate_source_identity_and_length};
use session_identity::PhysicalBackupSessionIdentityGuard;
use source_set_admission::{
    allocate_buffer, collect_sources, reject_symbolic_link, reject_symbolic_link_if_present,
    validate_source_set,
};

struct SourceProgress {
    source: PhysicalBackupSource,
    copied: u64,
    hasher: Sha256,
    input: File,
    output: File,
    durable: bool,
}

pub struct PhysicalBackupMaterializationSession {
    staging_root: PathBuf,
    final_root: PathBuf,
    sources: Vec<SourceProgress>,
    source_index: usize,
    buffer: Vec<u8>,
    counters: PhysicalBackupMaterializationCounters,
    published_recovery_sources: Option<Vec<PhysicalBackupSource>>,
    session_identity_guard: PhysicalBackupSessionIdentityGuard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalMaterializedBackupBundle {
    root: PathBuf,
    counters: PhysicalBackupMaterializationCounters,
}

impl PhysicalMaterializedBackupBundle {
    pub fn root(&self) -> &Path {
        &self.root
    }
    pub const fn counters(&self) -> PhysicalBackupMaterializationCounters {
        self.counters
    }
}

impl PhysicalBackupMaterializationSession {
    pub fn advance(&mut self) -> Result<bool, PhysicalBackupMaterializationDenial> {
        self.advance_legacy(None)
    }

    pub fn advance_with_cancellation(
        &mut self,
        cancellation: &crate::PhysicalBackupMaterializationCancellation,
    ) -> Result<bool, PhysicalBackupMaterializationDenial> {
        self.advance_legacy(Some(cancellation))
    }

    pub fn advance_boundary(
        &mut self,
    ) -> Result<Option<PhysicalBackupMaterializationProgress>, PhysicalBackupMaterializationDenial>
    {
        self.advance_boundary_observing(None)
    }

    pub fn advance_boundary_with_cancellation(
        &mut self,
        cancellation: &crate::PhysicalBackupMaterializationCancellation,
    ) -> Result<Option<PhysicalBackupMaterializationProgress>, PhysicalBackupMaterializationDenial>
    {
        self.advance_boundary_observing(Some(cancellation))
    }

    fn advance_legacy(
        &mut self,
        cancellation: Option<&crate::PhysicalBackupMaterializationCancellation>,
    ) -> Result<bool, PhysicalBackupMaterializationDenial> {
        loop {
            match self.advance_boundary_observing(cancellation)? {
                Some(PhysicalBackupMaterializationProgress::BytesCopied(_)) => return Ok(true),
                Some(PhysicalBackupMaterializationProgress::ArtifactDurable(_)) => {}
                None => return Ok(false),
            }
        }
    }

    fn advance_boundary_observing(
        &mut self,
        cancellation: Option<&crate::PhysicalBackupMaterializationCancellation>,
    ) -> Result<Option<PhysicalBackupMaterializationProgress>, PhysicalBackupMaterializationDenial>
    {
        if self.published_recovery_sources.is_some() {
            return Ok(None);
        }
        let Some(progress) = self.sources.get_mut(self.source_index) else {
            return Ok(None);
        };
        reject_cancellation(cancellation)?;
        if progress.copied == progress.source.expected_bytes() {
            if <[u8; 32]>::from(progress.hasher.clone().finalize())
                != progress.source.expected_digest()
            {
                return Err(PhysicalBackupMaterializationDenial::SourceDigestMismatch {
                    path: progress.source.source_path().to_path_buf(),
                });
            }
            if !progress.durable {
                reject_cancellation(cancellation)?;
                let output_path = self.staging_root.join(progress.source.output_name());
                progress
                    .output
                    .sync_all()
                    .map_err(|error| io_denial(&output_path, error))?;
                progress.durable = true;
                add_counter(&mut self.counters.sync_operations, 1)?;
                add_counter(&mut self.counters.artifact_sync_operations, 1)?;
            }
            let durable = PhysicalBackupArtifactDurabilityProgress::new(
                self.source_index,
                progress.source.expected_bytes(),
            );
            self.source_index += 1;
            return Ok(Some(
                PhysicalBackupMaterializationProgress::ArtifactDurable(durable),
            ));
        }
        let remaining = progress.source.expected_bytes() - progress.copied;
        let read_budget = self
            .buffer
            .len()
            .min(usize::try_from(remaining).unwrap_or(usize::MAX));
        let bytes = &mut self.buffer[..read_budget];
        progress
            .input
            .read_exact(bytes)
            .map_err(|source| io_denial(progress.source.source_path(), source))?;
        progress
            .output
            .write_all(bytes)
            .map_err(|source| io_denial(&self.staging_root, source))?;
        progress.hasher.update(bytes);
        progress.copied += read_budget as u64;
        add_counter(&mut self.counters.source_bytes_read, read_budget as u64)?;
        add_counter(&mut self.counters.output_bytes_written, read_budget as u64)?;
        Ok(Some(PhysicalBackupMaterializationProgress::BytesCopied(
            PhysicalBackupCopyProgress::new(
                self.source_index,
                read_budget as u64,
                progress.copied,
                progress.source.expected_bytes(),
            ),
        )))
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
            session_identity_guard,
            ..
        } = self;
        drop(sources);
        drop(buffer);
        let result = abandon_physical_materialization(staging_root, final_root);
        drop(session_identity_guard);
        result
    }

    pub fn publish(
        self,
        manifest_bytes: &[u8],
    ) -> Result<PhysicalMaterializedBackupBundle, PhysicalBackupMaterializationDenial> {
        let mut manifest = Vec::new();
        manifest
            .try_reserve_exact(manifest_bytes.len())
            .map_err(|_| PhysicalBackupMaterializationDenial::InvalidBufferBudget)?;
        manifest.extend_from_slice(manifest_bytes);
        self.begin_publication(manifest)?.finish()
    }

    pub fn begin_publication(
        mut self,
        manifest_bytes: Vec<u8>,
    ) -> Result<PhysicalBackupPublicationSession, PhysicalBackupMaterializationDenial> {
        if manifest_bytes.is_empty() {
            return Err(
                PhysicalBackupMaterializationDenial::ExistingPublicationMismatch {
                    path: self.staging_root.join("backup.manifest"),
                },
            );
        }
        if self.advance()? {
            return Err(PhysicalBackupMaterializationDenial::IncompleteSources);
        }
        let recovered_publication = self.published_recovery_sources.is_some();
        let sources = match self.published_recovery_sources.take() {
            Some(sources) => sources,
            None => {
                let mut sources = Vec::new();
                sources.try_reserve_exact(self.sources.len()).map_err(|_| {
                    PhysicalBackupMaterializationDenial::SourceCollectionAllocationFailed
                })?;
                sources.extend(self.sources.drain(..).map(|progress| progress.source));
                sources
            }
        };
        PhysicalBackupPublicationSession::new(PhysicalBackupPublicationOpening {
            staging_root: self.staging_root,
            final_root: self.final_root,
            sources,
            buffer: self.buffer,
            manifest_bytes,
            counters: self.counters,
            session_identity_guard: self.session_identity_guard,
            recovered_publication,
        })
    }

    pub const fn counters(&self) -> PhysicalBackupMaterializationCounters {
        self.counters
    }
}

fn open_read(path: &Path) -> Result<File, PhysicalBackupMaterializationDenial> {
    File::open(path).map_err(|source| io_denial(path, source))
}
fn io_denial(path: &Path, source: std::io::Error) -> PhysicalBackupMaterializationDenial {
    PhysicalBackupMaterializationDenial::Io {
        path: path.to_path_buf(),
        source,
    }
}

fn add_counter(counter: &mut u64, delta: u64) -> Result<(), PhysicalBackupMaterializationDenial> {
    *counter = counter
        .checked_add(delta)
        .ok_or(PhysicalBackupMaterializationDenial::CounterOverflow)?;
    Ok(())
}

fn reject_cancellation(
    cancellation: Option<&crate::PhysicalBackupMaterializationCancellation>,
) -> Result<(), PhysicalBackupMaterializationDenial> {
    if cancellation.is_some_and(crate::PhysicalBackupMaterializationCancellation::is_cancelled) {
        Err(PhysicalBackupMaterializationDenial::Cancelled)
    } else {
        Ok(())
    }
}

use std::path::{Path, PathBuf};

use super::{io_denial, reject_symbolic_link, PhysicalBackupMaterializationDenial};

#[derive(Debug)]
pub struct PhysicalBackupMaterializationAbandonment {
    incomplete_root: PathBuf,
    incomplete_output_removed: bool,
    completed_bundle_retained: bool,
    directory_sync_operations: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingPhysicalBackupMaterializationCleanup {
    staging_root: PathBuf,
    final_root: PathBuf,
}

#[derive(Debug)]
pub struct PhysicalBackupMaterializationAbandonmentDenial {
    cleanup: PendingPhysicalBackupMaterializationCleanup,
    source: PhysicalBackupMaterializationDenial,
}

impl PhysicalBackupMaterializationAbandonment {
    pub fn incomplete_root(&self) -> &Path {
        &self.incomplete_root
    }

    pub const fn incomplete_output_removed(&self) -> bool {
        self.incomplete_output_removed
    }

    pub const fn completed_bundle_retained(&self) -> bool {
        self.completed_bundle_retained
    }

    pub const fn directory_sync_operations(&self) -> u64 {
        self.directory_sync_operations
    }
}

impl PendingPhysicalBackupMaterializationCleanup {
    pub fn retry(
        self,
    ) -> Result<
        PhysicalBackupMaterializationAbandonment,
        PhysicalBackupMaterializationAbandonmentDenial,
    > {
        abandon_pending_cleanup(self)
    }

    pub fn incomplete_root(&self) -> &Path {
        &self.staging_root
    }
}

impl PhysicalBackupMaterializationAbandonmentDenial {
    pub fn into_retry(
        self,
    ) -> (
        PendingPhysicalBackupMaterializationCleanup,
        PhysicalBackupMaterializationDenial,
    ) {
        (self.cleanup, self.source)
    }
}

pub(super) fn abandon_physical_materialization(
    staging_root: PathBuf,
    final_root: PathBuf,
) -> Result<PhysicalBackupMaterializationAbandonment, PhysicalBackupMaterializationAbandonmentDenial>
{
    abandon_pending_cleanup(PendingPhysicalBackupMaterializationCleanup {
        staging_root,
        final_root,
    })
}

fn abandon_pending_cleanup(
    cleanup: PendingPhysicalBackupMaterializationCleanup,
) -> Result<PhysicalBackupMaterializationAbandonment, PhysicalBackupMaterializationAbandonmentDenial>
{
    remove_incomplete_materialization(&cleanup.staging_root, &cleanup.final_root)
        .map_err(|source| PhysicalBackupMaterializationAbandonmentDenial { cleanup, source })
}

fn remove_incomplete_materialization(
    staging_root: &Path,
    final_root: &Path,
) -> Result<PhysicalBackupMaterializationAbandonment, PhysicalBackupMaterializationDenial> {
    let parent = staging_root.parent().ok_or_else(|| {
        io_denial(
            staging_root,
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "staging root has no parent",
            ),
        )
    })?;
    let incomplete_output_removed = if staging_root.exists() {
        reject_symbolic_link(staging_root)?;
        std::fs::remove_dir_all(staging_root).map_err(|source| io_denial(staging_root, source))?;
        crate::directory_durability::sync_directory(parent)
            .map_err(|source| io_denial(parent, source))?;
        true
    } else {
        false
    };
    Ok(PhysicalBackupMaterializationAbandonment {
        incomplete_root: staging_root.to_path_buf(),
        incomplete_output_removed,
        completed_bundle_retained: final_root.exists(),
        directory_sync_operations: u64::from(incomplete_output_removed),
    })
}

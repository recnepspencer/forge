use std::path::Path;

use super::{io_denial, PhysicalBackupMaterializationDenial};
use crate::PhysicalBackupSource;

pub(super) fn collect_sources(
    sources: impl IntoIterator<Item = PhysicalBackupSource>,
) -> Result<Vec<PhysicalBackupSource>, PhysicalBackupMaterializationDenial> {
    let sources = sources.into_iter();
    let mut collected = Vec::new();
    collected
        .try_reserve_exact(sources.size_hint().0)
        .map_err(|_| PhysicalBackupMaterializationDenial::SourceCollectionAllocationFailed)?;
    for source in sources {
        if collected.len() == collected.capacity() {
            collected.try_reserve(1).map_err(|_| {
                PhysicalBackupMaterializationDenial::SourceCollectionAllocationFailed
            })?;
        }
        collected.push(source);
    }
    Ok(collected)
}

pub(super) fn validate_source_set(
    sources: &[PhysicalBackupSource],
) -> Result<(), PhysicalBackupMaterializationDenial> {
    if sources.is_empty() {
        return Err(PhysicalBackupMaterializationDenial::EmptySourceSet);
    }
    let mut names = Vec::new();
    names
        .try_reserve_exact(sources.len())
        .map_err(|_| PhysicalBackupMaterializationDenial::SourceCollectionAllocationFailed)?;
    names.extend(sources.iter().map(PhysicalBackupSource::output_name));
    names.sort_unstable_by(|left, right| {
        left.bytes()
            .map(|byte| byte.to_ascii_lowercase())
            .cmp(right.bytes().map(|byte| byte.to_ascii_lowercase()))
    });
    for name in &names {
        if matches!(
            *name,
            "." | ".." | "backup.manifest" | "backup.manifest.pending" | "materialization.session"
        ) {
            return Err(PhysicalBackupMaterializationDenial::ReservedOutputName {
                output_name: (*name).to_owned(),
            });
        }
    }
    if let Some(pair) = names
        .windows(2)
        .find(|pair| pair[0].eq_ignore_ascii_case(pair[1]))
    {
        return Err(PhysicalBackupMaterializationDenial::DuplicateOutputName {
            output_name: pair[0].to_owned(),
        });
    }
    Ok(())
}

pub(super) fn allocate_buffer(
    buffer_bytes: usize,
) -> Result<Vec<u8>, PhysicalBackupMaterializationDenial> {
    let mut buffer = Vec::new();
    buffer
        .try_reserve_exact(buffer_bytes)
        .map_err(|_| PhysicalBackupMaterializationDenial::InvalidBufferBudget)?;
    buffer.resize(buffer_bytes, 0);
    Ok(buffer)
}

pub(super) fn reject_symbolic_link(path: &Path) -> Result<(), PhysicalBackupMaterializationDenial> {
    let metadata = std::fs::symlink_metadata(path).map_err(|source| io_denial(path, source))?;
    if metadata.file_type().is_symlink() {
        Err(
            PhysicalBackupMaterializationDenial::SymbolicLinkUnsupported {
                path: path.to_path_buf(),
            },
        )
    } else {
        Ok(())
    }
}

pub(super) fn reject_symbolic_link_if_present(
    path: &Path,
) -> Result<(), PhysicalBackupMaterializationDenial> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(
            PhysicalBackupMaterializationDenial::SymbolicLinkUnsupported {
                path: path.to_path_buf(),
            },
        ),
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(io_denial(path, source)),
    }
}

use std::fs::File;
use std::io::Read;
use std::path::Path;

use sha2::{Digest, Sha256};

use super::{
    add_counter, io_denial, reject_symbolic_link, PhysicalBackupMaterializationCounters,
    PhysicalBackupMaterializationDenial,
};
use crate::PhysicalBackupSource;

pub(super) fn validate_final_bundle(
    root: &Path,
    sources: &[PhysicalBackupSource],
    manifest_bytes: &[u8],
    buffer: &mut [u8],
    counters: &mut PhysicalBackupMaterializationCounters,
) -> Result<(), PhysicalBackupMaterializationDenial> {
    if !root.is_dir() {
        return Err(
            PhysicalBackupMaterializationDenial::ExistingPublicationMismatch {
                path: root.to_path_buf(),
            },
        );
    }
    validate_exact_final_component_names(root, sources)?;
    require_exact_file(
        &root.join("backup.manifest"),
        manifest_bytes,
        buffer,
        counters,
    )?;
    validate_source_payloads(root, sources, buffer, counters)
}

pub(super) fn validate_recovered_staging_bundle(
    root: &Path,
    sources: &[PhysicalBackupSource],
    buffer: &mut [u8],
    counters: &mut PhysicalBackupMaterializationCounters,
) -> Result<(), PhysicalBackupMaterializationDenial> {
    validate_exact_staging_component_names(root, sources)?;
    validate_source_payloads(root, sources, buffer, counters)
}

fn validate_source_payloads(
    root: &Path,
    sources: &[PhysicalBackupSource],
    buffer: &mut [u8],
    counters: &mut PhysicalBackupMaterializationCounters,
) -> Result<(), PhysicalBackupMaterializationDenial> {
    for source in sources {
        let path = root.join(source.output_name());
        reject_symbolic_link(&path)?;
        let mut file = File::open(&path).map_err(|error| io_denial(&path, error))?;
        let actual = file
            .metadata()
            .map_err(|error| io_denial(&path, error))?
            .len();
        if actual != source.expected_bytes() {
            return Err(PhysicalBackupMaterializationDenial::ExistingPublicationMismatch { path });
        }
        let mut digest = Sha256::new();
        let mut remaining = actual;
        while remaining > 0 {
            let width = buffer
                .len()
                .min(usize::try_from(remaining).unwrap_or(usize::MAX));
            file.read_exact(&mut buffer[..width])
                .map_err(|error| io_denial(&path, error))?;
            digest.update(&buffer[..width]);
            remaining -= width as u64;
            add_counter(&mut counters.resume_validation_bytes, width as u64)?;
        }
        if <[u8; 32]>::from(digest.finalize()) != source.expected_digest() {
            return Err(PhysicalBackupMaterializationDenial::ExistingPublicationMismatch { path });
        }
    }
    Ok(())
}

pub(super) fn require_exact_file(
    path: &Path,
    expected: &[u8],
    buffer: &mut [u8],
    counters: &mut PhysicalBackupMaterializationCounters,
) -> Result<(), PhysicalBackupMaterializationDenial> {
    reject_symbolic_link(path)?;
    let mut file = File::open(path).map_err(|source| io_denial(path, source))?;
    let actual = file
        .metadata()
        .map_err(|source| io_denial(path, source))?
        .len();
    if actual != expected.len() as u64 {
        return Err(
            PhysicalBackupMaterializationDenial::ExistingPublicationMismatch {
                path: path.to_path_buf(),
            },
        );
    }
    let mut offset = 0_usize;
    while offset < expected.len() {
        let width = buffer.len().min(expected.len() - offset);
        file.read_exact(&mut buffer[..width])
            .map_err(|source| io_denial(path, source))?;
        add_counter(&mut counters.resume_validation_bytes, width as u64)?;
        if buffer[..width] != expected[offset..offset + width] {
            return Err(
                PhysicalBackupMaterializationDenial::ExistingPublicationMismatch {
                    path: path.to_path_buf(),
                },
            );
        }
        offset += width;
    }
    Ok(())
}

fn validate_exact_final_component_names(
    root: &Path,
    sources: &[PhysicalBackupSource],
) -> Result<(), PhysicalBackupMaterializationDenial> {
    validate_component_names(root, sources, false)
}

fn validate_exact_staging_component_names(
    root: &Path,
    sources: &[PhysicalBackupSource],
) -> Result<(), PhysicalBackupMaterializationDenial> {
    validate_component_names(root, sources, true)
}

fn validate_component_names(
    root: &Path,
    sources: &[PhysicalBackupSource],
    allow_publication_state: bool,
) -> Result<(), PhysicalBackupMaterializationDenial> {
    let descriptor_exists =
        allow_publication_state && root.join("materialization.session").exists();
    let pending_exists = allow_publication_state && root.join("backup.manifest.pending").exists();
    let published_exists = root.join("backup.manifest").exists();
    let expected = sources
        .len()
        .checked_add(usize::from(descriptor_exists))
        .and_then(|count| count.checked_add(usize::from(pending_exists)))
        .and_then(|count| count.checked_add(usize::from(published_exists)))
        .ok_or(PhysicalBackupMaterializationDenial::CounterOverflow)?;
    let mut observed = 0_usize;
    for entry in std::fs::read_dir(root).map_err(|source| io_denial(root, source))? {
        let entry = entry.map_err(|source| io_denial(root, source))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|source| io_denial(&path, source))?;
        let file_name = entry.file_name();
        let Some(name) = file_name.to_str() else {
            return Err(PhysicalBackupMaterializationDenial::ExistingPublicationMismatch { path });
        };
        let declared_source = sources.iter().any(|source| source.output_name() == name);
        let declared_state = allow_publication_state
            && matches!(
                name,
                "materialization.session" | "backup.manifest.pending" | "backup.manifest"
            );
        let declared_final_manifest = !allow_publication_state && name == "backup.manifest";
        if file_type.is_symlink()
            || !file_type.is_file()
            || (!declared_source && !declared_state && !declared_final_manifest)
        {
            return Err(PhysicalBackupMaterializationDenial::ExistingPublicationMismatch { path });
        }
        observed = observed
            .checked_add(1)
            .ok_or(PhysicalBackupMaterializationDenial::CounterOverflow)?;
    }
    if observed != expected {
        return Err(
            PhysicalBackupMaterializationDenial::ExistingPublicationMismatch {
                path: root.to_path_buf(),
            },
        );
    }
    Ok(())
}

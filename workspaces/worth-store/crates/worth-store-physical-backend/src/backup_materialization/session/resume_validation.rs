use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

use sha2::{Digest, Sha256};

use super::{
    add_counter, io_denial, open_read, PhysicalBackupMaterializationCounters,
    PhysicalBackupMaterializationDenial,
};
use crate::PhysicalBackupSource;

pub(super) fn validate_source_identity_and_length(
    source: &PhysicalBackupSource,
) -> Result<u64, PhysicalBackupMaterializationDenial> {
    let physical_identity = crate::offline_media::physical_file_identity(source.source_path())
        .map_err(|error| match error {
            crate::OfflineMediaReadDenial::Io { path, source } => io_denial(&path, source),
            _ => PhysicalBackupMaterializationDenial::SourceIdentityMismatch {
                path: source.source_path().to_path_buf(),
            },
        })?;
    if physical_identity != source.expected_physical_identity() {
        return Err(
            PhysicalBackupMaterializationDenial::SourceIdentityMismatch {
                path: source.source_path().to_path_buf(),
            },
        );
    }
    let actual = std::fs::metadata(source.source_path())
        .map_err(|error| io_denial(source.source_path(), error))?
        .len();
    if actual != source.expected_bytes() {
        return Err(PhysicalBackupMaterializationDenial::SourceLengthMismatch {
            path: source.source_path().to_path_buf(),
            expected: source.expected_bytes(),
            actual,
        });
    }
    Ok(actual)
}

pub(super) fn validate_resume_prefix(
    source: &PhysicalBackupSource,
    output: &Path,
    copied: u64,
    buffer: &mut [u8],
    hasher: &mut Sha256,
    counters: &mut PhysicalBackupMaterializationCounters,
) -> Result<u64, PhysicalBackupMaterializationDenial> {
    let mut source_file = open_read(source.source_path())?;
    let mut output_file = open_read(output)?;
    let mut offset = 0u64;
    while offset < copied {
        let width = buffer
            .len()
            .min(usize::try_from(copied - offset).unwrap_or(usize::MAX));
        let bytes = &mut buffer[..width];
        source_file
            .read_exact(bytes)
            .map_err(|error| io_denial(source.source_path(), error))?;
        let source_chunk_digest = Sha256::digest(&*bytes);
        let hasher_before_chunk = hasher.clone();
        hasher.update(&*bytes);
        output_file
            .read_exact(bytes)
            .map_err(|error| io_denial(output, error))?;
        add_counter(&mut counters.source_bytes_read, width as u64)?;
        let compared_bytes = (width as u64)
            .checked_mul(2)
            .ok_or(PhysicalBackupMaterializationDenial::CounterOverflow)?;
        add_counter(&mut counters.resume_validation_bytes, compared_bytes)?;
        counters.peak_buffer_bytes = counters.peak_buffer_bytes.max(width as u64);
        if Sha256::digest(&*bytes) != source_chunk_digest {
            *hasher = hasher_before_chunk;
            let file = OpenOptions::new()
                .write(true)
                .open(output)
                .map_err(|error| io_denial(output, error))?;
            file.set_len(offset)
                .map_err(|error| io_denial(output, error))?;
            file.sync_all().map_err(|error| io_denial(output, error))?;
            add_counter(&mut counters.sync_operations, 1)?;
            return Ok(offset);
        }
        offset += width as u64;
    }
    Ok(copied)
}

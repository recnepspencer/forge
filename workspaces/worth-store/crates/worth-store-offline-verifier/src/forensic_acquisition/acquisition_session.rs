use std::io::Write;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use worth_store_physical_backend::{OfflineMediaReadDenial, ReadOnlyOfflineMediaCapability};

use super::{
    ForensicBundle, ForensicBundleRange, ForensicCustodyRecord, ForensicRangePosture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForensicAcquisitionRequest {
    target_root: PathBuf,
    observer_identity: String,
    acquisition_method: String,
    resident_buffer_bytes: usize,
}

impl ForensicAcquisitionRequest {
    pub fn new(
        target_root: impl Into<PathBuf>,
        observer_identity: impl Into<String>,
        acquisition_method: impl Into<String>,
        resident_buffer_bytes: usize,
    ) -> Result<Self, ForensicAcquisitionDenial> {
        let observer_identity = observer_identity.into();
        let acquisition_method = acquisition_method.into();
        if observer_identity.is_empty() || acquisition_method.is_empty() {
            return Err(ForensicAcquisitionDenial::InvalidCustody);
        }
        if resident_buffer_bytes == 0 {
            return Err(ForensicAcquisitionDenial::InvalidBufferBudget);
        }
        Ok(Self {
            target_root: target_root.into(),
            observer_identity,
            acquisition_method,
            resident_buffer_bytes,
        })
    }
}

#[derive(Debug)]
pub enum ForensicAcquisitionDenial {
    InvalidCustody,
    InvalidBufferBudget,
    TargetOverlapsSource,
    TargetAlreadyContainsConflict,
    Media(OfflineMediaReadDenial),
    Io(std::io::Error),
}

impl From<std::io::Error> for ForensicAcquisitionDenial {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForensicAcquisitionCounters {
    source_files: u64,
    source_bytes_read: u64,
    output_bytes_written: u64,
    unreadable_ranges: u64,
    maximum_resident_buffer_bytes: u64,
}

impl ForensicAcquisitionCounters {
    pub const fn source_files(self) -> u64 {
        self.source_files
    }

    pub const fn source_bytes_read(self) -> u64 {
        self.source_bytes_read
    }

    pub const fn output_bytes_written(self) -> u64 {
        self.output_bytes_written
    }

    pub const fn unreadable_ranges(self) -> u64 {
        self.unreadable_ranges
    }

    pub const fn maximum_resident_buffer_bytes(self) -> u64 {
        self.maximum_resident_buffer_bytes
    }
}

#[derive(Debug)]
pub struct ForensicAcquisitionSession {
    request: ForensicAcquisitionRequest,
    media: ReadOnlyOfflineMediaCapability,
}

impl ForensicAcquisitionSession {
    pub fn open(
        request: ForensicAcquisitionRequest,
        media: ReadOnlyOfflineMediaCapability,
    ) -> Result<Self, ForensicAcquisitionDenial> {
        reject_target_source_overlap(&request, &media)?;
        Ok(Self { request, media })
    }

    pub fn acquire(
        mut self,
    ) -> Result<(ForensicBundle, ForensicAcquisitionCounters), ForensicAcquisitionDenial> {
        std::fs::create_dir_all(&self.request.target_root)?;
        let mut buffer = vec![0; self.request.resident_buffer_bytes];
        let mut ranges = Vec::new();
        ranges
            .try_reserve_exact(self.media.file_count())
            .map_err(|_| ForensicAcquisitionDenial::InvalidBufferBudget)?;
        let mut source_fingerprints = Vec::new();
        source_fingerprints
            .try_reserve_exact(self.media.file_count())
            .map_err(|_| ForensicAcquisitionDenial::InvalidBufferBudget)?;
        let mut counters = ForensicAcquisitionCounters {
            source_files: self.media.file_count() as u64,
            source_bytes_read: 0,
            output_bytes_written: 0,
            unreadable_ranges: 0,
            maximum_resident_buffer_bytes: buffer.len() as u64,
        };
        for source_index in 0..self.media.file_count() {
            let (source_length, source_fingerprint) = {
                let source = self.media.file(source_index).expect("bounded source index");
                (source.length(), source.metadata_fingerprint())
            };
            source_fingerprints.push(source_fingerprint);
            let range = acquire_source_file(
                &mut self.media,
                source_index,
                source_length,
                &self.request.target_root,
                &mut buffer,
                &mut counters,
            )?;
            ranges.push(range);
        }
        self.media
            .revalidate_consistency()
            .map_err(ForensicAcquisitionDenial::Media)?;
        let custody = ForensicCustodyRecord {
            observer_identity: self.request.observer_identity,
            acquisition_method: self.request.acquisition_method,
            consistency_basis_identity: Sha256::digest(self.media.basis().identity().as_bytes()).into(),
            source_media_fingerprints: source_fingerprints,
        };
        let bundle_identity = forensic_bundle_identity(&custody, &ranges);
        persist_manifest(&self.request.target_root, bundle_identity, &ranges)?;
        Ok((
            ForensicBundle {
                root: self.request.target_root,
                bundle_identity,
                ranges,
                custody,
            },
            counters,
        ))
    }
}

fn acquire_source_file(
    media: &mut ReadOnlyOfflineMediaCapability,
    source_index: usize,
    source_length: u64,
    target_root: &Path,
    buffer: &mut [u8],
    counters: &mut ForensicAcquisitionCounters,
) -> Result<ForensicBundleRange, ForensicAcquisitionDenial> {
    let output_name = format!("evidence-{source_index:08}.bin");
    let pending_path = target_root.join(format!(".{output_name}.pending"));
    let final_path = target_root.join(&output_name);
    if pending_path.exists() || final_path.exists() {
        return Err(ForensicAcquisitionDenial::TargetAlreadyContainsConflict);
    }
    let mut output = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&pending_path)?;
    let mut digest = Sha256::new();
    let mut offset = 0_u64;
    while offset < source_length {
        let observation = match media.read_bounded_into(source_index, offset, buffer) {
            Ok(observation) => observation,
            Err(_denial) => {
                drop(output);
                std::fs::remove_file(&pending_path)?;
                counters.unreadable_ranges += 1;
                return Ok(ForensicBundleRange {
                    source_index,
                    source_offset: offset,
                    byte_length: source_length.saturating_sub(offset),
                    output_name: None,
                    digest: None,
                    posture: ForensicRangePosture::Unreadable,
                });
            }
        };
        let read = observation.bytes_read();
        output.write_all(&buffer[..read])?;
        digest.update(&buffer[..read]);
        offset = offset.saturating_add(read as u64);
        counters.source_bytes_read = counters.source_bytes_read.saturating_add(read as u64);
        counters.output_bytes_written = counters.output_bytes_written.saturating_add(read as u64);
    }
    output.sync_all()?;
    std::fs::rename(&pending_path, &final_path)?;
    sync_directory(target_root)?;
    Ok(ForensicBundleRange {
        source_index,
        source_offset: 0,
        byte_length: source_length,
        output_name: Some(output_name),
        digest: Some(digest.finalize().into()),
        posture: ForensicRangePosture::Acquired,
    })
}

fn reject_target_source_overlap(
    request: &ForensicAcquisitionRequest,
    media: &ReadOnlyOfflineMediaCapability,
) -> Result<(), ForensicAcquisitionDenial> {
    let target = absolute_path(&request.target_root)?;
    for index in 0..media.file_count() {
        let source = media.file(index).expect("bounded source index").path();
        let source = absolute_path(source)?;
        if source.starts_with(&target) || target.starts_with(&source) {
            return Err(ForensicAcquisitionDenial::TargetOverlapsSource);
        }
    }
    Ok(())
}

fn absolute_path(path: &Path) -> Result<PathBuf, std::io::Error> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn forensic_bundle_identity(
    custody: &ForensicCustodyRecord,
    ranges: &[ForensicBundleRange],
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-forensic-bundle-v1");
    digest.update(custody.observer_identity.as_bytes());
    digest.update(custody.acquisition_method.as_bytes());
    digest.update(custody.consistency_basis_identity);
    for fingerprint in &custody.source_media_fingerprints {
        digest.update(fingerprint);
    }
    for range in ranges {
        digest.update((range.source_index as u64).to_be_bytes());
        digest.update(range.source_offset.to_be_bytes());
        digest.update(range.byte_length.to_be_bytes());
        digest.update([range.posture as u8]);
        digest.update(range.digest.unwrap_or([0; 32]));
    }
    digest.finalize().into()
}

fn persist_manifest(
    root: &Path,
    identity: [u8; 32],
    ranges: &[ForensicBundleRange],
) -> Result<(), ForensicAcquisitionDenial> {
    let mut manifest = Vec::new();
    manifest.extend_from_slice(b"WORTHFORENSIC1\n");
    manifest.extend_from_slice(&identity);
    manifest.extend_from_slice(&(ranges.len() as u64).to_be_bytes());
    let path = root.join("forensic.manifest");
    let mut file = std::fs::OpenOptions::new().create_new(true).write(true).open(path)?;
    file.write_all(&manifest)?;
    file.sync_all()?;
    sync_directory(root)?;
    Ok(())
}

#[cfg(windows)]
fn sync_directory(path: &Path) -> std::io::Result<()> {
    use std::os::windows::fs::OpenOptionsExt;
    std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(0x0200_0000)
        .open(path)?
        .sync_all()
}

#[cfg(not(windows))]
fn sync_directory(path: &Path) -> std::io::Result<()> {
    std::fs::File::open(path)?.sync_all()
}

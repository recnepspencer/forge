use std::io::{Read, Write};
use std::path::Path;

use sha2::{Digest, Sha256};
use worth_store_physical_format::{PageGenerationCell, PhysicalReference, PhysicalReferenceKind};

use super::CheckpointManifest;

const MAGIC: &[u8; 8] = b"WORTHCKP";
const VERSION: u16 = 1;
const HEADER_BYTES: usize = 78;
const PAGE_ROW_BYTES: usize = 32;
const FOOTER_BYTES: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointBackupArtifact {
    checkpoint_identity: String,
    manifest_generation: u64,
    durable_checkpoint_lsn: u64,
    root: PhysicalReference,
    covered_lsn: (u64, u64),
    redo_lsn: u64,
    pages: Vec<(PageGenerationCell, u64)>,
}

impl CheckpointBackupArtifact {
    pub fn from_sharp_manifest(
        manifest: &CheckpointManifest,
        manifest_generation: u64,
        durable_checkpoint_lsn: u64,
    ) -> Option<Self> {
        let root = manifest.root_posture().root_reference()?;
        let covered = manifest.covered_lsn_range().range();
        let covered_lsn = (covered.start().get(), covered.end_exclusive().get());
        let redo_lsn = manifest.redo_boundary().lsn().get();
        if root.kind() != PhysicalReferenceKind::RootPublication
            || manifest_generation == 0
            || durable_checkpoint_lsn < redo_lsn
            || durable_checkpoint_lsn > covered_lsn.1
        {
            return None;
        }
        let mut pages = manifest
            .page_lsn_frontier()
            .pages()
            .iter()
            .map(|(page, lsn)| (*page, lsn.lsn().get()))
            .collect::<Vec<_>>();
        pages.sort_by_key(|(page, _)| {
            (
                page.segment_id().get(),
                page.page_id().get(),
                page.generation().get(),
            )
        });
        let unique = pages.windows(2).all(|pair| pair[0].0 != pair[1].0);
        if pages.is_empty() || !unique {
            return None;
        }
        Some(Self {
            checkpoint_identity: manifest.checkpoint_id().digest().as_str().to_owned(),
            manifest_generation,
            durable_checkpoint_lsn,
            root,
            covered_lsn,
            redo_lsn,
            pages,
        })
    }

    pub fn checkpoint_identity(&self) -> &str {
        &self.checkpoint_identity
    }

    pub const fn manifest_generation(&self) -> u64 {
        self.manifest_generation
    }

    pub const fn durable_checkpoint_lsn(&self) -> u64 {
        self.durable_checkpoint_lsn
    }

    pub const fn root(&self) -> PhysicalReference {
        self.root
    }

    pub fn encode(&self, mut output: impl Write) -> Result<u64, std::io::Error> {
        let mut digest = Sha256::new();
        let identity = self.checkpoint_identity.as_bytes();
        let header = encode_header(self, identity.len())?;
        write_hashed(&mut output, &mut digest, &header)?;
        for (page, page_lsn) in &self.pages {
            let mut row = [0_u8; PAGE_ROW_BYTES];
            row[0..8].copy_from_slice(&page.segment_id().get().to_le_bytes());
            row[8..16].copy_from_slice(&page.page_id().get().to_le_bytes());
            row[16..24].copy_from_slice(&page.generation().get().to_le_bytes());
            row[24..32].copy_from_slice(&page_lsn.to_le_bytes());
            write_hashed(&mut output, &mut digest, &row)?;
        }
        write_hashed(&mut output, &mut digest, identity)?;
        output.write_all(&digest.finalize())?;
        Ok(HEADER_BYTES as u64
            + self.pages.len() as u64 * PAGE_ROW_BYTES as u64
            + identity.len() as u64
            + FOOTER_BYTES as u64)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoundedCheckpointBackupVerificationRequest<'a> {
    pub checkpoint_identity: &'a str,
    pub manifest_generation: u64,
    pub durable_checkpoint_lsn: u64,
    pub root_generation: u64,
    pub expected_bytes: u64,
    pub expected_digest: [u8; 32],
    pub max_buffer_bytes: usize,
}

#[derive(Debug)]
pub enum BoundedCheckpointBackupDenial {
    Io(std::io::Error),
    BufferTooSmall,
    AllocationFailed,
    LengthMismatch { expected: u64, actual: u64 },
    InvalidHeader,
    InvalidPageFrontier,
    BindingMismatch,
    InternalDigestMismatch,
    ArtifactDigestMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundedCheckpointBackupObservation {
    checkpoint_identity_digest: [u8; 32],
    manifest_generation: u64,
    durable_checkpoint_lsn: u64,
    root_generation: u64,
    page_count: u64,
    bytes_read: u64,
    decoder_allocation_bytes: u64,
    peak_buffer_bytes: u64,
    artifact_digest: [u8; 32],
}

impl BoundedCheckpointBackupObservation {
    pub const fn checkpoint_identity_digest(self) -> [u8; 32] {
        self.checkpoint_identity_digest
    }
    pub const fn manifest_generation(self) -> u64 {
        self.manifest_generation
    }
    pub const fn durable_checkpoint_lsn(self) -> u64 {
        self.durable_checkpoint_lsn
    }
    pub const fn root_generation(self) -> u64 {
        self.root_generation
    }
    pub const fn page_count(self) -> u64 {
        self.page_count
    }
    pub const fn bytes_read(self) -> u64 {
        self.bytes_read
    }
    pub const fn decoder_allocation_bytes(self) -> u64 {
        self.decoder_allocation_bytes
    }
    pub const fn peak_buffer_bytes(self) -> u64 {
        self.peak_buffer_bytes
    }
    pub const fn artifact_digest(self) -> [u8; 32] {
        self.artifact_digest
    }
}

pub fn verify_bounded_checkpoint_backup_artifact(
    path: &Path,
    request: BoundedCheckpointBackupVerificationRequest<'_>,
) -> Result<BoundedCheckpointBackupObservation, BoundedCheckpointBackupDenial> {
    let mut file = std::fs::File::open(path).map_err(BoundedCheckpointBackupDenial::Io)?;
    let actual = file
        .metadata()
        .map_err(BoundedCheckpointBackupDenial::Io)?
        .len();
    verify_bounded_checkpoint_backup_artifact_from_reader(&mut file, actual, request)
}

pub fn verify_bounded_checkpoint_backup_artifact_from_reader(
    reader: &mut impl Read,
    actual: u64,
    request: BoundedCheckpointBackupVerificationRequest<'_>,
) -> Result<BoundedCheckpointBackupObservation, BoundedCheckpointBackupDenial> {
    if request.max_buffer_bytes <= HEADER_BYTES + FOOTER_BYTES {
        return Err(BoundedCheckpointBackupDenial::BufferTooSmall);
    }
    if actual != request.expected_bytes {
        return Err(BoundedCheckpointBackupDenial::LengthMismatch {
            expected: request.expected_bytes,
            actual,
        });
    }
    let mut header = [0_u8; HEADER_BYTES];
    reader
        .read_exact(&mut header)
        .map_err(BoundedCheckpointBackupDenial::Io)?;
    let fields = decode_header(&header)?;
    if fields.manifest_generation != request.manifest_generation
        || fields.durable_lsn != request.durable_checkpoint_lsn
        || fields.root_generation != request.root_generation
        || fields.identity_bytes != request.checkpoint_identity.len() as u64
    {
        return Err(BoundedCheckpointBackupDenial::BindingMismatch);
    }
    let encoded_bytes = fields
        .page_count
        .checked_mul(PAGE_ROW_BYTES as u64)
        .and_then(|bytes| bytes.checked_add(HEADER_BYTES as u64))
        .and_then(|bytes| bytes.checked_add(fields.identity_bytes))
        .and_then(|bytes| bytes.checked_add(FOOTER_BYTES as u64))
        .ok_or(BoundedCheckpointBackupDenial::InvalidHeader)?;
    if encoded_bytes != actual {
        return Err(BoundedCheckpointBackupDenial::InvalidHeader);
    }

    let mut internal_digest = Sha256::new();
    let mut artifact_digest = Sha256::new();
    internal_digest.update(header);
    artifact_digest.update(header);
    let mut previous_page = None;
    for _ in 0..fields.page_count {
        let mut row = [0_u8; PAGE_ROW_BYTES];
        reader
            .read_exact(&mut row)
            .map_err(BoundedCheckpointBackupDenial::Io)?;
        internal_digest.update(row);
        artifact_digest.update(row);
        let page = (read_u64(&row, 0), read_u64(&row, 8), read_u64(&row, 16));
        let page_lsn = read_u64(&row, 24);
        if page.0 == 0
            || page.1 == 0
            || page.2 == 0
            || page_lsn < fields.redo_lsn
            || previous_page.is_some_and(|previous| previous >= page)
        {
            return Err(BoundedCheckpointBackupDenial::InvalidPageFrontier);
        }
        previous_page = Some(page);
    }

    let chunk_bytes = (request.max_buffer_bytes - HEADER_BYTES - FOOTER_BYTES).min(64 * 1024);
    let mut buffer = Vec::new();
    buffer
        .try_reserve_exact(chunk_bytes)
        .map_err(|_| BoundedCheckpointBackupDenial::AllocationFailed)?;
    buffer.resize(chunk_bytes, 0);
    let expected_identity = request.checkpoint_identity.as_bytes();
    let mut identity_offset = 0_usize;
    while identity_offset < expected_identity.len() {
        let take = (expected_identity.len() - identity_offset).min(chunk_bytes);
        reader
            .read_exact(&mut buffer[..take])
            .map_err(BoundedCheckpointBackupDenial::Io)?;
        if buffer[..take] != expected_identity[identity_offset..identity_offset + take] {
            return Err(BoundedCheckpointBackupDenial::BindingMismatch);
        }
        internal_digest.update(&buffer[..take]);
        artifact_digest.update(&buffer[..take]);
        identity_offset += take;
    }
    let mut footer = [0_u8; FOOTER_BYTES];
    reader
        .read_exact(&mut footer)
        .map_err(BoundedCheckpointBackupDenial::Io)?;
    if internal_digest.finalize()[..] != footer {
        return Err(BoundedCheckpointBackupDenial::InternalDigestMismatch);
    }
    artifact_digest.update(footer);
    if <[u8; 32]>::from(artifact_digest.finalize()) != request.expected_digest {
        return Err(BoundedCheckpointBackupDenial::ArtifactDigestMismatch);
    }
    Ok(BoundedCheckpointBackupObservation {
        checkpoint_identity_digest: Sha256::digest(request.checkpoint_identity.as_bytes()).into(),
        manifest_generation: request.manifest_generation,
        durable_checkpoint_lsn: request.durable_checkpoint_lsn,
        root_generation: request.root_generation,
        page_count: fields.page_count,
        bytes_read: actual,
        decoder_allocation_bytes: chunk_bytes as u64,
        peak_buffer_bytes: (HEADER_BYTES + FOOTER_BYTES + chunk_bytes) as u64,
        artifact_digest: request.expected_digest,
    })
}

#[derive(Debug, Clone, Copy)]
struct HeaderFields {
    manifest_generation: u64,
    durable_lsn: u64,
    root_generation: u64,
    redo_lsn: u64,
    page_count: u64,
    identity_bytes: u64,
}

fn encode_header(
    artifact: &CheckpointBackupArtifact,
    identity_bytes: usize,
) -> Result<[u8; HEADER_BYTES], std::io::Error> {
    let mut header = [0_u8; HEADER_BYTES];
    header[0..8].copy_from_slice(MAGIC);
    header[8..10].copy_from_slice(&VERSION.to_le_bytes());
    let values = [
        artifact.manifest_generation,
        artifact.durable_checkpoint_lsn,
        artifact
            .root
            .root_reference()
            .expect("validated root")
            .get(),
        artifact.root.generation().get(),
        artifact.covered_lsn.0,
        artifact.covered_lsn.1,
        artifact.redo_lsn,
        artifact.pages.len() as u64,
    ];
    for (index, value) in values.into_iter().enumerate() {
        let offset = 10 + index * 8;
        header[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
    }
    let identity_bytes = u32::try_from(identity_bytes)
        .map_err(|_| std::io::Error::other("checkpoint identity too large"))?;
    header[74..78].copy_from_slice(&identity_bytes.to_le_bytes());
    Ok(header)
}

fn decode_header(
    header: &[u8; HEADER_BYTES],
) -> Result<HeaderFields, BoundedCheckpointBackupDenial> {
    if &header[0..8] != MAGIC || read_u16(header, 8) != VERSION {
        return Err(BoundedCheckpointBackupDenial::InvalidHeader);
    }
    let root_reference = read_u64(header, 26);
    let covered_start = read_u64(header, 42);
    let covered_end = read_u64(header, 50);
    let fields = HeaderFields {
        manifest_generation: read_u64(header, 10),
        durable_lsn: read_u64(header, 18),
        root_generation: read_u64(header, 34),
        redo_lsn: read_u64(header, 58),
        page_count: read_u64(header, 66),
        identity_bytes: u64::from(read_u32(header, 74)),
    };
    if fields.manifest_generation == 0
        || root_reference == 0
        || fields.root_generation == 0
        || covered_start >= covered_end
        || fields.redo_lsn < covered_start
        || fields.redo_lsn >= covered_end
        || fields.durable_lsn < fields.redo_lsn
        || fields.durable_lsn > covered_end
        || fields.page_count == 0
        || fields.identity_bytes == 0
    {
        return Err(BoundedCheckpointBackupDenial::InvalidHeader);
    }
    Ok(fields)
}

fn write_hashed(
    output: &mut impl Write,
    digest: &mut Sha256,
    bytes: &[u8],
) -> Result<(), std::io::Error> {
    output.write_all(bytes)?;
    digest.update(bytes);
    Ok(())
}

fn read_u16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(bytes[offset..offset + 2].try_into().expect("fixed field"))
}
fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("fixed field"))
}
fn read_u64(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().expect("fixed field"))
}

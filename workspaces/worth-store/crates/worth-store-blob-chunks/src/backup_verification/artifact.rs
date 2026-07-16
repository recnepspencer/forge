use std::io::Write;

use sha2::{Digest, Sha256};

use crate::{
    chunk_integrity::{stable_digest_for, stable_digest_for_bytes},
    BlobChunkByteRange, BlobChunkIntegrityProof, BlobChunkOrdinal,
    BlobChunkSecurityMetadataWitness,
};

pub(super) const MAGIC: &[u8; 8] = b"WORTHBLB";
pub(super) const VERSION: u16 = 1;
pub(super) const HEADER_BYTES: usize = 57;
pub(super) const FOOTER_BYTES: usize = 32;
pub(super) const RULE_VERSION: &str = "s7.fixed-size.raw-chunk.v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobBackupChunkArtifact {
    pub(super) ordinal: u64,
    pub(super) range_start: u64,
    pub(super) payload: Vec<u8>,
    pub(super) checksum_digest: String,
    pub(super) stored_digest: String,
    pub(super) content_digest: String,
    pub(super) chunk_identity: String,
    pub(super) security: [u8; 5],
}

impl BlobBackupChunkArtifact {
    pub fn from_integrity_proof(proof: &BlobChunkIntegrityProof, bytes: &[u8]) -> Option<Self> {
        let range = proof.byte_range();
        if range.len() != bytes.len() as u64
            || proof.rule_version() != RULE_VERSION
            || expected_checksum(bytes) != proof.checksum().checksum().digest().as_str()
            || expected_content(proof.ordinal(), range, bytes)
                != proof.content_digest().digest().as_str()
            || expected_stored(
                proof.ordinal(),
                range,
                proof.checksum().checksum().digest().as_str(),
            ) != proof.stored_digest().digest().as_str()
            || expected_identity(
                proof.ordinal(),
                range,
                proof.stored_digest().digest().as_str(),
                proof.security_metadata(),
            ) != proof.identity().chunk_digest().as_str()
        {
            return None;
        }
        Some(Self {
            ordinal: proof.ordinal().get(),
            range_start: range.start(),
            payload: bytes.to_vec(),
            checksum_digest: proof.checksum().checksum().digest().as_str().to_owned(),
            stored_digest: proof.stored_digest().digest().as_str().to_owned(),
            content_digest: proof.content_digest().digest().as_str().to_owned(),
            chunk_identity: proof.identity().chunk_digest().as_str().to_owned(),
            security: security_codes(proof.security_metadata()),
        })
    }

    pub fn chunk_identity(&self) -> &str {
        &self.chunk_identity
    }

    pub fn encode(&self, mut output: impl Write) -> Result<u64, std::io::Error> {
        let fields = [
            RULE_VERSION.as_bytes(),
            self.checksum_digest.as_bytes(),
            self.stored_digest.as_bytes(),
            self.content_digest.as_bytes(),
            self.chunk_identity.as_bytes(),
        ];
        let header = encode_header(self, fields.map(<[u8]>::len))?;
        let mut digest = Sha256::new();
        write_hashed(&mut output, &mut digest, &header)?;
        for field in fields {
            write_hashed(&mut output, &mut digest, field)?;
        }
        write_hashed(&mut output, &mut digest, &self.payload)?;
        output.write_all(&digest.finalize())?;
        Ok(HEADER_BYTES as u64
            + fields.iter().map(|field| field.len() as u64).sum::<u64>()
            + self.payload.len() as u64
            + FOOTER_BYTES as u64)
    }
}

pub(super) fn expected_checksum(bytes: &[u8]) -> String {
    format!("fnv64:{:016x}", fnv64(0xcbf2_9ce4_8422_2325, bytes))
}

fn expected_content(ordinal: BlobChunkOrdinal, range: BlobChunkByteRange, bytes: &[u8]) -> String {
    stable_digest_for_bytes("content", RULE_VERSION, ordinal, range, bytes)
        .as_str()
        .to_owned()
}

fn expected_stored(ordinal: BlobChunkOrdinal, range: BlobChunkByteRange, checksum: &str) -> String {
    stable_digest_for("stored", RULE_VERSION, ordinal, range, checksum)
        .as_str()
        .to_owned()
}

fn expected_identity(
    ordinal: BlobChunkOrdinal,
    range: BlobChunkByteRange,
    stored: &str,
    security: BlobChunkSecurityMetadataWitness,
) -> String {
    stable_digest_for(
        "chunk",
        RULE_VERSION,
        ordinal,
        range,
        &identity_evidence(stored, security_codes(security)),
    )
    .as_str()
    .to_owned()
}

pub(super) fn identity_evidence(stored: &str, security: [u8; 5]) -> String {
    format!(
        "{}:{}:{}:{}:{}:{}",
        stored, security[0], security[1], security[2], security[3], security[4]
    )
}

const fn security_codes(metadata: BlobChunkSecurityMetadataWitness) -> [u8; 5] {
    [
        metadata.key_scope() as u8,
        metadata.key_version_posture() as u8,
        metadata.tenant_scope() as u8,
        match metadata.authenticity_requirement().class() {
            Some(class) => class as u8,
            None => 0,
        },
        metadata.custody_posture() as u8,
    ]
}

pub(super) fn fnv64(mut state: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        state ^= u64::from(*byte);
        state = state.wrapping_mul(0x0000_0100_0000_01b3);
    }
    state
}

fn encode_header(
    artifact: &BlobBackupChunkArtifact,
    lengths: [usize; 5],
) -> Result<[u8; HEADER_BYTES], std::io::Error> {
    let mut header = [0_u8; HEADER_BYTES];
    header[0..8].copy_from_slice(MAGIC);
    header[8..10].copy_from_slice(&VERSION.to_le_bytes());
    header[10..18].copy_from_slice(&artifact.ordinal.to_le_bytes());
    header[18..26].copy_from_slice(&artifact.range_start.to_le_bytes());
    header[26..34].copy_from_slice(&(artifact.payload.len() as u64).to_le_bytes());
    header[34..42].copy_from_slice(&(artifact.payload.len() as u64).to_le_bytes());
    for (index, length) in lengths.into_iter().enumerate() {
        let length = u16::try_from(length)
            .map_err(|_| std::io::Error::other("blob artifact field too large"))?;
        let offset = 42 + index * 2;
        header[offset..offset + 2].copy_from_slice(&length.to_le_bytes());
    }
    header[52..57].copy_from_slice(&artifact.security);
    Ok(header)
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

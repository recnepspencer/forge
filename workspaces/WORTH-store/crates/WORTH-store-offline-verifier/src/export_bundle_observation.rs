#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineExportChunkDeclaration {
    pub ordinal: u64,
    pub chunk_identity: String,
    pub stored_digest: String,
    pub checksum_digest: String,
    pub bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineExportDigestEvidence {
    pub logical_content_digest: String,
    pub export_bundle_digest: String,
    pub declaration_digest: String,
    pub declared_chunk_count: u64,
    pub declared_total_bytes: u64,
}

impl OfflineExportDigestEvidence {
    pub const fn evidence_item_count(&self) -> u64 {
        5
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineExportBundleObservation {
    declarations: Vec<OfflineExportChunkDeclaration>,
    total_bytes: u64,
    digest_evidence: OfflineExportDigestEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OfflineExportBundleObservationDenial {
    EmptyBundle,
    EmptyDigestField,
    NonCanonicalChunkOrdering,
    DigestEvidenceMismatch,
}

pub fn inspect_offline_export_bundle(
    declarations: impl IntoIterator<Item = OfflineExportChunkDeclaration>,
    digest_evidence: OfflineExportDigestEvidence,
) -> Result<OfflineExportBundleObservation, OfflineExportBundleObservationDenial> {
    let declarations: Vec<_> = declarations.into_iter().collect();
    if declarations.is_empty() {
        return Err(OfflineExportBundleObservationDenial::EmptyBundle);
    }
    if !digest_evidence.is_structurally_valid() {
        return Err(OfflineExportBundleObservationDenial::EmptyDigestField);
    }
    if declarations.iter().any(|chunk| {
        chunk.chunk_identity.trim().is_empty()
            || chunk.stored_digest.trim().is_empty()
            || chunk.checksum_digest.trim().is_empty()
            || !chunk.chunk_identity.starts_with("s7:")
            || !chunk.stored_digest.starts_with("s7:")
            || !chunk.checksum_digest.starts_with("fnv64:")
            || chunk.bytes == 0
    }) {
        return Err(OfflineExportBundleObservationDenial::EmptyDigestField);
    }
    if declarations
        .windows(2)
        .any(|pair| pair[1].ordinal != pair[0].ordinal + 1)
    {
        return Err(OfflineExportBundleObservationDenial::NonCanonicalChunkOrdering);
    }
    let total_bytes: u64 = declarations.iter().map(|chunk| chunk.bytes).sum();
    if digest_evidence.declared_chunk_count != declarations.len() as u64
        || digest_evidence.declared_total_bytes != total_bytes
        || digest_evidence.declaration_digest != declaration_digest(&declarations)
    {
        return Err(OfflineExportBundleObservationDenial::DigestEvidenceMismatch);
    }
    Ok(OfflineExportBundleObservation {
        total_bytes,
        digest_evidence,
        declarations,
    })
}

impl OfflineExportBundleObservation {
    pub fn declarations(&self) -> &[OfflineExportChunkDeclaration] {
        &self.declarations
    }

    pub const fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    pub fn digest_evidence(&self) -> &OfflineExportDigestEvidence {
        &self.digest_evidence
    }

    pub const fn digest_evidence_count(&self) -> u64 {
        self.digest_evidence.evidence_item_count()
    }
}

impl OfflineExportDigestEvidence {
    fn is_structurally_valid(&self) -> bool {
        stable_digest_like(&self.logical_content_digest)
            && canonical_export_digest_like(&self.export_bundle_digest)
            && stable_digest_like(&self.declaration_digest)
    }
}

fn stable_digest_like(value: &str) -> bool {
    let Some((algorithm, digest)) = value.split_once(':') else {
        return false;
    };
    !algorithm.trim().is_empty()
        && !digest.trim().is_empty()
        && !algorithm.contains(char::is_whitespace)
        && !digest.contains(char::is_whitespace)
}

fn canonical_export_digest_like(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn declaration_digest(declarations: &[OfflineExportChunkDeclaration]) -> String {
    let mut hash = stable_hash_bytes(0xcbf2_9ce4_8422_2325, b"phase19.export.declarations");
    let total_bytes: u64 = declarations.iter().map(|chunk| chunk.bytes).sum();
    for declaration in declarations {
        hash = stable_hash_u64(hash, declaration.ordinal);
        hash = stable_hash_bytes(hash, declaration.chunk_identity.as_bytes());
        hash = stable_hash_bytes(hash, declaration.stored_digest.as_bytes());
        hash = stable_hash_bytes(hash, declaration.checksum_digest.as_bytes());
        hash = stable_hash_u64(hash, declaration.bytes);
    }
    hash = stable_hash_u64(hash, total_bytes);
    hash = stable_hash_u64(hash, declarations.len() as u64);
    format!("s7:export-declarations:{hash:016x}")
}

fn stable_hash_u64(hash: u64, value: u64) -> u64 {
    stable_hash_bytes(hash, &value.to_le_bytes())
}

fn stable_hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

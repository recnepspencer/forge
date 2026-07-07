use forge_store_contracts::StableDigest;
use forge_store_physical_format::PhysicalChunkChecksumWitness;

use crate::{
    BlobChunkByteRange, BlobChunkByteWindow, BlobChunkContentDigest, BlobChunkIdentity,
    BlobChunkIntegrityCounterSnapshot, BlobChunkIntegrityDenial, BlobChunkOrdinal,
    BlobChunkSecurityMetadataWitness, BlobChunkingRuleAdmission, StoredChunkDigest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkIntegrityProof {
    identity: BlobChunkIdentity,
    stored_digest: StoredChunkDigest,
    content_digest: BlobChunkContentDigest,
    checksum: PhysicalChunkChecksumWitness,
    security_metadata: BlobChunkSecurityMetadataWitness,
    ordinal: BlobChunkOrdinal,
    byte_range: BlobChunkByteRange,
    rule_version: &'static str,
    counters: BlobChunkIntegrityCounterSnapshot,
}

impl BlobChunkIntegrityProof {
    pub(crate) fn admit(
        ordinal: BlobChunkOrdinal,
        window: BlobChunkByteWindow<'_>,
        checksum: PhysicalChunkChecksumWitness,
        rule: &BlobChunkingRuleAdmission,
        security_metadata: BlobChunkSecurityMetadataWitness,
        counters: BlobChunkIntegrityCounterSnapshot,
    ) -> Result<Self, BlobChunkIntegrityDenial> {
        window.validate_against_rule(rule)?;
        if checksum.bytes_checked() != window.range().len() {
            return Err(BlobChunkIntegrityDenial::WindowChecksumLengthMismatch { counters });
        }

        let stored_digest = StoredChunkDigest::from_declared_digest(stable_digest_for(
            "stored",
            rule.rule_version(),
            ordinal,
            window.range(),
            checksum.checksum().digest().as_str(),
        ));
        let content_digest = BlobChunkContentDigest::from_integrity_parts(stable_digest_for_bytes(
            "content",
            rule.rule_version(),
            ordinal,
            window.range(),
            window.bytes(),
        ));
        let identity_evidence = format!(
            "{}:{}:{}:{}:{}:{}",
            stored_digest.digest().as_str(),
            security_metadata.key_scope() as u8,
            security_metadata.key_version_posture() as u8,
            security_metadata.tenant_scope() as u8,
            security_metadata
                .authenticity_requirement()
                .class()
                .map_or(0, |class| class as u8),
            security_metadata.custody_posture() as u8
        );
        let identity = BlobChunkIdentity::from_integrity_parts(stable_digest_for(
            "chunk",
            rule.rule_version(),
            ordinal,
            window.range(),
            &identity_evidence,
        ));

        Ok(Self {
            identity,
            stored_digest,
            content_digest,
            checksum,
            security_metadata,
            ordinal,
            byte_range: window.range(),
            rule_version: rule.rule_version(),
            counters: counters.record_chunk_admitted(window.range().len()),
        })
    }

    pub const fn identity(&self) -> &BlobChunkIdentity {
        &self.identity
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn content_digest(&self) -> &BlobChunkContentDigest {
        &self.content_digest
    }

    pub const fn checksum(&self) -> &PhysicalChunkChecksumWitness {
        &self.checksum
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn ordinal(&self) -> BlobChunkOrdinal {
        self.ordinal
    }

    pub const fn byte_range(&self) -> BlobChunkByteRange {
        self.byte_range
    }

    pub const fn rule_version(&self) -> &'static str {
        self.rule_version
    }

    pub const fn counters(&self) -> BlobChunkIntegrityCounterSnapshot {
        self.counters
    }
}

pub(crate) fn stable_digest_for(
    lane: &str,
    rule_version: &str,
    ordinal: BlobChunkOrdinal,
    range: BlobChunkByteRange,
    evidence: &str,
) -> StableDigest {
    let mut hash = stable_hash_seed(lane);
    hash = stable_hash_bytes(hash, rule_version.as_bytes());
    hash = stable_hash_u64(hash, ordinal.get());
    hash = stable_hash_u64(hash, range.start());
    hash = stable_hash_u64(hash, range.len());
    hash = stable_hash_bytes(hash, evidence.as_bytes());
    StableDigest::new(format!("s7:{lane}:{hash:016x}")).expect("stable digest is nonempty")
}

pub(crate) fn stable_digest_for_bytes(
    lane: &str,
    rule_version: &str,
    ordinal: BlobChunkOrdinal,
    range: BlobChunkByteRange,
    bytes: &[u8],
) -> StableDigest {
    let mut hash = stable_hash_seed(lane);
    hash = stable_hash_bytes(hash, rule_version.as_bytes());
    hash = stable_hash_u64(hash, ordinal.get());
    hash = stable_hash_u64(hash, range.start());
    hash = stable_hash_u64(hash, range.len());
    hash = stable_hash_bytes(hash, bytes);
    StableDigest::new(format!("s7:{lane}:{hash:016x}")).expect("stable digest is nonempty")
}

fn stable_hash_seed(lane: &str) -> u64 {
    stable_hash_bytes(0xcbf2_9ce4_8422_2325, lane.as_bytes())
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

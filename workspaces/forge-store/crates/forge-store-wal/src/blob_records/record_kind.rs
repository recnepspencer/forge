use crate::{DurablePublicationDeclaration, DurablePublicationScope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobWalRecordKind {
    ChunkAppend,
    LsmValue,
    LsmTombstone,
    RootCandidate,
    GenerationPublication,
    SessionCheckpoint,
    SessionCloseout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobWalRecordIdentity {
    sequence: u64,
    kind: BlobWalRecordKind,
}

impl BlobWalRecordIdentity {
    pub const fn new(sequence: u64, kind: BlobWalRecordKind) -> Option<Self> {
        if sequence == 0 {
            return None;
        }
        Some(Self { sequence, kind })
    }

    pub const fn sequence(self) -> u64 {
        self.sequence
    }

    pub const fn kind(self) -> BlobWalRecordKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobWalRecordEnvelope {
    identity: BlobWalRecordIdentity,
    durable_publication: DurablePublicationDeclaration,
    payload_digest: String,
}

impl BlobWalRecordEnvelope {
    pub fn new(
        identity: BlobWalRecordIdentity,
        durable_publication: DurablePublicationDeclaration,
        payload_digest: impl Into<String>,
    ) -> Result<Self, BlobWalRecordScopeDenial> {
        let payload_digest = payload_digest.into();
        if payload_digest.is_empty() {
            return Err(BlobWalRecordScopeDenial::MissingPayloadDigest);
        }
        if !matches!(
            durable_publication.scope(),
            DurablePublicationScope::WalFrame(_)
        ) {
            return Err(BlobWalRecordScopeDenial::WalFrameScopeRequired);
        }
        Ok(Self {
            identity,
            durable_publication,
            payload_digest,
        })
    }

    pub const fn identity(&self) -> BlobWalRecordIdentity {
        self.identity
    }

    pub const fn durable_publication(&self) -> &DurablePublicationDeclaration {
        &self.durable_publication
    }

    pub fn payload_digest(&self) -> &str {
        &self.payload_digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobWalRecordScopeDenial {
    MissingPayloadDigest,
    WalFrameScopeRequired,
}

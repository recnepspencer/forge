use forge_store_blob_chunks::{BlobObjectId, ChunkTreeRoot};
use forge_store_physical_certification::S7MaterializedCloseoutEvidenceBundle;

use super::verifier::VerifiedS7CloseoutRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S7NativeBlobStoreCloseout {
    materialized_evidence: S7MaterializedCloseoutEvidenceBundle,
    binding_tag: String,
}

pub(crate) fn build_closeout_certificate(
    request: VerifiedS7CloseoutRequest,
) -> S7NativeBlobStoreCloseout {
    let materialized_evidence = request.input.into_materialized_evidence();
    let primary = materialized_evidence
        .executed_sources()
        .evidence_bundle()
        .primary();
    let lower = materialized_evidence.executed_sources().lifecycle_evidence();
    let binding_tag = format!(
        "s7-closeout:{}:{}:{:02x}{:02x}{:02x}{:02x}:{:02x}{:02x}{:02x}{:02x}",
        digest_prefix(lower.lifecycle_declaration().object_id().digest().as_str()),
        digest_prefix(lower.lifecycle_declaration().chunk_tree_root().digest().as_str()),
        primary.plan_digest()[0],
        primary.plan_digest()[1],
        primary.plan_digest()[2],
        primary.plan_digest()[3],
        primary.transcript_digest()[0],
        primary.transcript_digest()[1],
        primary.transcript_digest()[2],
        primary.transcript_digest()[3],
    );
    S7NativeBlobStoreCloseout {
        materialized_evidence,
        binding_tag,
    }
}

impl S7NativeBlobStoreCloseout {
    pub const fn materialized_evidence(&self) -> &S7MaterializedCloseoutEvidenceBundle {
        &self.materialized_evidence
    }

    pub fn binding_tag(&self) -> &str {
        &self.binding_tag
    }

    pub fn object_id(&self) -> &BlobObjectId {
        self.materialized_evidence
            .executed_sources()
            .lifecycle_evidence()
            .lifecycle_declaration()
            .object_id()
    }

    pub fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        self.materialized_evidence
            .executed_sources()
            .lifecycle_evidence()
            .lifecycle_declaration()
            .chunk_tree_root()
    }

    pub const fn declared_chunk_count(&self) -> u64 {
        self.materialized_evidence
            .executed_sources()
            .lifecycle_evidence()
            .executed_topology()
            .chunk_count()
    }

    pub const fn declared_bytes(&self) -> u64 {
        self.materialized_evidence
            .executed_sources()
            .lifecycle_evidence()
            .executed_topology()
            .logical_bytes()
    }
}

fn digest_prefix(value: &str) -> &str {
    value.get(..8).unwrap_or(value)
}

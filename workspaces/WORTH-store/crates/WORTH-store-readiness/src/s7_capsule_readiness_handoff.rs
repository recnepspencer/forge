use crate::S7CapsuleReadinessNonClaim;
use worth_store_blob_chunks::BlobCapsuleReadinessWitness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S7CapsuleReadinessHandoff {
    readiness_digest: String,
    declared_chunk_count: u64,
    declared_bytes: u64,
    planned_chunks: u64,
    materialized_chunks: u64,
    skipped_chunks: u64,
    denied_chunks: u64,
    readiness_publications: u64,
    non_claims: [S7CapsuleReadinessNonClaim; 4],
}

impl S7CapsuleReadinessHandoff {
    pub(crate) fn from_lower_capsule_readiness(
        readiness_digest: String,
        declared_chunk_count: u64,
        declared_bytes: u64,
        planned_chunks: u64,
        materialized_chunks: u64,
        skipped_chunks: u64,
        denied_chunks: u64,
        readiness_publications: u64,
    ) -> Self {
        Self {
            readiness_digest,
            declared_chunk_count,
            declared_bytes,
            planned_chunks,
            materialized_chunks,
            skipped_chunks,
            denied_chunks,
            readiness_publications,
            non_claims: S7CapsuleReadinessNonClaim::required(),
        }
    }

    pub fn readiness_digest(&self) -> &str {
        &self.readiness_digest
    }

    pub const fn declared_chunk_count(&self) -> u64 {
        self.declared_chunk_count
    }

    pub const fn declared_bytes(&self) -> u64 {
        self.declared_bytes
    }

    pub const fn planned_chunks(&self) -> u64 {
        self.planned_chunks
    }

    pub const fn materialized_chunks(&self) -> u64 {
        self.materialized_chunks
    }

    pub const fn skipped_chunks(&self) -> u64 {
        self.skipped_chunks
    }

    pub const fn denied_chunks(&self) -> u64 {
        self.denied_chunks
    }

    pub const fn readiness_publications(&self) -> u64 {
        self.readiness_publications
    }

    pub const fn non_claims(&self) -> &[S7CapsuleReadinessNonClaim; 4] {
        &self.non_claims
    }
}

pub fn admit_s7_capsule_readiness_handoff(
    readiness: &BlobCapsuleReadinessWitness,
) -> S7CapsuleReadinessHandoff {
    S7CapsuleReadinessHandoff::from_lower_capsule_readiness(
        readiness.readiness_digest().to_owned(),
        readiness.selected_chunks().len() as u64,
        readiness.declared_bytes(),
        readiness.counters().planned_chunks(),
        readiness.counters().materialized_chunks(),
        readiness.counters().skipped_chunks(),
        readiness.counters().denied_chunks(),
        readiness.counters().readiness_publications(),
    )
}

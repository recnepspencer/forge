use forge_store_readiness::S7CapsuleReadinessHandoff;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S7CapsuleReadinessCertificationReport {
    readiness_digest: String,
    declared_chunk_count: u64,
    declared_bytes: u64,
    planned_chunks: u64,
    materialized_chunks: u64,
    skipped_chunks: u64,
    denied_chunks: u64,
    readiness_publications: u64,
    non_claims: [forge_store_readiness::S7CapsuleReadinessNonClaim; 4],
}

pub fn certify_s7_capsule_readiness(
    handoff: &S7CapsuleReadinessHandoff,
) -> S7CapsuleReadinessCertificationReport {
    S7CapsuleReadinessCertificationReport {
        readiness_digest: handoff.readiness_digest().to_owned(),
        declared_chunk_count: handoff.declared_chunk_count(),
        declared_bytes: handoff.declared_bytes(),
        planned_chunks: handoff.planned_chunks(),
        materialized_chunks: handoff.materialized_chunks(),
        skipped_chunks: handoff.skipped_chunks(),
        denied_chunks: handoff.denied_chunks(),
        readiness_publications: handoff.readiness_publications(),
        non_claims: *handoff.non_claims(),
    }
}

impl S7CapsuleReadinessCertificationReport {
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

    pub const fn non_claims(&self) -> &[forge_store_readiness::S7CapsuleReadinessNonClaim; 4] {
        &self.non_claims
    }
}

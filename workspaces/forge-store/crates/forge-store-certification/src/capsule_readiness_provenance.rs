use forge_store_blob_chunks::BlobCapsuleReadinessWitness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCapsuleReadinessCertificationReport {
    readiness_digest: String,
    declared_chunk_count: u64,
    declared_bytes: u64,
    planned_chunks: u64,
    materialized_chunks: u64,
    skipped_chunks: u64,
    denied_chunks: u64,
    readiness_publications: u64,
}

pub fn certify_blob_capsule_readiness(
    witness: &BlobCapsuleReadinessWitness,
) -> BlobCapsuleReadinessCertificationReport {
    let counters = witness.counters();
    BlobCapsuleReadinessCertificationReport {
        readiness_digest: witness.readiness_digest().to_owned(),
        declared_chunk_count: witness.selected_chunks().len() as u64,
        declared_bytes: witness.declared_bytes(),
        planned_chunks: counters.planned_chunks(),
        materialized_chunks: counters.materialized_chunks(),
        skipped_chunks: counters.skipped_chunks(),
        denied_chunks: counters.denied_chunks(),
        readiness_publications: counters.readiness_publications(),
    }
}

impl BlobCapsuleReadinessCertificationReport {
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

}

mod canonical_basis;
mod counters;
mod denial;
mod integrity;
mod reference_accounting;
mod root_comparison;
mod root_counters;
mod root_denial;
mod root_publication;
mod rule;
mod sequence;

#[cfg(test)]
mod root_publication_tests;
#[cfg(test)]
mod tests;

pub use canonical_basis::BlobChunkRootCanonicalBasis;
pub use counters::BlobChunkIntegrityCounterSnapshot;
pub use denial::{
    reject_checksum_only_evidence_as_blob_chunk_integrity,
    reject_digest_only_evidence_as_blob_chunk_integrity, BlobChunkIntegrityDenial,
};
pub(crate) use integrity::{stable_digest_for, stable_digest_for_bytes};
pub use integrity::BlobChunkIntegrityProof;
pub use reference_accounting::{
    BlobChunkReferenceAccountingDenial, BlobChunkReferenceAccountingRegistry,
};
pub use root_comparison::BlobChunkRootCanonicalComparison;
pub use root_counters::BlobChunkRootCounterSnapshot;
pub use root_denial::{
    reject_checksum_only_evidence_as_chunk_root_publication,
    reject_digest_only_evidence_as_chunk_root_publication, BlobChunkRootPublicationDenial,
};
pub use root_publication::BlobChunkRootPublication;
pub use rule::{BlobChunkSize, BlobChunkingRuleAdmission};
pub use sequence::{
    AdmittedBlobChunkSequence, BlobChunkProofFrontier, BlobChunkProofLeaf,
    BlobChunkSequenceAdmission,
};

// Dedupe-owned witnesses re-exported for integrity-chain callers.
pub use crate::dedupe::evidence::BlobChunkCanonicalComparisonBasis;
pub use crate::dedupe::verification::BlobChunkCollisionVerificationReceipt;
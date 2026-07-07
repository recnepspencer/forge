//! Dedupe proof grammar: evidence → classification → verification → receipt → share claim.
mod admission;
pub(crate) mod classification;
mod collision_posture;
mod counters;
mod denial;
pub(crate) mod evidence;
mod index_posture;
mod policy;
mod receipt;
pub(crate) mod receipt_construction;
mod reference_edges;
pub(crate) mod transitions;
pub(crate) mod verification;

pub use admission::{BlobChunkDedupeAdmission, BlobChunkDedupeAdmissionOutcome};
pub use collision_posture::BlobChunkDedupeCollisionPosture;
pub use counters::BlobChunkDedupeCounterSnapshot;
pub use denial::BlobChunkDedupeAdmissionDenial;
pub use evidence::{
    BlobChunkCanonicalEquivalence, BlobChunkDedupeByteComparison, BlobChunkDedupeCandidate,
};
pub use index_posture::{BlobChunkDedupeDigestRewriteBasis, BlobChunkDedupeIndexPartition};
pub use policy::BlobChunkDedupePolicy;
pub use receipt::{BlobChunkDedupeReceipt, BlobChunkDedupeShareClaim};
pub use reference_edges::{
    BlobChunkDedupeReclaimDecision, BlobChunkDedupeReferenceRegistry,
    BlobChunkDedupeReferenceRelease, BlobChunkRegisteredDedupeReference,
};

mod aggregate;
mod aggregate_validation;
mod cache_identity;
mod compilation;
mod evidence_reader;
mod partition;
mod promotion;
mod shard;

pub(crate) use aggregate::repository_source_identity;
pub use aggregate::{CiCertificationAggregate, CiPartitionEvidence, MissingCiProofPartition};
pub use cache_identity::CiCacheIdentity;
pub use compilation::{
    CiCompilationAudit, CiCompilationDifference, CiCompilerArtifactObservation,
    CiExplainedCompilationDuplication,
};
pub use evidence_reader::read_partition_evidence;
pub use partition::{
    catalog, partition_products, required_lanes, CiProofPartition, CiProofPartitionKind,
    RequiredCiLane,
};
pub use shard::{CiShardAssignment, CiShardPlan};

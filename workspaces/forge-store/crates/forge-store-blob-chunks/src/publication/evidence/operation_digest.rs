use crate::{ChunkTreeRoot, LogicalContentDigest};

use super::identity::BlobPublicationRecoveryOperationDigest;
use super::super::{BlobReachabilityStaging, BlobRootCandidateForPublication};

pub(crate) fn chunk_write_recovery_operation_digest(
    digest: &LogicalContentDigest,
) -> BlobPublicationRecoveryOperationDigest {
    BlobPublicationRecoveryOperationDigest::from_stable_parts("chunk-write", digest.digest().as_str())
}

pub(crate) fn checksum_recovery_operation_digest(
    digest: &LogicalContentDigest,
) -> BlobPublicationRecoveryOperationDigest {
    BlobPublicationRecoveryOperationDigest::from_stable_parts(
        "checksum-admitted",
        digest.digest().as_str(),
    )
}

pub(crate) fn chunk_tree_recovery_operation_digest(
    root: &ChunkTreeRoot,
) -> BlobPublicationRecoveryOperationDigest {
    BlobPublicationRecoveryOperationDigest::from_stable_parts("chunk-tree-durable", root.digest().as_str())
}

pub(crate) fn root_candidate_recovery_operation_digest(
    candidate: &BlobRootCandidateForPublication,
) -> BlobPublicationRecoveryOperationDigest {
    BlobPublicationRecoveryOperationDigest::from_stable_parts(
        "root-candidate",
        candidate.intent().chunk_tree_root().digest().as_str(),
    )
}

pub(crate) fn reachability_recovery_operation_digest(
    staged: &BlobReachabilityStaging,
) -> BlobPublicationRecoveryOperationDigest {
    BlobPublicationRecoveryOperationDigest::from_stable_parts(
        "reachability-staged",
        staged
            .staging_identity()
            .publication_record_digest()
            .as_str(),
    )
}
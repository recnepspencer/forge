use super::*;

pub(super) fn edge_digest(
    kind: BlobReachabilityEdgeKind,
    object_id: &BlobObjectId,
    generation: BlobGeneration,
    root: &ChunkTreeRoot,
    logical: &LogicalContentDigest,
    leaf: &BlobChunkProofLeaf,
) -> StableDigest {
    edge_digest_from_parts(
        kind,
        object_id,
        generation,
        root,
        logical,
        leaf.identity().chunk_digest(),
        leaf.ordinal().get(),
    )
}

pub(super) fn dedupe_edge_digest(
    object_id: &BlobObjectId,
    generation: BlobGeneration,
    root: &ChunkTreeRoot,
    logical: &LogicalContentDigest,
    leaf: &BlobChunkProofLeaf,
    reference_identity: &StableDigest,
) -> StableDigest {
    StableDigest::new(format!(
        "s7.reach.edge:dedupe:{}:{}:{}:{}:{}:{}:{}",
        object_id.digest().as_str(),
        generation.sequence(),
        root.digest().as_str(),
        logical.digest().as_str(),
        leaf.identity().chunk_digest().as_str(),
        leaf.ordinal().get(),
        reference_identity.as_str()
    ))
    .expect("dedupe reachability edge digest is nonempty")
}

pub(super) fn edge_digest_from_parts(
    kind: BlobReachabilityEdgeKind,
    object_id: &BlobObjectId,
    generation: BlobGeneration,
    root: &ChunkTreeRoot,
    logical: &LogicalContentDigest,
    chunk_digest: &StableDigest,
    ordinal: u64,
) -> StableDigest {
    StableDigest::new(format!(
        "s7.reach.edge:{}:{}:{}:{}:{}:{}:{}",
        edge_kind_name(kind),
        object_id.digest().as_str(),
        generation.sequence(),
        root.digest().as_str(),
        logical.digest().as_str(),
        chunk_digest.as_str(),
        ordinal
    ))
    .expect("reachability edge digest is nonempty")
}

fn edge_kind_name(kind: BlobReachabilityEdgeKind) -> &'static str {
    match kind {
        BlobReachabilityEdgeKind::PrimaryBlobReference => "primary",
        BlobReachabilityEdgeKind::DerivedBlobReference => "derived",
        BlobReachabilityEdgeKind::GenerationHoldReference => "generation-hold",
        BlobReachabilityEdgeKind::TimeWindowHoldReference => "time-window-hold",
        BlobReachabilityEdgeKind::ResumeSessionReference => "resume",
        BlobReachabilityEdgeKind::CheckpointHoldReference => "checkpoint",
        BlobReachabilityEdgeKind::BackupHoldReference => "backup",
        BlobReachabilityEdgeKind::ExportHoldReference => "export",
        BlobReachabilityEdgeKind::TenantCustodyHoldReference => "tenant-custody",
        BlobReachabilityEdgeKind::ExternalConsumerHoldReference => "external",
        BlobReachabilityEdgeKind::ReplicationCapsuleReference => "capsule",
        BlobReachabilityEdgeKind::ReadPlanHoldReference => "read-plan",
        BlobReachabilityEdgeKind::QuarantineHoldReference => "quarantine",
        BlobReachabilityEdgeKind::PlacementMoveReference => "placement-move",
        BlobReachabilityEdgeKind::DedupeSharedReference => "dedupe",
    }
}

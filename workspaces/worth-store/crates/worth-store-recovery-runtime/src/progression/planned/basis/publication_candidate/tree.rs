use worth_store_physical_format::{
    durable_artifact_checksum, CurrentPhysicalRecordPlacement, FreeSpaceBlockReference,
    ManifestBlockReference, PhysicalFreeSpaceMembershipBlock, PhysicalRootRoutingBlock,
    PhysicalSegmentMembershipBlock, RecordArtifactFile, RecordFreeSpaceManifestEntry,
    RecordSegmentPageManifestEntry, SegmentManifestBlockReference,
};

use super::{CandidateBuild, CandidateBuildDenial};

pub(super) fn root_routing(
    build: &mut CandidateBuild,
    entries: &[CurrentPhysicalRecordPlacement],
    tree: u64,
    generation: u64,
    capacity: u16,
    mut next_block: u64,
) -> Result<(Option<ManifestBlockReference>, u64), CandidateBuildDenial> {
    let mut roots = Vec::new();
    for chunk in entries.chunks(usize::from(capacity)) {
        let block_id = allocate(&mut next_block)?;
        let block =
            PhysicalRootRoutingBlock::leaf(tree, generation, block_id, chunk.to_vec(), capacity)
                .ok_or(CandidateBuildDenial::Invalid)?;
        let bytes = block.encode(build.format);
        roots.push(block.reference(durable_artifact_checksum(&bytes)));
        build.push(
            RecordArtifactFile::RootRoutingBlock {
                generation,
                block: block_id,
            },
            bytes,
        )?;
    }
    while roots.len() > 1 {
        let mut parents = Vec::new();
        for chunk in roots.chunks(usize::from(capacity)) {
            let block_id = allocate(&mut next_block)?;
            let level = chunk[0]
                .level()
                .checked_add(1)
                .ok_or(CandidateBuildDenial::Invalid)?;
            let block = PhysicalRootRoutingBlock::branch(
                tree,
                generation,
                block_id,
                level,
                chunk.to_vec(),
                capacity,
            )
            .ok_or(CandidateBuildDenial::Invalid)?;
            let bytes = block.encode(build.format);
            parents.push(block.reference(durable_artifact_checksum(&bytes)));
            build.push(
                RecordArtifactFile::RootRoutingBlock {
                    generation,
                    block: block_id,
                },
                bytes,
            )?;
        }
        roots = parents;
    }
    Ok((roots.pop(), next_block))
}

pub(super) fn segment_routing(
    build: &mut CandidateBuild,
    entries: &[RecordSegmentPageManifestEntry],
    tree: u64,
    generation: u64,
    capacity: u16,
    mut next_block: u64,
) -> Result<(Option<SegmentManifestBlockReference>, u64), CandidateBuildDenial> {
    let mut roots = Vec::new();
    for chunk in entries.chunks(usize::from(capacity)) {
        let block_id = allocate(&mut next_block)?;
        let block = PhysicalSegmentMembershipBlock::leaf(
            tree,
            generation,
            block_id,
            chunk.to_vec(),
            capacity,
        )
        .ok_or(CandidateBuildDenial::Invalid)?;
        let bytes = block.encode(build.format);
        roots.push(block.reference(durable_artifact_checksum(&bytes)));
        build.push(
            RecordArtifactFile::SegmentMembershipBlock {
                generation,
                block: block_id,
            },
            bytes,
        )?;
    }
    while roots.len() > 1 {
        let mut parents = Vec::new();
        for chunk in roots.chunks(usize::from(capacity)) {
            let block_id = allocate(&mut next_block)?;
            let level = chunk[0]
                .level()
                .checked_add(1)
                .ok_or(CandidateBuildDenial::Invalid)?;
            let block = PhysicalSegmentMembershipBlock::branch(
                tree,
                generation,
                block_id,
                level,
                chunk.to_vec(),
                capacity,
            )
            .ok_or(CandidateBuildDenial::Invalid)?;
            let bytes = block.encode(build.format);
            parents.push(block.reference(durable_artifact_checksum(&bytes)));
            build.push(
                RecordArtifactFile::SegmentMembershipBlock {
                    generation,
                    block: block_id,
                },
                bytes,
            )?;
        }
        roots = parents;
    }
    Ok((roots.pop(), next_block))
}

pub(super) fn free_space_routing(
    build: &mut CandidateBuild,
    entries: &[RecordFreeSpaceManifestEntry],
    tree: u64,
    generation: u64,
    capacity: u16,
    mut next_block: u64,
) -> Result<(Option<FreeSpaceBlockReference>, u64), CandidateBuildDenial> {
    let mut roots = Vec::new();
    for chunk in entries.chunks(usize::from(capacity)) {
        let block_id = allocate(&mut next_block)?;
        let block = PhysicalFreeSpaceMembershipBlock::leaf(
            tree,
            generation,
            block_id,
            chunk.to_vec(),
            capacity,
        )
        .ok_or(CandidateBuildDenial::Invalid)?;
        let bytes = block.encode(build.format);
        roots.push(block.reference(durable_artifact_checksum(&bytes)));
        build.push(
            RecordArtifactFile::FreeSpaceMembershipBlock {
                generation,
                block: block_id,
            },
            bytes,
        )?;
    }
    while roots.len() > 1 {
        let mut parents = Vec::new();
        for chunk in roots.chunks(usize::from(capacity)) {
            let block_id = allocate(&mut next_block)?;
            let level = chunk[0]
                .level()
                .checked_add(1)
                .ok_or(CandidateBuildDenial::Invalid)?;
            let block = PhysicalFreeSpaceMembershipBlock::branch(
                tree,
                generation,
                block_id,
                level,
                chunk.to_vec(),
                capacity,
            )
            .ok_or(CandidateBuildDenial::Invalid)?;
            let bytes = block.encode(build.format);
            parents.push(block.reference(durable_artifact_checksum(&bytes)));
            build.push(
                RecordArtifactFile::FreeSpaceMembershipBlock {
                    generation,
                    block: block_id,
                },
                bytes,
            )?;
        }
        roots = parents;
    }
    Ok((roots.pop(), next_block))
}

fn allocate(next: &mut u64) -> Result<u64, CandidateBuildDenial> {
    let block = *next;
    *next = next.checked_add(1).ok_or(CandidateBuildDenial::Invalid)?;
    if block == 0 {
        return Err(CandidateBuildDenial::Invalid);
    }
    Ok(block)
}

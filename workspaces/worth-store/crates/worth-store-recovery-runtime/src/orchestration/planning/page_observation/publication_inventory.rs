use std::collections::{BTreeSet, VecDeque};

use worth_store::physical_runtime::BoundedRecoveryFilesystemDiscovery;
use worth_store_physical_format::{
    durable_artifact_checksum, BoundedFreeSpaceMembershipBlockDecodeDenial,
    BoundedSegmentMembershipBlockDecodeDenial, DurableFreeSpaceManifestHeader,
    DurablePhysicalRootManifest, FreeSpaceMembershipBlockDecodeLimits,
    PhysicalFreeSpaceMembershipBlock, PhysicalRecordFormatDeclaration,
    PhysicalSegmentMembershipBlock, RecordArtifactFile, RecordFreeSpaceManifestEntry,
    RecordSegmentPageManifestEntry, SegmentMembershipBlockDecodeLimits,
};

use super::{required, PageObservationFailure};
use crate::orchestration::planning::segment_observation::ManifestEntryBudget;
use crate::progression::RecoveryPublicationSourceInventory;

pub(super) fn observe(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    root: &DurablePhysicalRootManifest,
    format: PhysicalRecordFormatDeclaration,
    budget: &mut ManifestEntryBudget,
    byte_limit: u64,
) -> Result<RecoveryPublicationSourceInventory, PageObservationFailure> {
    budget.admit_pending_block_read()?;
    let free_space = read_free_space_header(discovery, root, format, byte_limit)?;
    let (segment_entries, segment_artifacts) =
        read_segment_entries(discovery, root, format, budget, byte_limit)?;
    let (free_entries, free_artifacts) =
        read_free_entries(discovery, &free_space, format, budget, byte_limit)?;
    let mut source_artifacts = BTreeSet::from([RecordArtifactFile::FreeSpaceManifest {
        generation: root.generation(),
    }]);
    source_artifacts.extend(segment_artifacts);
    source_artifacts.extend(free_artifacts);
    Ok(RecoveryPublicationSourceInventory {
        free_space,
        segment_entries: segment_entries.into_boxed_slice(),
        free_entries: free_entries.into_boxed_slice(),
        source_artifacts: source_artifacts
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    })
}

fn read_free_space_header(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    root: &DurablePhysicalRootManifest,
    format: PhysicalRecordFormatDeclaration,
    byte_limit: u64,
) -> Result<DurableFreeSpaceManifestHeader, PageObservationFailure> {
    let artifact = RecordArtifactFile::FreeSpaceManifest {
        generation: root.generation(),
    };
    let bytes = required(
        discovery.read_free_space_manifest(root.generation(), byte_limit),
        None,
        artifact,
    )?;
    let (header, found_format) =
        DurableFreeSpaceManifestHeader::decode(&bytes, u16::MAX).map_err(|_| invalid(artifact))?;
    if found_format != format
        || header.generation() != root.generation()
        || header.root() != root.free_space_root()
        || durable_artifact_checksum(&bytes) != root.free_space_checksum()
    {
        return Err(invalid(artifact));
    }
    Ok(header)
}

fn read_segment_entries(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    root: &DurablePhysicalRootManifest,
    format: PhysicalRecordFormatDeclaration,
    budget: &mut ManifestEntryBudget,
    byte_limit: u64,
) -> Result<
    (
        Vec<RecordSegmentPageManifestEntry>,
        BTreeSet<RecordArtifactFile>,
    ),
    PageObservationFailure,
> {
    let mut pending = root.segment_root().into_iter().collect::<VecDeque<_>>();
    let mut visited = BTreeSet::new();
    let mut artifacts = BTreeSet::new();
    let mut entries = Vec::new();
    while let Some(reference) = pending.pop_front() {
        budget.admit_pending_block_read()?;
        let artifact = RecordArtifactFile::SegmentMembershipBlock {
            generation: reference.generation(),
            block: reference.block(),
        };
        artifacts.insert(artifact);
        if !visited.insert((reference.generation(), reference.block())) {
            return Err(invalid(artifact));
        }
        let bytes = required(
            discovery.read_segment_membership_block(
                reference.generation(),
                reference.block(),
                byte_limit,
            ),
            None,
            artifact,
        )?;
        let (block, found_format) = PhysicalSegmentMembershipBlock::decode_bounded(
            &bytes,
            root.node_capacity(),
            SegmentMembershipBlockDecodeLimits {
                leaf_entries: budget.remaining(),
                branch_children: budget.remaining(),
            },
        )
        .map_err(|denial| segment_denial(denial, artifact))?;
        if found_format != format
            || block.tree_identity() != root.tree_identity()
            || block.reference(durable_artifact_checksum(&bytes)) != reference
        {
            return Err(invalid(artifact));
        }
        if let Some(found) = block.entries() {
            budget.consume(found.len())?;
            entries.extend_from_slice(found);
        } else if let Some(children) = block.children() {
            budget.consume(children.len())?;
            pending.extend(children.iter().copied());
        }
    }
    entries
        .sort_unstable_by_key(|entry| (entry.page_cell().segment_id().get(), entry.page().get()));
    if entries.windows(2).any(|pair| {
        pair[0].page_cell().segment_id() == pair[1].page_cell().segment_id()
            && pair[0].page() == pair[1].page()
    }) {
        return Err(invalid(RecordArtifactFile::RootManifest {
            generation: root.generation(),
        }));
    }
    Ok((entries, artifacts))
}

fn read_free_entries(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    header: &DurableFreeSpaceManifestHeader,
    format: PhysicalRecordFormatDeclaration,
    budget: &mut ManifestEntryBudget,
    byte_limit: u64,
) -> Result<
    (
        Vec<RecordFreeSpaceManifestEntry>,
        BTreeSet<RecordArtifactFile>,
    ),
    PageObservationFailure,
> {
    let mut pending = header.root().into_iter().collect::<VecDeque<_>>();
    let mut visited = BTreeSet::new();
    let mut artifacts = BTreeSet::new();
    let mut entries = Vec::new();
    while let Some(reference) = pending.pop_front() {
        budget.admit_pending_block_read()?;
        let artifact = RecordArtifactFile::FreeSpaceMembershipBlock {
            generation: reference.generation(),
            block: reference.block(),
        };
        artifacts.insert(artifact);
        if !visited.insert((reference.generation(), reference.block())) {
            return Err(invalid(artifact));
        }
        let bytes = required(
            discovery.read_free_space_membership_block(
                reference.generation(),
                reference.block(),
                byte_limit,
            ),
            None,
            artifact,
        )?;
        let (block, found_format) = PhysicalFreeSpaceMembershipBlock::decode_bounded(
            &bytes,
            header.node_capacity(),
            FreeSpaceMembershipBlockDecodeLimits {
                leaf_entries: budget.remaining(),
                branch_children: budget.remaining(),
            },
        )
        .map_err(|denial| free_denial(denial, artifact))?;
        if found_format != format
            || block.tree_identity() != header.tree_identity()
            || block.reference(durable_artifact_checksum(&bytes)) != reference
        {
            return Err(invalid(artifact));
        }
        if let Some(found) = block.entries() {
            budget.consume(found.len())?;
            entries.extend_from_slice(found);
        } else if let Some(children) = block.children() {
            budget.consume(children.len())?;
            pending.extend(children.iter().copied());
        }
    }
    entries.sort_unstable_by_key(|entry| (entry.class() as u8, entry.owner()));
    if entries
        .windows(2)
        .any(|pair| pair[0].class() == pair[1].class() && pair[0].owner() == pair[1].owner())
    {
        return Err(invalid(RecordArtifactFile::FreeSpaceManifest {
            generation: header.generation(),
        }));
    }
    Ok((entries, artifacts))
}

fn segment_denial(
    denial: BoundedSegmentMembershipBlockDecodeDenial,
    artifact: RecordArtifactFile,
) -> PageObservationFailure {
    match denial {
        BoundedSegmentMembershipBlockDecodeDenial::LeafEntries { .. }
        | BoundedSegmentMembershipBlockDecodeDenial::BranchChildren { .. } => {
            PageObservationFailure::ManifestEntryLimit
        }
        BoundedSegmentMembershipBlockDecodeDenial::Format(_) => invalid(artifact),
    }
}

fn free_denial(
    denial: BoundedFreeSpaceMembershipBlockDecodeDenial,
    artifact: RecordArtifactFile,
) -> PageObservationFailure {
    match denial {
        BoundedFreeSpaceMembershipBlockDecodeDenial::LeafEntries { .. }
        | BoundedFreeSpaceMembershipBlockDecodeDenial::BranchChildren { .. } => {
            PageObservationFailure::ManifestEntryLimit
        }
        BoundedFreeSpaceMembershipBlockDecodeDenial::Format(_) => invalid(artifact),
    }
}

const fn invalid(artifact: RecordArtifactFile) -> PageObservationFailure {
    PageObservationFailure::InvalidManifest {
        target: None,
        artifact,
    }
}

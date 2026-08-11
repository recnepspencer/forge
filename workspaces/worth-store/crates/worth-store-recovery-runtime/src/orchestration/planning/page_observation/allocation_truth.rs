use std::collections::{BTreeMap, BTreeSet};

use worth_store::physical_runtime::BoundedRecoveryFilesystemDiscovery;
use worth_store_physical_format::{
    durable_artifact_checksum, BoundedFreeSpaceMembershipBlockDecodeDenial,
    CurrentPhysicalRecordPlacement, DurableFreeSpaceManifestHeader, DurablePhysicalRootManifest,
    FreeSpaceBlockReference, FreeSpaceKey, FreeSpaceMembershipBlockDecodeLimits,
    PhysicalFreeSpaceMembershipBlock, PhysicalRecordFormatDeclaration, RecordAllocationClass,
    RecordArtifactFile, RecordFreeSpaceManifestEntry,
};
use worth_store_recovery_physics::{
    PhysicalRedoTarget, PhysicalRedoTargetIdentity, RecoveryPageObservation,
};

use super::{required, PageObservationFailure};
use crate::orchestration::planning::segment_observation::ManifestEntryBudget;

mod inline_allocation;
#[cfg(test)]
mod tests;

use inline_allocation::admit_inline_allocations;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AdmittedAbsentTargets {
    pub(super) observations: Vec<RecoveryPageObservation>,
    pub(super) inline_truth: Option<InlineAllocationTruth>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::orchestration::planning) struct InlineAllocationTruth {
    pub(in crate::orchestration::planning) next_segment: u64,
    pub(in crate::orchestration::planning) page_capacity: u32,
}

pub(super) fn admit_absent_targets(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    root: &DurablePhysicalRootManifest,
    placements: &[CurrentPhysicalRecordPlacement],
    targets: Vec<&PhysicalRedoTarget>,
    format: PhysicalRecordFormatDeclaration,
    manifest_entries: &mut ManifestEntryBudget,
    byte_limit: u64,
    absence_identity: [u8; 32],
) -> Result<AdmittedAbsentTargets, PageObservationFailure> {
    if targets.is_empty() {
        return Ok(AdmittedAbsentTargets {
            observations: Vec::new(),
            inline_truth: None,
        });
    }
    manifest_entries.admit_pending_block_read()?;
    let header = read_header(discovery, root, format, byte_limit, targets[0].identity())?;
    admit_inline_allocations(
        discovery,
        root,
        placements,
        &targets,
        &header,
        format,
        manifest_entries,
        byte_limit,
    )?;
    admit_extent_allocations(&targets, &header)?;
    Ok(AdmittedAbsentTargets {
        observations: targets
            .into_iter()
            .map(|target| RecoveryPageObservation::absent(target, absence_identity))
            .collect(),
        inline_truth: Some(InlineAllocationTruth {
            next_segment: header.next_segment(),
            page_capacity: header.segment_page_capacity(),
        }),
    })
}

fn read_header(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    root: &DurablePhysicalRootManifest,
    format: PhysicalRecordFormatDeclaration,
    byte_limit: u64,
    target: PhysicalRedoTargetIdentity,
) -> Result<DurableFreeSpaceManifestHeader, PageObservationFailure> {
    let artifact = RecordArtifactFile::FreeSpaceManifest {
        generation: root.generation(),
    };
    let bytes = required(
        discovery.read_free_space_manifest(root.generation(), byte_limit),
        Some(target),
        artifact,
    )?;
    let (header, found_format) =
        DurableFreeSpaceManifestHeader::decode(&bytes, u16::MAX).map_err(|_| {
            PageObservationFailure::InvalidManifest {
                target: Some(target),
                artifact,
            }
        })?;
    if found_format != format
        || header.generation() != root.generation()
        || header.root() != root.free_space_root()
        || durable_artifact_checksum(&bytes) != root.free_space_checksum()
    {
        return Err(PageObservationFailure::InvalidManifest {
            target: Some(target),
            artifact,
        });
    }
    Ok(header)
}

fn admit_extent_allocations(
    targets: &[&PhysicalRedoTarget],
    header: &DurableFreeSpaceManifestHeader,
) -> Result<(), PageObservationFailure> {
    let mut extents = BTreeMap::new();
    for target in targets {
        let PhysicalRedoTargetIdentity::ExtentChunk {
            extent, generation, ..
        } = target.identity()
        else {
            continue;
        };
        if generation != 1 {
            return Err(PageObservationFailure::InvalidTarget(target.identity()));
        }
        extents.entry(extent).or_insert(*target);
    }
    if !sequence_starts_at(extents.keys().copied(), header.next_extent()) {
        return Err(PageObservationFailure::InvalidTarget(
            extents
                .into_values()
                .next()
                .expect("nonempty failed sequence")
                .identity(),
        ));
    }
    Ok(())
}

fn sequence_starts_at(values: impl IntoIterator<Item = u64>, first: u64) -> bool {
    values
        .into_iter()
        .enumerate()
        .all(|(ordinal, value)| first.checked_add(ordinal as u64) == Some(value))
}

fn reusable_capacity(
    entry: RecordFreeSpaceManifestEntry,
    selected_generation: u64,
    selected_pages: u64,
    capacity: u32,
) -> Option<u64> {
    (entry.generation() == selected_generation
        && entry.first_unallocated() == selected_pages.saturating_add(1)
        && entry
            .first_unallocated()
            .checked_add(entry.unallocated_count())
            == Some(u64::from(capacity) + 1))
    .then_some(entry.unallocated_count())
}

#[allow(clippy::too_many_arguments)]
fn locate_free_entry(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    header: &DurableFreeSpaceManifestHeader,
    key: FreeSpaceKey,
    format: PhysicalRecordFormatDeclaration,
    entry_budget: &mut ManifestEntryBudget,
    byte_limit: u64,
    target: PhysicalRedoTargetIdentity,
) -> Result<Option<RecordFreeSpaceManifestEntry>, PageObservationFailure> {
    let Some(mut reference) = header.root().filter(|reference| reference.contains(key)) else {
        return Ok(None);
    };
    loop {
        entry_budget.admit_pending_block_read()?;
        let artifact = free_block_artifact(reference);
        let bytes = required(
            discovery.read_free_space_membership_block(
                reference.generation(),
                reference.block(),
                byte_limit,
            ),
            Some(target),
            artifact,
        )?;
        let (block, found_format) = PhysicalFreeSpaceMembershipBlock::decode_bounded(
            &bytes,
            header.node_capacity(),
            FreeSpaceMembershipBlockDecodeLimits {
                leaf_entries: entry_budget.remaining(),
                branch_children: entry_budget.remaining(),
            },
        )
        .map_err(|denial| match denial {
            BoundedFreeSpaceMembershipBlockDecodeDenial::LeafEntries { .. }
            | BoundedFreeSpaceMembershipBlockDecodeDenial::BranchChildren { .. } => {
                PageObservationFailure::ManifestEntryLimit
            }
            BoundedFreeSpaceMembershipBlockDecodeDenial::Format(_) => {
                PageObservationFailure::InvalidManifest {
                    target: Some(target),
                    artifact,
                }
            }
        })?;
        if found_format != format
            || block.tree_identity() != header.tree_identity()
            || block.reference(durable_artifact_checksum(&bytes)) != reference
        {
            return Err(PageObservationFailure::InvalidManifest {
                target: Some(target),
                artifact,
            });
        }
        match block {
            PhysicalFreeSpaceMembershipBlock::Leaf { entries, .. } => {
                entry_budget.consume(entries.len())?;
                return Ok(entries
                    .into_iter()
                    .find(|entry| FreeSpaceKey::from(*entry) == key));
            }
            PhysicalFreeSpaceMembershipBlock::Branch { children, .. } => {
                entry_budget.consume(children.len())?;
                reference = children
                    .into_iter()
                    .find(|child| child.contains(key))
                    .ok_or(PageObservationFailure::InvalidManifest {
                        target: Some(target),
                        artifact,
                    })?;
            }
        }
    }
}

const fn free_block_artifact(reference: FreeSpaceBlockReference) -> RecordArtifactFile {
    RecordArtifactFile::FreeSpaceMembershipBlock {
        generation: reference.generation(),
        block: reference.block(),
    }
}

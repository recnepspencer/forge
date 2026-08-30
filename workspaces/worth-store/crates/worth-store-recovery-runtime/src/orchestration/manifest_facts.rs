use std::collections::{BTreeSet, VecDeque};

use worth_store::physical_runtime::BoundedRecoveryFilesystemDiscovery;
use worth_store_physical_format::{
    durable_artifact_checksum, BoundedRootRoutingBlockDecodeDenial, ManifestBlockReference,
    PhysicalRootRoutingBlock, RootRoutingBlockDecodeLimits,
};
use worth_store_recovery_physics::{
    PhysicalManifestBlockCandidate, PhysicalRootSlotObservation, PhysicalRootSourceCandidate,
};

use crate::entry::{
    PhysicalManifestObservationDenial, PhysicalRecoveryBlockKind as PhysicalRecoveryBlock,
    PhysicalRecoveryLimitDimension,
};
use crate::orchestration::discovery::DiscoveryFailure;

pub(crate) enum ManifestFactsDiscovery {
    Unavailable,
    Rejected(PhysicalManifestObservationDenial),
    Observed {
        blocks: Vec<PhysicalManifestBlockCandidate>,
    },
}

pub(super) struct ManifestObservationBudget<'a> {
    pub remaining_bytes: &'a mut u64,
    pub admitted_bytes: u64,
    pub remaining_entries: &'a mut u64,
    pub admitted_entries: u64,
    pub remaining_blocks: &'a mut u64,
    pub admitted_blocks: u64,
}

pub(super) fn observe_manifest_facts(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    root: &PhysicalRootSlotObservation,
    mut budget: ManifestObservationBudget<'_>,
) -> Result<ManifestFactsDiscovery, DiscoveryFailure> {
    let PhysicalRootSlotObservation::Candidate(root) = root else {
        return Ok(ManifestFactsDiscovery::Unavailable);
    };
    let mut pending = root
        .manifest()
        .routing_root()
        .into_iter()
        .collect::<VecDeque<_>>();
    let mut visited = BTreeSet::new();
    let mut candidates = Vec::new();
    while let Some(reference) = pending.pop_front() {
        if !visited.insert((reference.generation(), reference.block())) {
            return Ok(ManifestFactsDiscovery::Rejected(
                PhysicalManifestObservationDenial::DuplicateReference { reference },
            ));
        }
        let queued_blocks = pending.len() as u64;
        let observed =
            observe_manifest_block(discovery, root, reference, queued_blocks, &mut budget)?;
        let (block, bytes) = match observed {
            Ok(observed) => observed,
            Err(denial) => return Ok(ManifestFactsDiscovery::Rejected(denial)),
        };
        if let PhysicalRootRoutingBlock::Branch { children, .. } = &block {
            pending.extend(children.iter().copied());
        }
        candidates.push(PhysicalManifestBlockCandidate::new(reference, bytes));
    }
    Ok(ManifestFactsDiscovery::Observed { blocks: candidates })
}

fn observe_manifest_block(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    root: &PhysicalRootSourceCandidate,
    reference: ManifestBlockReference,
    queued_blocks: u64,
    budget: &mut ManifestObservationBudget<'_>,
) -> Result<
    Result<(PhysicalRootRoutingBlock, Vec<u8>), PhysicalManifestObservationDenial>,
    DiscoveryFailure,
> {
    consume_block_budget(budget)?;
    let block_byte_limit =
        u64::from(root.selector().format().page_size().bytes()).min(*budget.remaining_bytes);
    let artifact = discovery
        .read_root_routing_block(reference.generation(), reference.block(), block_byte_limit)
        .map_err(|failure| {
            super::discovery::map_cumulative_discovery_failure(
                failure,
                PhysicalRecoveryLimitDimension::ManifestEntries,
                PhysicalRecoveryLimitDimension::ManifestBytes,
                budget.admitted_bytes,
                *budget.remaining_bytes,
            )
        })?;
    let Some(bytes) = artifact.into_bytes() else {
        return Ok(Err(PhysicalManifestObservationDenial::MissingArtifact {
            reference,
        }));
    };
    *budget.remaining_bytes = budget
        .remaining_bytes
        .checked_sub(bytes.len() as u64)
        .ok_or_else(|| DiscoveryFailure::from(PhysicalRecoveryBlock::DiscoveryLimit))?;
    let observed = match decode_and_validate_manifest_block(
        root,
        reference,
        &bytes,
        RootRoutingBlockDecodeLimits {
            leaf_entries: *budget.remaining_entries,
            branch_children: budget.remaining_blocks.saturating_sub(queued_blocks),
        },
    ) {
        Ok(observed) => observed,
        Err(ManifestBlockObservationFailure::LeafEntryLimit { observed }) => {
            let consumed = budget
                .admitted_entries
                .saturating_sub(*budget.remaining_entries);
            return Err(super::discovery::discovery_limit(
                PhysicalRecoveryLimitDimension::ManifestEntries,
                consumed.saturating_add(observed),
                budget.admitted_entries,
            ));
        }
        Err(ManifestBlockObservationFailure::BranchChildLimit { observed }) => {
            let consumed = budget
                .admitted_blocks
                .saturating_sub(*budget.remaining_blocks);
            return Err(super::discovery::discovery_limit(
                PhysicalRecoveryLimitDimension::ManifestEntries,
                consumed
                    .saturating_add(queued_blocks)
                    .saturating_add(observed),
                budget.admitted_blocks,
            ));
        }
        Err(ManifestBlockObservationFailure::Format(denial)) => return Ok(Err(denial)),
    };
    consume_entry_budget(&observed, budget)?;
    Ok(Ok((observed, bytes)))
}

fn decode_and_validate_manifest_block(
    root: &PhysicalRootSourceCandidate,
    reference: ManifestBlockReference,
    bytes: &[u8],
    limits: RootRoutingBlockDecodeLimits,
) -> Result<PhysicalRootRoutingBlock, ManifestBlockObservationFailure> {
    let (block, observed_format) =
        PhysicalRootRoutingBlock::decode_bounded(bytes, root.manifest().node_capacity(), limits)
            .map_err(|denial| match denial {
                BoundedRootRoutingBlockDecodeDenial::Format(denial) => {
                    ManifestBlockObservationFailure::Format(
                        PhysicalManifestObservationDenial::Decode { reference, denial },
                    )
                }
                BoundedRootRoutingBlockDecodeDenial::LeafEntries { observed, .. } => {
                    ManifestBlockObservationFailure::LeafEntryLimit { observed }
                }
                BoundedRootRoutingBlockDecodeDenial::BranchChildren { observed, .. } => {
                    ManifestBlockObservationFailure::BranchChildLimit { observed }
                }
            })?;
    let expected_format = root.selector().format();
    if observed_format != expected_format {
        return Err(ManifestBlockObservationFailure::Format(
            PhysicalManifestObservationDenial::FormatIdentity {
                reference,
                expected: expected_format,
                observed: observed_format,
            },
        ));
    }
    let expected_tree = root.manifest().tree_identity();
    if block.tree_identity() != expected_tree {
        return Err(ManifestBlockObservationFailure::Format(
            PhysicalManifestObservationDenial::TreeIdentity {
                reference,
                expected: expected_tree,
                observed: block.tree_identity(),
            },
        ));
    }
    let observed_reference = block.reference(durable_artifact_checksum(bytes));
    if observed_reference != reference {
        return Err(ManifestBlockObservationFailure::Format(
            PhysicalManifestObservationDenial::ReferenceIntegrity {
                expected: reference,
                observed: observed_reference,
            },
        ));
    }
    Ok(block)
}

enum ManifestBlockObservationFailure {
    Format(PhysicalManifestObservationDenial),
    LeafEntryLimit { observed: u64 },
    BranchChildLimit { observed: u64 },
}

fn consume_block_budget(
    budget: &mut ManifestObservationBudget<'_>,
) -> Result<(), DiscoveryFailure> {
    if *budget.remaining_blocks == 0 {
        return Err(super::discovery::discovery_limit(
            PhysicalRecoveryLimitDimension::ManifestEntries,
            budget.admitted_blocks.saturating_add(1),
            budget.admitted_blocks,
        ));
    }
    *budget.remaining_blocks -= 1;
    Ok(())
}

fn consume_entry_budget(
    block: &PhysicalRootRoutingBlock,
    budget: &mut ManifestObservationBudget<'_>,
) -> Result<(), DiscoveryFailure> {
    let entry_count = match block {
        PhysicalRootRoutingBlock::Leaf { entries, .. } => entries.len() as u64,
        PhysicalRootRoutingBlock::Branch { .. } => 0,
    };
    if entry_count > *budget.remaining_entries {
        return Err(super::discovery::discovery_limit(
            PhysicalRecoveryLimitDimension::ManifestEntries,
            budget
                .admitted_entries
                .saturating_sub(*budget.remaining_entries)
                .saturating_add(entry_count),
            budget.admitted_entries,
        ));
    }
    *budget.remaining_entries -= entry_count;
    Ok(())
}

impl ManifestFactsDiscovery {
    pub(crate) fn block_count(&self) -> u64 {
        match self {
            Self::Observed { blocks, .. } => blocks.len() as u64,
            Self::Unavailable | Self::Rejected(_) => 0,
        }
    }
}

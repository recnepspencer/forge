use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use worth_store::physical_runtime::BoundedRecoveryFilesystemDiscovery;
use worth_store_physical_format::{
    durable_artifact_checksum, BoundedSegmentMembershipBlockDecodeDenial,
    DurablePhysicalRootManifest, PhysicalRecordFormatDeclaration, PhysicalSegmentMembershipBlock,
    RecordArtifactFile, RecordSegmentPageManifestEntry, SegmentMembershipBlockDecodeLimits,
    SegmentPageKey,
};
use worth_store_recovery_physics::PhysicalRedoTarget;

use super::page_observation::PageObservationFailure;

#[derive(Debug, Clone, Copy)]
pub(super) struct ResolvedSegmentPage {
    pub(super) entry: RecordSegmentPageManifestEntry,
    pub(super) routing_identity: [u8; 32],
    pub(super) membership_artifact: RecordArtifactFile,
}

pub(super) struct ManifestEntryBudget {
    remaining: u64,
}

impl ManifestEntryBudget {
    pub(super) const fn new(remaining: u64) -> Self {
        Self { remaining }
    }

    pub(super) fn admit_pending_block_read(&self) -> Result<(), PageObservationFailure> {
        (self.remaining != 0)
            .then_some(())
            .ok_or(PageObservationFailure::ManifestEntryLimit)
    }

    pub(super) fn consume(&mut self, entries: usize) -> Result<(), PageObservationFailure> {
        self.remaining = self
            .remaining
            .checked_sub(entries as u64)
            .ok_or(PageObservationFailure::ManifestEntryLimit)?;
        Ok(())
    }

    pub(super) const fn remaining(&self) -> u64 {
        self.remaining
    }
}

pub(super) fn resolve_inline_targets<'target>(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    root: &DurablePhysicalRootManifest,
    targets: impl Iterator<Item = &'target PhysicalRedoTarget>,
    format: PhysicalRecordFormatDeclaration,
    entry_budget: &mut ManifestEntryBudget,
    byte_limit: u64,
) -> Result<BTreeMap<(u64, u64), ResolvedSegmentPage>, PageObservationFailure> {
    let target_keys = targets
        .filter_map(|target| match target.identity() {
            worth_store_recovery_physics::PhysicalRedoTargetIdentity::InlinePage {
                segment,
                page,
                ..
            } => Some((segment, page)),
            worth_store_recovery_physics::PhysicalRedoTargetIdentity::ExtentChunk { .. } => None,
        })
        .collect::<BTreeSet<_>>();
    if target_keys.is_empty() {
        return Ok(BTreeMap::new());
    }
    let root_artifact = RecordArtifactFile::RootManifest {
        generation: root.generation(),
    };
    let root_reference = root
        .segment_root()
        .ok_or(PageObservationFailure::InvalidManifest {
            target: None,
            artifact: root_artifact,
        })?;
    let mut pending = VecDeque::from([root_reference]);
    let mut visited = BTreeSet::new();
    let mut resolved = BTreeMap::new();
    while let Some(reference) = pending.pop_front() {
        entry_budget.admit_pending_block_read()?;
        let reference_key = (reference.generation(), reference.block());
        let membership_artifact = RecordArtifactFile::SegmentMembershipBlock {
            generation: reference.generation(),
            block: reference.block(),
        };
        if !visited.insert(reference_key) {
            return Err(PageObservationFailure::InvalidManifest {
                target: None,
                artifact: membership_artifact,
            });
        }
        let bytes = super::page_observation::required(
            discovery.read_segment_membership_block(
                reference.generation(),
                reference.block(),
                byte_limit,
            ),
            None,
            membership_artifact,
        )?;
        let (block, found_format) = PhysicalSegmentMembershipBlock::decode_bounded(
            &bytes,
            root.node_capacity(),
            SegmentMembershipBlockDecodeLimits {
                leaf_entries: entry_budget.remaining(),
                branch_children: entry_budget.remaining(),
            },
        )
        .map_err(|denial| match denial {
            BoundedSegmentMembershipBlockDecodeDenial::LeafEntries { .. }
            | BoundedSegmentMembershipBlockDecodeDenial::BranchChildren { .. } => {
                PageObservationFailure::ManifestEntryLimit
            }
            BoundedSegmentMembershipBlockDecodeDenial::Format(_) => {
                PageObservationFailure::InvalidManifest {
                    target: None,
                    artifact: membership_artifact,
                }
            }
        })?;
        if found_format != format
            || block.tree_identity() != root.tree_identity()
            || block.reference(durable_artifact_checksum(&bytes)) != reference
        {
            return Err(PageObservationFailure::InvalidManifest {
                target: None,
                artifact: membership_artifact,
            });
        }
        if let Some(entries) = block.entries() {
            entry_budget.consume(entries.len())?;
            for entry in entries {
                let key = (entry.page_cell().segment_id().get(), entry.page().get());
                let resolved_page = ResolvedSegmentPage {
                    entry: *entry,
                    routing_identity: routing_identity(root, format, reference, *entry),
                    membership_artifact,
                };
                if target_keys.contains(&key) && resolved.insert(key, resolved_page).is_some() {
                    return Err(PageObservationFailure::InvalidManifest {
                        target: None,
                        artifact: membership_artifact,
                    });
                }
            }
        } else if let Some(children) = block.children() {
            entry_budget.consume(children.len())?;
            pending.extend(children.iter().copied().filter(|child| {
                target_keys.iter().any(|&(segment, page)| {
                    let segment = worth_store_physical_format::PhysicalSegmentId::from_raw(segment)
                        .expect("redo target carries nonzero segment identity");
                    let page = worth_store_physical_format::PhysicalPageId::from_raw(page)
                        .expect("redo target carries nonzero page identity");
                    child.contains(SegmentPageKey::new(segment, page))
                })
            }));
        }
    }
    if resolved.len() != target_keys.len() {
        return Err(PageObservationFailure::MissingArtifact {
            target: None,
            artifact: root_artifact,
        });
    }
    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use super::{ManifestEntryBudget, PageObservationFailure};

    #[test]
    fn exhausted_branch_budget_denies_the_child_before_its_read() {
        let mut budget = ManifestEntryBudget::new(1);
        assert_eq!(budget.admit_pending_block_read(), Ok(()));
        assert_eq!(budget.consume(1), Ok(()));
        assert_eq!(
            budget.admit_pending_block_read(),
            Err(PageObservationFailure::ManifestEntryLimit)
        );
    }
}

fn routing_identity(
    root: &DurablePhysicalRootManifest,
    format: PhysicalRecordFormatDeclaration,
    reference: worth_store_physical_format::SegmentManifestBlockReference,
    entry: RecordSegmentPageManifestEntry,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth.store.recovery.segment-page-routing.v1");
    digest.update(root.encode(format));
    digest.update(reference.generation().to_le_bytes());
    digest.update(reference.block().to_le_bytes());
    digest.update(reference.level().to_le_bytes());
    digest.update(reference.checksum().to_le_bytes());
    digest.update(reference.first().segment().get().to_le_bytes());
    digest.update(reference.first().page().get().to_le_bytes());
    digest.update(reference.last().segment().get().to_le_bytes());
    digest.update(reference.last().page().get().to_le_bytes());
    digest.update(entry.page_cell().segment_id().get().to_le_bytes());
    digest.update(entry.page().get().to_le_bytes());
    digest.update(entry.page_generation().to_le_bytes());
    digest.update(entry.data_generation().to_le_bytes());
    digest.update(entry.data_page_count().to_le_bytes());
    digest.update(entry.frame_index().to_le_bytes());
    digest.finalize().into()
}

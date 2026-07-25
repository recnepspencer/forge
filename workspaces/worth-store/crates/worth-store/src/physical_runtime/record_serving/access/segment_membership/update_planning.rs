use std::collections::BTreeMap;

use worth_store_physical_format::{
    durable_artifact_checksum, DurablePhysicalRootManifest, PhysicalSegmentMembershipBlock,
    RecordArtifactFile, RecordSegmentPageManifestEntry, SegmentManifestBlockReference,
    SegmentPageKey,
};

use super::SegmentMembershipReader;
use crate::physical_runtime::record_serving::{
    access::manifest_routing::{ManifestDiscoveryCounterSnapshot, ManifestLookupFailure},
    residency::{frame_loading::CanonicalFrameReadSource, frame_ports::RecordFramePorts},
    AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy,
};

pub(in crate::physical_runtime::record_serving) struct SegmentMembershipPublicationPlan {
    pub(in crate::physical_runtime::record_serving) root: Option<SegmentManifestBlockReference>,
    pub(in crate::physical_runtime::record_serving) next_block: u64,
    pub(in crate::physical_runtime::record_serving) blocks: Vec<(RecordArtifactFile, Vec<u8>)>,
    pub(in crate::physical_runtime::record_serving) discovery: ManifestDiscoveryCounterSnapshot,
}

pub(in crate::physical_runtime::record_serving) struct SegmentMembershipUpdateContext<'plan> {
    pub(in crate::physical_runtime::record_serving) frame_ports: RecordFramePorts,
    pub(in crate::physical_runtime::record_serving) source: CanonicalFrameReadSource,
    pub(in crate::physical_runtime::record_serving) format: AdmittedPhysicalRecordFormat,
    pub(in crate::physical_runtime::record_serving) access: AdmittedRecordAccessPolicy,
    pub(in crate::physical_runtime::record_serving) current: &'plan DurablePhysicalRootManifest,
    pub(in crate::physical_runtime::record_serving) successor_generation: u64,
    pub(in crate::physical_runtime::record_serving) successor_capacity: u16,
}

pub(in crate::physical_runtime::record_serving) fn plan_segment_membership_updates(
    context: SegmentMembershipUpdateContext<'_>,
    updates: BTreeMap<SegmentPageKey, RecordSegmentPageManifestEntry>,
) -> Result<SegmentMembershipPublicationPlan, ManifestLookupFailure> {
    let SegmentMembershipUpdateContext {
        frame_ports,
        source,
        format,
        access,
        current,
        successor_generation,
        successor_capacity,
    } = context;
    if !crate::physical_runtime::record_serving::planning::policy_units::
        manifest_capacity_can_branch(successor_capacity)
        || !crate::physical_runtime::record_serving::planning::policy_units::
            manifest_capacity_can_branch(current.node_capacity())
    {
        return Err(ManifestLookupFailure::Damaged);
    }
    let reader =
        SegmentMembershipReader::serving(frame_ports, source, format, access, current.clone());
    let mut planner = SegmentMembershipUpdatePlanner {
        reader: &reader,
        current,
        successor_generation,
        successor_capacity,
        next_block: current.next_segment_block(),
        blocks: Vec::new(),
        discovery: ManifestDiscoveryCounterSnapshot::default(),
    };
    let mut roots = match current.segment_root() {
        Some(root) if updates.is_empty() && successor_capacity == current.node_capacity() => {
            vec![root]
        }
        Some(root) if successor_capacity != current.node_capacity() => {
            planner.rewrite_all(root, &updates)?
        }
        Some(root) => planner.rewrite(root, &updates)?,
        None => planner.write_leaves(updates.into_values().collect())?,
    };
    while roots.len() > 1 {
        roots = planner.write_branch_level(roots)?;
    }
    Ok(SegmentMembershipPublicationPlan {
        root: roots.pop(),
        next_block: planner.next_block,
        blocks: planner.blocks,
        discovery: planner.discovery,
    })
}

struct SegmentMembershipUpdatePlanner<'reader> {
    reader: &'reader SegmentMembershipReader<'reader>,
    current: &'reader DurablePhysicalRootManifest,
    successor_generation: u64,
    successor_capacity: u16,
    next_block: u64,
    blocks: Vec<(RecordArtifactFile, Vec<u8>)>,
    discovery: ManifestDiscoveryCounterSnapshot,
}

impl SegmentMembershipUpdatePlanner<'_> {
    fn rewrite_all(
        &mut self,
        reference: SegmentManifestBlockReference,
        updates: &BTreeMap<SegmentPageKey, RecordSegmentPageManifestEntry>,
    ) -> Result<Vec<SegmentManifestBlockReference>, ManifestLookupFailure> {
        match self.reader.read_block(reference, &mut self.discovery)? {
            PhysicalSegmentMembershipBlock::Leaf { entries, .. } => {
                let mut merged = entries
                    .into_iter()
                    .map(|entry| (SegmentPageKey::from(entry), entry))
                    .collect::<BTreeMap<_, _>>();
                merged.extend(updates.iter().map(|(key, entry)| (*key, *entry)));
                self.write_leaves(merged.into_values().collect())
            }
            PhysicalSegmentMembershipBlock::Branch { children, .. } => {
                let assigned = self.assign_updates(&children, updates);
                let mut rewritten = Vec::new();
                for (child, child_updates) in children.into_iter().zip(assigned) {
                    rewritten.extend(self.rewrite_all(child, &child_updates)?);
                }
                self.write_branch_level(rewritten)
            }
        }
    }

    fn rewrite(
        &mut self,
        reference: SegmentManifestBlockReference,
        updates: &BTreeMap<SegmentPageKey, RecordSegmentPageManifestEntry>,
    ) -> Result<Vec<SegmentManifestBlockReference>, ManifestLookupFailure> {
        match self.reader.read_block(reference, &mut self.discovery)? {
            PhysicalSegmentMembershipBlock::Leaf { entries, .. } => {
                let mut merged = entries
                    .into_iter()
                    .map(|entry| (SegmentPageKey::from(entry), entry))
                    .collect::<BTreeMap<_, _>>();
                merged.extend(updates.iter().map(|(key, entry)| (*key, *entry)));
                self.write_leaves(merged.into_values().collect())
            }
            PhysicalSegmentMembershipBlock::Branch { children, .. } => {
                self.rewrite_children(children, updates)
            }
        }
    }

    fn rewrite_children(
        &mut self,
        children: Vec<SegmentManifestBlockReference>,
        updates: &BTreeMap<SegmentPageKey, RecordSegmentPageManifestEntry>,
    ) -> Result<Vec<SegmentManifestBlockReference>, ManifestLookupFailure> {
        let assigned = self.assign_updates(&children, updates);
        let mut rewritten = Vec::new();
        for (child, child_updates) in children.into_iter().zip(assigned) {
            if child_updates.is_empty() {
                rewritten.push(child);
            } else {
                rewritten.extend(self.rewrite(child, &child_updates)?);
            }
        }
        self.write_branch_level(rewritten)
    }

    fn assign_updates(
        &mut self,
        children: &[SegmentManifestBlockReference],
        updates: &BTreeMap<SegmentPageKey, RecordSegmentPageManifestEntry>,
    ) -> Vec<BTreeMap<SegmentPageKey, RecordSegmentPageManifestEntry>> {
        let mut assigned = vec![BTreeMap::new(); children.len()];
        for (key, entry) in updates {
            let (index, comparisons) =
                crate::physical_runtime::record_serving::access::counted_search::partition_point(
                    children,
                    |child| child.last() < *key,
                );
            self.discovery.observe_comparisons(comparisons);
            let index = index.min(children.len().saturating_sub(1));
            assigned[index].insert(*key, *entry);
        }
        assigned
    }

    fn write_leaves(
        &mut self,
        entries: Vec<RecordSegmentPageManifestEntry>,
    ) -> Result<Vec<SegmentManifestBlockReference>, ManifestLookupFailure> {
        let capacity = usize::from(self.successor_capacity);
        entries
            .chunks(capacity)
            .map(|chunk| {
                let block_id = self.allocate_block()?;
                let block = PhysicalSegmentMembershipBlock::leaf(
                    self.current.tree_identity(),
                    self.successor_generation,
                    block_id,
                    chunk.to_vec(),
                    self.successor_capacity,
                )
                .ok_or(ManifestLookupFailure::Damaged)?;
                Ok(self.stage(block))
            })
            .collect()
    }

    fn write_branch_level(
        &mut self,
        children: Vec<SegmentManifestBlockReference>,
    ) -> Result<Vec<SegmentManifestBlockReference>, ManifestLookupFailure> {
        let capacity = usize::from(self.successor_capacity);
        children
            .chunks(capacity)
            .map(|chunk| {
                let block_id = self.allocate_block()?;
                let level = chunk[0]
                    .level()
                    .checked_add(1)
                    .ok_or(ManifestLookupFailure::Damaged)?;
                let block = PhysicalSegmentMembershipBlock::branch(
                    self.current.tree_identity(),
                    self.successor_generation,
                    block_id,
                    level,
                    chunk.to_vec(),
                    self.successor_capacity,
                )
                .ok_or(ManifestLookupFailure::Damaged)?;
                Ok(self.stage(block))
            })
            .collect()
    }

    fn allocate_block(&mut self) -> Result<u64, ManifestLookupFailure> {
        let block = self.next_block;
        self.next_block = self
            .next_block
            .checked_add(1)
            .ok_or(ManifestLookupFailure::Damaged)?;
        Ok(block)
    }

    fn stage(&mut self, block: PhysicalSegmentMembershipBlock) -> SegmentManifestBlockReference {
        let bytes = block.encode(self.reader.format_declaration());
        let reference = block.reference(durable_artifact_checksum(&bytes));
        self.blocks.push((
            RecordArtifactFile::SegmentMembershipBlock {
                generation: self.successor_generation,
                block: block.block(),
            },
            bytes,
        ));
        reference
    }
}

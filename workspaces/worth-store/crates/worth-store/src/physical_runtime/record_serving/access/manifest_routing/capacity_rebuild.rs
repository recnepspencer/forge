use std::collections::BTreeMap;

use worth_store_physical_format::{
    durable_artifact_checksum, CurrentPhysicalRecordPlacement, ManifestBlockReference,
    PersistedRecordIdentity, PhysicalRootRoutingBlock, RecordArtifactFile,
};

use super::{ManifestDiscoveryCounterSnapshot, ManifestLookupFailure, ManifestReader};

pub(in crate::physical_runtime::record_serving) struct CapacityRebuild {
    pub(in crate::physical_runtime::record_serving) root: Option<ManifestBlockReference>,
    pub(in crate::physical_runtime::record_serving) next_block: u64,
    pub(in crate::physical_runtime::record_serving) blocks: Vec<(RecordArtifactFile, Vec<u8>)>,
    pub(in crate::physical_runtime::record_serving) discovery: ManifestDiscoveryCounterSnapshot,
    pub(in crate::physical_runtime::record_serving) inserted: u64,
}

pub(in crate::physical_runtime::record_serving) fn rebuild_capacity(
    reader: &ManifestReader<'_>,
    allocation: &worth_store_buffer_pool::OperationAllocationGrant,
    current_root: ManifestBlockReference,
    tree_identity: u64,
    generation: u64,
    capacity: u16,
    next_block: u64,
    mut updates: BTreeMap<PersistedRecordIdentity, CurrentPhysicalRecordPlacement>,
) -> Result<CapacityRebuild, ManifestLookupFailure> {
    if !super::super::super::planning::policy_units::manifest_capacity_can_branch(capacity) {
        return Err(ManifestLookupFailure::Damaged);
    }
    let mut writer = StreamingTreeWriter::new(
        reader.format_declaration(),
        tree_identity,
        generation,
        capacity,
        next_block,
    );
    let mut discovery = ManifestDiscoveryCounterSnapshot::default();
    let mut inserted = 0_u64;
    walk_entries(
        reader,
        allocation,
        current_root,
        &mut discovery,
        &mut updates,
        &mut inserted,
        &mut writer,
    )?;
    for (_, placement) in updates {
        inserted = inserted.saturating_add(1);
        writer.push_entry(placement)?;
    }
    let root = writer.finish()?;
    Ok(CapacityRebuild {
        root,
        next_block: writer.next_block,
        blocks: writer.blocks,
        discovery,
        inserted,
    })
}

fn walk_entries(
    reader: &ManifestReader<'_>,
    allocation: &worth_store_buffer_pool::OperationAllocationGrant,
    reference: ManifestBlockReference,
    discovery: &mut ManifestDiscoveryCounterSnapshot,
    updates: &mut BTreeMap<PersistedRecordIdentity, CurrentPhysicalRecordPlacement>,
    inserted: &mut u64,
    writer: &mut StreamingTreeWriter,
) -> Result<(), ManifestLookupFailure> {
    match reader.read_block(allocation, reference, discovery)? {
        PhysicalRootRoutingBlock::Leaf { entries, .. } => {
            for existing in entries {
                while updates
                    .first_key_value()
                    .is_some_and(|(record, _)| *record < existing.record())
                {
                    let (_, placement) = updates.pop_first().expect("first update exists");
                    *inserted = inserted.saturating_add(1);
                    writer.push_entry(placement)?;
                }
                let selected = updates.remove(&existing.record()).unwrap_or(existing);
                writer.push_entry(selected)?;
            }
            Ok(())
        }
        PhysicalRootRoutingBlock::Branch { children, .. } => {
            for child in children {
                walk_entries(
                    reader, allocation, child, discovery, updates, inserted, writer,
                )?;
            }
            Ok(())
        }
    }
}

struct StreamingTreeWriter {
    format: worth_store_physical_format::PhysicalRecordFormatDeclaration,
    tree_identity: u64,
    generation: u64,
    capacity: usize,
    next_block: u64,
    blocks: Vec<(RecordArtifactFile, Vec<u8>)>,
    pending_entries: Vec<CurrentPhysicalRecordPlacement>,
    pending_levels: Vec<Vec<ManifestBlockReference>>,
}

impl StreamingTreeWriter {
    fn new(
        format: worth_store_physical_format::PhysicalRecordFormatDeclaration,
        tree_identity: u64,
        generation: u64,
        capacity: u16,
        next_block: u64,
    ) -> Self {
        Self {
            format,
            tree_identity,
            generation,
            capacity: usize::from(capacity),
            next_block,
            blocks: Vec::new(),
            pending_entries: Vec::with_capacity(usize::from(capacity)),
            pending_levels: Vec::new(),
        }
    }

    fn push_entry(
        &mut self,
        placement: CurrentPhysicalRecordPlacement,
    ) -> Result<(), ManifestLookupFailure> {
        self.pending_entries.push(placement);
        if self.pending_entries.len() == self.capacity {
            self.flush_leaf()?;
        }
        Ok(())
    }

    fn flush_leaf(&mut self) -> Result<(), ManifestLookupFailure> {
        if self.pending_entries.is_empty() {
            return Ok(());
        }
        let entries =
            std::mem::replace(&mut self.pending_entries, Vec::with_capacity(self.capacity));
        let block = PhysicalRootRoutingBlock::leaf(
            self.tree_identity,
            self.generation,
            self.allocate_block()?,
            entries,
            self.capacity as u16,
        )
        .ok_or(ManifestLookupFailure::Damaged)?;
        let reference = self.stage(block);
        self.push_reference(0, reference)
    }

    fn push_reference(
        &mut self,
        level: usize,
        reference: ManifestBlockReference,
    ) -> Result<(), ManifestLookupFailure> {
        if self.pending_levels.len() <= level {
            self.pending_levels.resize_with(level + 1, Vec::new);
        }
        self.pending_levels[level].push(reference);
        if self.pending_levels[level].len() == self.capacity {
            self.flush_level(level)?;
        }
        Ok(())
    }

    fn flush_level(&mut self, level: usize) -> Result<(), ManifestLookupFailure> {
        let children = std::mem::take(&mut self.pending_levels[level]);
        let block = PhysicalRootRoutingBlock::branch(
            self.tree_identity,
            self.generation,
            self.allocate_block()?,
            u16::try_from(level + 1).map_err(|_| ManifestLookupFailure::Damaged)?,
            children,
            self.capacity as u16,
        )
        .ok_or(ManifestLookupFailure::Damaged)?;
        let reference = self.stage(block);
        self.push_reference(level + 1, reference)
    }

    fn finish(&mut self) -> Result<Option<ManifestBlockReference>, ManifestLookupFailure> {
        self.flush_leaf()?;
        loop {
            let total = self.pending_levels.iter().map(Vec::len).sum::<usize>();
            if total == 0 {
                return Ok(None);
            }
            if total == 1 {
                return Ok(self.pending_levels.iter_mut().find_map(Vec::pop));
            }
            let level = self
                .pending_levels
                .iter()
                .position(|references| !references.is_empty())
                .expect("a pending level exists");
            self.flush_level(level)?;
        }
    }

    fn allocate_block(&mut self) -> Result<u64, ManifestLookupFailure> {
        let block = self.next_block;
        self.next_block = block.checked_add(1).ok_or(ManifestLookupFailure::Damaged)?;
        Ok(block)
    }

    fn stage(&mut self, block: PhysicalRootRoutingBlock) -> ManifestBlockReference {
        let bytes = block.encode(self.format);
        let reference = block.reference(durable_artifact_checksum(&bytes));
        self.blocks.push((
            RecordArtifactFile::RootRoutingBlock {
                generation: self.generation,
                block: block.block(),
            },
            bytes,
        ));
        reference
    }
}

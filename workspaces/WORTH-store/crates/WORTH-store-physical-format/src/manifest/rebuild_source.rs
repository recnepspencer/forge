use crate::{
    ManifestDiscoveryCounterSnapshot, PhysicalPageId, PhysicalRootManifest, PhysicalRootReference,
    PhysicalSegmentId, SegmentPageManifestEntry,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalRootManifestRebuildRow {
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
}

impl PhysicalRootManifestRebuildRow {
    fn new(entry: SegmentPageManifestEntry, _root_reference: PhysicalRootReference) -> Self {
        let slot = entry.page_slot();
        Self {
            segment_id: slot.segment_id(),
            page_id: slot.page_id(),
        }
    }

    pub const fn segment_id(&self) -> PhysicalSegmentId {
        self.segment_id
    }

    pub const fn page_id(&self) -> PhysicalPageId {
        self.page_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalRootManifestRebuildWitness {
    manifest: PhysicalRootManifest,
    rows: Vec<PhysicalRootManifestRebuildRow>,
    counter_shape: Vec<u64>,
}

impl PhysicalRootManifestRebuildWitness {
    pub fn admit(manifest: PhysicalRootManifest) -> Self {
        let mut rows = manifest
            .page_slots()
            .iter()
            .copied()
            .map(|entry| {
                PhysicalRootManifestRebuildRow::new(
                    entry,
                    manifest.root_publication().root_reference(),
                )
            })
            .collect::<Vec<_>>();
        rows.sort_by(|left, right| {
            left.segment_id()
                .get()
                .cmp(&right.segment_id().get())
                .then_with(|| left.page_id().get().cmp(&right.page_id().get()))
        });

        Self {
            counter_shape: counter_shape(manifest.publish_counters(), rows.len()),
            manifest,
            rows,
        }
    }

    pub const fn manifest(&self) -> &PhysicalRootManifest {
        &self.manifest
    }

    pub fn rows(&self) -> &[PhysicalRootManifestRebuildRow] {
        &self.rows
    }

    pub fn counter_shape(&self) -> &[u64] {
        &self.counter_shape
    }
}

fn counter_shape(counters: ManifestDiscoveryCounterSnapshot, row_count: usize) -> Vec<u64> {
    let mut shape = vec![
        u64::from(counters.root_manifest_publish_count()),
        u64::from(counters.root_manifest_entry_count()),
        u64::from(counters.segment_manifest_entry_count()),
        u64::from(counters.extent_manifest_entry_count()),
        u64::from(counters.allocation_class_entry_count()),
        u64::from(counters.free_space_map_entry_count()),
        row_count as u64,
    ];
    shape.sort_unstable();
    shape
}

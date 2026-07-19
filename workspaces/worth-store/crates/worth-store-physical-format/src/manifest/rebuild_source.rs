use crate::{
    ManifestDiscoveryCounterSnapshot, PhysicalPageId, PhysicalReference, PhysicalRootManifest,
    PhysicalRootReference, PhysicalSegmentId, PhysicalStoreIdentity, SegmentPageManifestEntry,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalRootManifestRebuildRow {
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
    value_fingerprint: String,
}

impl PhysicalRootManifestRebuildRow {
    fn new(entry: SegmentPageManifestEntry, root_reference: PhysicalRootReference) -> Self {
        let slot = entry.page_slot();
        Self {
            segment_id: slot.segment_id(),
            page_id: slot.page_id(),
            value_fingerprint: format!(
                "root:{}:segment:{}:page:{}:slot:{}:generation:{}",
                root_reference.get(),
                slot.segment_id().get(),
                slot.page_id().get(),
                slot.slot().get(),
                slot.generation().get(),
            ),
        }
    }

    pub const fn segment_id(&self) -> PhysicalSegmentId {
        self.segment_id
    }

    pub const fn page_id(&self) -> PhysicalPageId {
        self.page_id
    }

    pub fn value_fingerprint(&self) -> &str {
        &self.value_fingerprint
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalRootManifestRebuildWitness {
    manifest: PhysicalRootManifest,
    rows: Vec<PhysicalRootManifestRebuildRow>,
    counter_shape: Vec<u64>,
}

/// Store-issued source for rebuilding a derived index from the current root manifest.
///
/// The source can only be issued by an opened [`crate::InMemoryPhysicalFormatModel`]. This keeps the
/// decoded manifest and the physical Store identity on one authority path instead of allowing a
/// caller to pair an independently built manifest witness with copied Store metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalRootManifestRebuildSource {
    witness: PhysicalRootManifestRebuildWitness,
    store_identity: PhysicalStoreIdentity,
}

impl PhysicalRootManifestRebuildSource {
    pub(crate) fn issue(
        manifest: PhysicalRootManifest,
        store_identity: PhysicalStoreIdentity,
    ) -> Self {
        Self {
            witness: PhysicalRootManifestRebuildWitness::admit(manifest),
            store_identity,
        }
    }

    pub const fn witness(&self) -> &PhysicalRootManifestRebuildWitness {
        &self.witness
    }

    pub const fn store_identity(&self) -> &PhysicalStoreIdentity {
        &self.store_identity
    }

    pub fn store_authority_identity(&self) -> worth_store_authority::StoreCurrentAuthorityIdentity {
        self.store_identity.authority_identity()
    }
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

    pub const fn root_reference(&self) -> PhysicalReference {
        PhysicalReference::from_root_publication_cell(self.manifest.root_publication())
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

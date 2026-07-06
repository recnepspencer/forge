use crate::{
    BlobChunkRootPublication, BlobCorruptionClassification, BlobGeneration,
    BlobGenerationRegistryAuthority, BlobGenerationRegistryCounterSnapshot,
    BlobGenerationRegistryDenial, BlobObjectClassification, BlobObjectClassificationAdmission,
    BlobObjectId, ChunkTreeRoot, LifecycleReceipt, LogicalContentDigest,
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BlobGenerationRegistry {
    entries: Vec<BlobGenerationRegistryEntry>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobGenerationRegistryAdmission {
    root_publication: BlobChunkRootPublication,
    lifecycle_receipt: LifecycleReceipt,
    classification_admission: BlobObjectClassificationAdmission,
    counters: BlobGenerationRegistryCounterSnapshot,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobGenerationRegistryEntry {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    classification: BlobObjectClassification,
    lifecycle_receipt: LifecycleReceipt,
    counters: BlobGenerationRegistryCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobGenerationObservation<'a> {
    object_id: &'a BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: &'a ChunkTreeRoot,
    logical_content_digest: &'a LogicalContentDigest,
    classification: BlobObjectClassification,
    lifecycle_receipt: &'a LifecycleReceipt,
    counters: BlobGenerationRegistryCounterSnapshot,
}

impl BlobGenerationRegistry {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn publish(
        &mut self,
        admission: BlobGenerationRegistryAdmission,
        authority: BlobGenerationRegistryAuthority,
    ) -> Result<&BlobGenerationRegistryEntry, BlobGenerationRegistryDenial> {
        let _current_authority = authority.into_current_authority();
        admission.reject_if_lifecycle_inputs_diverged()?;
        if let Some(index) = self.find_matching_generation(&admission) {
            return self.admit_existing_binding(index, admission);
        }
        self.entries.push(admission.into_entry());
        Ok(self.entries.last().expect("entry was just inserted"))
    }

    pub fn observe_registered_generation(
        &self,
        object_id: &BlobObjectId,
        generation: BlobGeneration,
    ) -> Result<BlobGenerationObservation<'_>, BlobGenerationRegistryDenial> {
        self.entries
            .iter()
            .find(|entry| entry.object_id == *object_id && entry.generation == generation)
            .map(BlobGenerationRegistryEntry::observe)
            .ok_or(BlobGenerationRegistryDenial::BlobGenerationNotPublished {
                counters: BlobGenerationRegistryCounterSnapshot::start().record_denial(),
            })
    }

    fn find_matching_generation(
        &self,
        admission: &BlobGenerationRegistryAdmission,
    ) -> Option<usize> {
        let declaration = admission.lifecycle_receipt.declaration();
        self.entries.iter().position(|entry| {
            entry.object_id == *declaration.object_id()
                && entry.generation == declaration.generation()
        })
    }

    fn admit_existing_binding(
        &self,
        index: usize,
        admission: BlobGenerationRegistryAdmission,
    ) -> Result<&BlobGenerationRegistryEntry, BlobGenerationRegistryDenial> {
        let existing = &self.entries[index];
        if existing.same_registry_binding_as(&admission) {
            return Ok(existing);
        }
        Err(
            BlobGenerationRegistryDenial::BlobGenerationAlreadyBoundDifferently {
                counters: admission.counters.record_denial(),
            },
        )
    }
}

impl BlobGenerationRegistryAdmission {
    pub fn from_executed_lifecycle(
        root_publication: BlobChunkRootPublication,
        lifecycle_receipt: LifecycleReceipt,
        classification_admission: BlobObjectClassificationAdmission,
    ) -> Self {
        Self {
            root_publication,
            lifecycle_receipt,
            classification_admission,
            counters: BlobGenerationRegistryCounterSnapshot::start(),
        }
    }

    pub fn publish<'a>(
        self,
        registry: &'a mut BlobGenerationRegistry,
        authority: BlobGenerationRegistryAuthority,
    ) -> Result<&'a BlobGenerationRegistryEntry, BlobGenerationRegistryDenial> {
        registry.publish(self, authority)
    }

    fn reject_if_lifecycle_inputs_diverged(&self) -> Result<(), BlobGenerationRegistryDenial> {
        let declaration = self.lifecycle_receipt.declaration();
        if self.root_publication.chunk_tree_root() != declaration.chunk_tree_root() {
            return Err(
                BlobGenerationRegistryDenial::RootPublicationLifecycleRootMismatch {
                    counters: self.counters.record_denial(),
                },
            );
        }
        if self.root_publication.logical_content_digest() != declaration.logical_content_digest() {
            return Err(
                BlobGenerationRegistryDenial::RootPublicationLifecycleDigestMismatch {
                    counters: self.counters.record_denial(),
                },
            );
        }
        if self.classification_admission.object_id() != declaration.object_id()
            || self.classification_admission.generation() != declaration.generation()
            || self.classification_admission.chunk_tree_root() != declaration.chunk_tree_root()
            || self.classification_admission.logical_content_digest()
                != declaration.logical_content_digest()
            || self.classification_admission.classification()
                != BlobObjectClassificationAdmission::from_executed_lifecycle(
                    &self.lifecycle_receipt,
                )
                .classification()
        {
            return Err(
                BlobGenerationRegistryDenial::ClassificationLifecycleBindingMismatch {
                    counters: self.counters.record_denial(),
                },
            );
        }
        Ok(())
    }

    fn into_entry(self) -> BlobGenerationRegistryEntry {
        let declaration = self.lifecycle_receipt.declaration();
        BlobGenerationRegistryEntry {
            object_id: declaration.object_id().clone(),
            generation: declaration.generation(),
            chunk_tree_root: self.root_publication.chunk_tree_root().clone(),
            logical_content_digest: self.root_publication.logical_content_digest().clone(),
            classification: self.classification_admission.classification(),
            lifecycle_receipt: self.lifecycle_receipt,
            counters: self.counters.record_publication(),
        }
    }
}

impl BlobGenerationRegistryEntry {
    pub const fn observe(&self) -> BlobGenerationObservation<'_> {
        BlobGenerationObservation {
            object_id: &self.object_id,
            generation: self.generation,
            chunk_tree_root: &self.chunk_tree_root,
            logical_content_digest: &self.logical_content_digest,
            classification: self.classification,
            lifecycle_receipt: &self.lifecycle_receipt,
            counters: self.counters.record_observation(),
        }
    }

    pub const fn classify_blob_corruption(&self) -> BlobCorruptionClassification {
        BlobCorruptionClassification::new(
            self.classification,
            self.counters.record_classification_check(),
        )
    }

    pub const fn counters(&self) -> BlobGenerationRegistryCounterSnapshot {
        self.counters
    }

    fn same_registry_binding_as(&self, admission: &BlobGenerationRegistryAdmission) -> bool {
        let declaration = admission.lifecycle_receipt.declaration();
        self.chunk_tree_root == *declaration.chunk_tree_root()
            && self.logical_content_digest == *declaration.logical_content_digest()
            && self.classification == admission.classification_admission.classification()
            && self.lifecycle_receipt == admission.lifecycle_receipt
    }
}

impl BlobGenerationObservation<'_> {
    pub const fn object_id(&self) -> &BlobObjectId {
        self.object_id
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        self.chunk_tree_root
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        self.logical_content_digest
    }

    pub const fn classification(&self) -> BlobObjectClassification {
        self.classification
    }

    pub const fn lifecycle_receipt(&self) -> &LifecycleReceipt {
        self.lifecycle_receipt
    }

    pub const fn counters(&self) -> BlobGenerationRegistryCounterSnapshot {
        self.counters
    }
}

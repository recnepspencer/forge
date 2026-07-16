use crate::capsule_readiness::counters::BlobCapsuleReadinessCounters;
use crate::capsule_readiness::declaration::{
    BlobCapsuleMaterializationPolicy, BlobCapsuleSliceDeclaration,
};
use crate::capsule_readiness::denial::BlobCapsuleReadinessDenial;
use crate::capsule_readiness::materialization::{
    admit_materialized_capsule_read, reachability_fingerprint, readiness_digest,
    validate_materialized_chunks, MaterializedBlobCapsuleBundle,
    PreparedBlobCapsuleMaterialization,
};
use crate::placement::BlobPlacementClass;
use crate::{
    AdmittedBlobPlacement, BlobChunkIdentity, BlobChunkProofLeaf, BlobChunkQuarantine,
    BlobChunkReachabilityProofSet, BlobChunkSecurityScope, BlobGeneration,
    BlobGenerationObservation, BlobObjectClassification, BlobObjectId,
    BlobStreamingReadObservation, BlobStreamingVerifiedRead, ChunkTreeRoot, LogicalContentDigest,
};

use super::{classify_blob_capsule_placement_availability, BlobCapsulePlacementAvailability};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedBlobCapsuleSlice {
    generation: BlobGeneration,
    pub(super) selected_leaves: Vec<BlobChunkProofLeaf>,
    require_parent_root_basis: bool,
    materialization_policy: BlobCapsuleMaterializationPolicy,
    reachability_fingerprint: String,
    declared_bytes: u64,
    pub(super) counters: BlobCapsuleReadinessCounters,
}

impl PlannedBlobCapsuleSlice {
    pub fn selected_leaves(&self) -> &[BlobChunkProofLeaf] {
        &self.selected_leaves
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassifiedBlobCapsuleSlice {
    pub(super) planned: PlannedBlobCapsuleSlice,
    placement_scope: worth_store_security::StoreSecurityScopeIdentity,
    pub(super) placement_class: BlobPlacementClass,
    pub(super) availability_posture: BlobCapsulePlacementAvailability,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCapsuleReadinessWitness {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    selected_chunks: Vec<BlobChunkIdentity>,
    readiness_digest: String,
    declared_bytes: u64,
    counters: BlobCapsuleReadinessCounters,
}

#[derive(Debug, Clone)]
pub struct BlobCapsuleMaterializationAuthority {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    classification: BlobObjectClassification,
    reachability: BlobChunkReachabilityProofSet,
    ordered_leaves: Vec<BlobChunkProofLeaf>,
}

impl BlobCapsuleMaterializationAuthority {
    pub const fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }
    pub const fn generation(&self) -> BlobGeneration {
        self.generation
    }
    pub const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        &self.chunk_tree_root
    }
    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub fn from_generation_observation(
        observation: &BlobGenerationObservation<'_>,
        ordered_leaves: &[BlobChunkProofLeaf],
    ) -> Result<Self, BlobCapsuleReadinessDenial> {
        let counters = BlobCapsuleReadinessCounters::start();
        if ordered_leaves.is_empty() {
            return Err(BlobCapsuleReadinessDenial::EmptySelection { counters });
        }
        if observation
            .lifecycle_receipt()
            .reachability()
            .reachable_chunks()
            .is_empty()
        {
            return Err(BlobCapsuleReadinessDenial::MissingChunk {
                ordinal: 0,
                counters,
            });
        }
        Ok(Self {
            object_id: observation.object_id().clone(),
            generation: observation.generation(),
            chunk_tree_root: observation.chunk_tree_root().clone(),
            logical_content_digest: observation.logical_content_digest().clone(),
            classification: observation.classification(),
            reachability: observation.lifecycle_receipt().reachability().clone(),
            ordered_leaves: ordered_leaves.to_vec(),
        })
    }

    pub fn plan_slice(
        &self,
        declaration: BlobCapsuleSliceDeclaration,
    ) -> Result<PlannedBlobCapsuleSlice, BlobCapsuleReadinessDenial> {
        let counters = BlobCapsuleReadinessCounters::start();
        if declaration.generation() != self.generation {
            return Err(BlobCapsuleReadinessDenial::GenerationMismatch { counters });
        }
        if !declaration.require_parent_root_basis_flag()
            && declaration.selection().ordinals().len() != self.ordered_leaves.len()
        {
            return Err(BlobCapsuleReadinessDenial::MissingParentRootBasis { counters });
        }

        let mut selected_leaves = Vec::new();
        for ordinal in declaration.selection().ordinals() {
            let leaf = self
                .ordered_leaves
                .iter()
                .find(|leaf| leaf.ordinal() == *ordinal)
                .cloned()
                .ok_or(BlobCapsuleReadinessDenial::MissingChunk {
                    ordinal: ordinal.get(),
                    counters,
                })?;
            selected_leaves.push(leaf);
        }

        let declared_bytes = selected_leaves
            .iter()
            .map(|leaf| leaf.byte_range().len())
            .sum();
        Ok(PlannedBlobCapsuleSlice {
            generation: declaration.generation(),
            selected_leaves,
            require_parent_root_basis: declaration.require_parent_root_basis_flag(),
            materialization_policy: declaration.materialization_policy(),
            reachability_fingerprint: reachability_fingerprint(&self.reachability),
            declared_bytes,
            counters: counters
                .with_planned_chunks(declaration.selection().ordinals().len() as u64)
                .with_skipped_chunks(
                    (self.ordered_leaves.len() - declaration.selection().ordinals().len()) as u64,
                )
                .with_declared_bytes(declared_bytes),
        })
    }

    pub fn classify_slice_for_materialization(
        &self,
        planned: PlannedBlobCapsuleSlice,
        current_scope: &BlobChunkSecurityScope,
        placement: &AdmittedBlobPlacement,
        quarantines: &[BlobChunkQuarantine],
    ) -> Result<ClassifiedBlobCapsuleSlice, BlobCapsuleReadinessDenial> {
        let availability_posture =
            classify_blob_capsule_placement_availability(placement.cold_state());
        if current_scope.identity() != self.reachability.security_metadata().identity() {
            return Err(BlobCapsuleReadinessDenial::StaleSecurityScope {
                counters: planned.counters.record_denied_chunk(),
            });
        }
        if placement.security_metadata().identity()
            != self.reachability.security_metadata().identity()
        {
            return Err(BlobCapsuleReadinessDenial::CrossScopeSharedChunk {
                counters: planned.counters.record_denied_chunk(),
            });
        }
        if !matches!(
            availability_posture,
            BlobCapsulePlacementAvailability::HotReady
                | BlobCapsulePlacementAvailability::ColdReady
        ) {
            return Err(BlobCapsuleReadinessDenial::ColdPlacementUnavailable {
                counters: planned.counters.record_denied_chunk(),
            });
        }
        for leaf in &planned.selected_leaves {
            if quarantines.iter().any(|quarantine| {
                quarantine.generation() == self.generation
                    && quarantine.stored_digest() == leaf.stored_digest()
            }) {
                return Err(BlobCapsuleReadinessDenial::QuarantinedChunk {
                    ordinal: leaf.ordinal().get(),
                    counters: planned.counters.record_denied_chunk(),
                });
            }
        }

        Ok(ClassifiedBlobCapsuleSlice {
            planned,
            placement_scope: placement.security_metadata().identity(),
            placement_class: placement.class(),
            availability_posture,
        })
    }

    pub fn admit_materialized_capsule_read(
        &self,
        classified: &ClassifiedBlobCapsuleSlice,
        verified_read: BlobStreamingVerifiedRead,
        observations: impl IntoIterator<Item = BlobStreamingReadObservation>,
    ) -> Result<PreparedBlobCapsuleMaterialization, BlobCapsuleReadinessDenial> {
        admit_materialized_capsule_read(self, classified, verified_read, observations)
    }

    pub fn materialize_capsule_bundle(
        &self,
        classified: ClassifiedBlobCapsuleSlice,
        live_reachability: &BlobChunkReachabilityProofSet,
        materialized: PreparedBlobCapsuleMaterialization,
    ) -> Result<MaterializedBlobCapsuleBundle, BlobCapsuleReadinessDenial> {
        if classified.planned.reachability_fingerprint
            != reachability_fingerprint(live_reachability)
        {
            return Err(
                BlobCapsuleReadinessDenial::ReachabilityChangedDuringCreation {
                    counters: classified.planned.counters.record_denied_chunk(),
                },
            );
        }

        validate_materialized_chunks(&classified, &materialized)?;

        let _ = classified.planned.require_parent_root_basis;
        let _ = classified.planned.materialization_policy;
        let _ = classified.placement_class;
        let _ = classified.availability_posture;
        Ok(MaterializedBlobCapsuleBundle {
            object_id: self.object_id.clone(),
            generation: self.generation,
            chunk_tree_root: self.chunk_tree_root.clone(),
            logical_content_digest: self.logical_content_digest.clone(),
            classification: self.classification,
            materialized_chunks: materialized.chunks,
            declared_bytes: classified.planned.declared_bytes,
            placement_scope: classified.placement_scope,
            reachability_fingerprint: classified.planned.reachability_fingerprint,
            counters: classified
                .planned
                .counters
                .record_materialized_chunks(classified.planned.selected_leaves.len() as u64),
        })
    }

    pub fn publish_capsule_readiness(
        &self,
        materialized: MaterializedBlobCapsuleBundle,
    ) -> Result<BlobCapsuleReadinessWitness, BlobCapsuleReadinessDenial> {
        let _ = materialized.classification;
        let _ = materialized.placement_scope;
        let readiness_digest = readiness_digest(
            &materialized.object_id,
            materialized.generation,
            &materialized.chunk_tree_root,
            &materialized.logical_content_digest,
            &materialized.materialized_chunks,
            materialized.declared_bytes,
            &materialized.reachability_fingerprint,
        );
        Ok(BlobCapsuleReadinessWitness {
            object_id: materialized.object_id,
            generation: materialized.generation,
            chunk_tree_root: materialized.chunk_tree_root,
            logical_content_digest: materialized.logical_content_digest,
            selected_chunks: materialized
                .materialized_chunks
                .iter()
                .map(|chunk| chunk.chunk_identity.clone())
                .collect(),
            readiness_digest,
            declared_bytes: materialized.declared_bytes,
            counters: materialized.counters.record_readiness_publication(),
        })
    }
}

impl BlobCapsuleReadinessWitness {
    pub const fn generation(&self) -> BlobGeneration {
        self.generation
    }
    pub fn readiness_digest(&self) -> &str {
        &self.readiness_digest
    }
    pub const fn declared_bytes(&self) -> u64 {
        self.declared_bytes
    }
    pub const fn counters(&self) -> BlobCapsuleReadinessCounters {
        self.counters
    }
    pub fn selected_chunks(&self) -> &[BlobChunkIdentity] {
        &self.selected_chunks
    }
    pub const fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }
    pub const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        &self.chunk_tree_root
    }
    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }
}

pub fn reject_copied_capsule_row_as_capsule_readiness(_raw: &str) -> BlobCapsuleReadinessDenial {
    BlobCapsuleReadinessDenial::CopiedCapsuleRow {
        counters: BlobCapsuleReadinessCounters::start().record_denied_chunk(),
    }
}

pub fn reject_digest_only_chunk_reference_as_capsule_readiness(
    _raw: &str,
) -> BlobCapsuleReadinessDenial {
    BlobCapsuleReadinessDenial::DigestOnlyChunkReference {
        counters: BlobCapsuleReadinessCounters::start().record_denied_chunk(),
    }
}

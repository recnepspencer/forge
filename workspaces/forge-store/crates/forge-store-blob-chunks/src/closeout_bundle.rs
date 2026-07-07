use crate::export_bundle::{
    BlobExportBundleCounters, BlobExportDigestEvidence, BlobExportOfflineChunkDeclaration,
    BlobExportPublishedBundle,
};
use crate::heavy_fixture::HeavyBlobFixtureExecutionEvidence;
use crate::handoffs::{
    BlobHarnessAccessMode, BlobHarnessActorMix, BlobHarnessChunkTopology, BlobHarnessFailurePoint,
    BlobHarnessPlacementClass, BlobHarnessSecurityScopeClass,
};
use crate::harness_execution::BlobHarnessObservedYieldpoint;
use crate::lifecycle::{BlobLifecycleCounterSnapshot, BlobLifecycleDeclaration};
use crate::{
    AdmittedBlobPlacement, BlobChunkReachabilityProofSet, BlobChunkRootPublication,
    BlobChunkSecurityMetadataWitness, BlobGeneration, BlobObjectId, ChunkTreeRoot,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S7ExecutedLifecycleEvidenceBundle {
    security_scope_class: BlobHarnessSecurityScopeClass,
    placement_class: BlobHarnessPlacementClass,
    access_mode: BlobHarnessAccessMode,
    failure_point: BlobHarnessFailurePoint,
    actor_mix: BlobHarnessActorMix,
    observed_yieldpoint: BlobHarnessObservedYieldpoint,
    declared_topology: BlobHarnessChunkTopology,
    executed_topology: BlobHarnessChunkTopology,
    allocation_bytes: u64,
    lifecycle_declaration: BlobLifecycleDeclaration,
    lifecycle_counters: BlobLifecycleCounterSnapshot,
    reachability: BlobChunkReachabilityProofSet,
    admitted_placement: AdmittedBlobPlacement,
    root_publication: BlobChunkRootPublication,
    export_object_id: BlobObjectId,
    export_generation: BlobGeneration,
    export_chunk_tree_root: ChunkTreeRoot,
    export_security_metadata: BlobChunkSecurityMetadataWitness,
    export_digest_evidence: BlobExportDigestEvidence,
    export_offline_declarations: Vec<BlobExportOfflineChunkDeclaration>,
    export_counters: BlobExportBundleCounters,
    cross_scope_dedupe_denied: bool,
    heavy_fixture_evidence: Option<HeavyBlobFixtureExecutionEvidence>,
}

impl S7ExecutedLifecycleEvidenceBundle {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_harness_execution(
        security_scope_class: BlobHarnessSecurityScopeClass,
        placement_class: BlobHarnessPlacementClass,
        access_mode: BlobHarnessAccessMode,
        failure_point: BlobHarnessFailurePoint,
        actor_mix: BlobHarnessActorMix,
        observed_yieldpoint: BlobHarnessObservedYieldpoint,
        declared_topology: BlobHarnessChunkTopology,
        executed_topology: BlobHarnessChunkTopology,
        allocation_bytes: u64,
        lifecycle_declaration: BlobLifecycleDeclaration,
        lifecycle_counters: BlobLifecycleCounterSnapshot,
        reachability: BlobChunkReachabilityProofSet,
        admitted_placement: AdmittedBlobPlacement,
        root_publication: BlobChunkRootPublication,
        export_bundle: BlobExportPublishedBundle,
        cross_scope_dedupe_denied: bool,
        heavy_fixture_evidence: Option<HeavyBlobFixtureExecutionEvidence>,
    ) -> Self {
        Self {
            security_scope_class,
            placement_class,
            access_mode,
            failure_point,
            actor_mix,
            observed_yieldpoint,
            declared_topology,
            executed_topology,
            allocation_bytes,
            lifecycle_declaration,
            lifecycle_counters,
            reachability,
            admitted_placement,
            root_publication,
            export_object_id: export_bundle.object_id().clone(),
            export_generation: export_bundle.generation(),
            export_chunk_tree_root: export_bundle.chunk_tree_root().clone(),
            export_security_metadata: export_bundle.security_metadata(),
            export_digest_evidence: export_bundle.digest_evidence().clone(),
            export_offline_declarations: export_bundle.offline_declarations().to_vec(),
            export_counters: export_bundle.counters(),
            cross_scope_dedupe_denied,
            heavy_fixture_evidence,
        }
    }

    pub const fn security_scope_class(&self) -> BlobHarnessSecurityScopeClass { self.security_scope_class }
    pub const fn placement_class(&self) -> BlobHarnessPlacementClass { self.placement_class }
    pub const fn access_mode(&self) -> BlobHarnessAccessMode { self.access_mode }
    pub const fn failure_point(&self) -> BlobHarnessFailurePoint { self.failure_point }
    pub const fn actor_mix(&self) -> BlobHarnessActorMix { self.actor_mix }
    pub const fn observed_yieldpoint(&self) -> BlobHarnessObservedYieldpoint { self.observed_yieldpoint }
    pub const fn declared_topology(&self) -> BlobHarnessChunkTopology { self.declared_topology }
    pub const fn executed_topology(&self) -> BlobHarnessChunkTopology { self.executed_topology }
    pub const fn allocation_bytes(&self) -> u64 { self.allocation_bytes }
    pub const fn lifecycle_declaration(&self) -> &BlobLifecycleDeclaration { &self.lifecycle_declaration }
    pub const fn lifecycle_counters(&self) -> BlobLifecycleCounterSnapshot { self.lifecycle_counters }
    pub const fn reachability(&self) -> &BlobChunkReachabilityProofSet { &self.reachability }
    pub const fn admitted_placement(&self) -> &AdmittedBlobPlacement { &self.admitted_placement }
    pub const fn root_publication(&self) -> &BlobChunkRootPublication { &self.root_publication }
    pub const fn export_object_id(&self) -> &BlobObjectId { &self.export_object_id }
    pub const fn export_generation(&self) -> BlobGeneration { self.export_generation }
    pub const fn export_chunk_tree_root(&self) -> &ChunkTreeRoot { &self.export_chunk_tree_root }
    pub const fn export_security_metadata(&self) -> BlobChunkSecurityMetadataWitness { self.export_security_metadata }
    pub const fn export_digest_evidence(&self) -> &BlobExportDigestEvidence { &self.export_digest_evidence }
    pub fn export_offline_declarations(&self) -> &[BlobExportOfflineChunkDeclaration] { &self.export_offline_declarations }
    pub const fn export_counters(&self) -> BlobExportBundleCounters { self.export_counters }
    pub const fn cross_scope_dedupe_denied(&self) -> bool { self.cross_scope_dedupe_denied }
    pub fn heavy_fixture_evidence(&self) -> Option<&HeavyBlobFixtureExecutionEvidence> { self.heavy_fixture_evidence.as_ref() }

    pub fn export_declared_chunk_count(&self) -> u64 {
        self.export_digest_evidence.declared_chunk_count()
    }

    pub fn export_declared_total_bytes(&self) -> u64 {
        self.export_digest_evidence.declared_total_bytes()
    }

    pub fn export_logical_digest_matches_lifecycle(&self) -> bool {
        self.export_digest_evidence.logical_content_digest() == self.lifecycle_declaration.logical_content_digest()
    }

    pub fn export_checksum_distinct_from_stored_digest(&self) -> bool {
        self.export_offline_declarations
            .first()
            .map(|chunk| {
                chunk.checksum_digest() != chunk.stored_digest()
                    && chunk.stored_digest() == self.lifecycle_declaration.stored_chunk_digest().digest().as_str()
            })
            .unwrap_or(false)
    }

    pub const fn reachability_reference_edges(&self) -> u64 {
        self.reachability.counters().reference_edges()
    }

    pub fn reachability_stored_digest_matches_lifecycle(&self) -> bool {
        self.reachability.stored_digest() == self.lifecycle_declaration.stored_chunk_digest()
    }

    pub fn root_publication_matches_lifecycle_identity(&self) -> bool {
        self.root_publication.chunk_tree_root() == self.lifecycle_declaration.chunk_tree_root()
            && self.root_publication.logical_content_digest()
                == self.lifecycle_declaration.logical_content_digest()
    }

    pub fn export_matches_root_and_lifecycle_identity(&self) -> bool {
        self.export_object_id == *self.lifecycle_declaration.object_id()
            && self.export_generation == self.lifecycle_declaration.generation()
            && self.export_chunk_tree_root == *self.lifecycle_declaration.chunk_tree_root()
            && self.export_chunk_tree_root == *self.root_publication.chunk_tree_root()
            && self.export_security_metadata == self.lifecycle_declaration.security_metadata()
    }

    pub fn reachability_matches_lifecycle_identity(&self) -> bool {
        self.reachability
            .matches_lifecycle_declaration(&self.lifecycle_declaration)
            && self.reachability.security_metadata() == self.lifecycle_declaration.security_metadata()
    }

    pub fn placement_matches_reachability(&self) -> bool {
        self.admitted_placement.matches_reachability(&self.reachability)
    }

    pub fn placement_matches_lifecycle_scope(&self) -> bool {
        self.admitted_placement.security_metadata() == self.lifecycle_declaration.security_metadata()
            && self.admitted_placement.stored_digest() == self.lifecycle_declaration.stored_chunk_digest()
    }
}

use forge_store_budgets::BlobHarnessEnvelopeProfile;

use crate::BlobChunkRootPublication;
use crate::S7ExecutedLifecycleEvidenceBundle;
use crate::heavy_fixture::{
    DeterministicBytePatternProfile, HeavyBlobFixtureExecutionEvidence,
    HeavyBlobFixtureMaterializationMode, HeavyBlobFixturePlan,
};
use crate::handoffs::{
    BlobHarnessAccessMode, BlobHarnessActorMix, BlobHarnessChunkTopology, BlobHarnessFailurePoint,
    BlobHarnessPlacementClass, BlobHarnessSecurityScopeClass, BlobHarnessSizeClass,
};
use forge_store_physical_backend::HeavyFixtureBackendProfile;

use super::chunk_sequence::build_chunk_sequence;
use super::dedupe_observation::observe_cross_scope_dedupe;
use super::export_publication::publish_export_bundle;
use super::lifecycle_execution::{execute_lifecycle, ExecutedBlobLane};
use super::placement_admission::admit_placement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobHarnessExecutionInput {
    profile: BlobHarnessEnvelopeProfile,
    size_class: BlobHarnessSizeClass,
    placement_class: BlobHarnessPlacementClass,
    security_scope_class: BlobHarnessSecurityScopeClass,
    access_mode: BlobHarnessAccessMode,
    failure_point: BlobHarnessFailurePoint,
    actor_mix: BlobHarnessActorMix,
    topology: BlobHarnessChunkTopology,
    heavy_materialization_mode: Option<HeavyBlobFixtureMaterializationMode>,
    heavy_byte_pattern_profile: Option<DeterministicBytePatternProfile>,
    heavy_backend_profile: Option<HeavyFixtureBackendProfile>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobHarnessObservedYieldpoint {
    WalAppendBeforeFlush,
    FreshRuntimeReplayOpen,
    RootPublicationBeforeObserve,
    MemoryPressureBoundary,
    IoPressureBoundary,
    OfflineVerifierLayoutWalkBeforeRuntimeRecovery,
    ShortcutRejectionBoundary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobHarnessExecutedWitness {
    executed_topology: BlobHarnessChunkTopology,
    declared_topology: BlobHarnessChunkTopology,
    allocation_bytes: u64,
    observed_yieldpoint: BlobHarnessObservedYieldpoint,
    export_declared_chunk_count: u64,
    export_declared_total_bytes: u64,
    export_logical_digest_matches_lifecycle: bool,
    export_checksum_distinct_from_stored_digest: bool,
    reachability_reference_edges: u64,
    reachability_stored_digest_matches_lifecycle: bool,
    cross_scope_dedupe_denied: bool,
    heavy_fixture_evidence: Option<HeavyBlobFixtureExecutionEvidence>,
    closeout_evidence: S7ExecutedLifecycleEvidenceBundle,
}

impl BlobHarnessExecutionInput {
    pub const fn new(
        profile: BlobHarnessEnvelopeProfile,
        size_class: BlobHarnessSizeClass,
        placement_class: BlobHarnessPlacementClass,
        security_scope_class: BlobHarnessSecurityScopeClass,
        access_mode: BlobHarnessAccessMode,
        failure_point: BlobHarnessFailurePoint,
        actor_mix: BlobHarnessActorMix,
        topology: BlobHarnessChunkTopology,
    ) -> Self {
        Self {
            profile,
            size_class,
            placement_class,
            security_scope_class,
            access_mode,
            failure_point,
            actor_mix,
            topology,
            heavy_materialization_mode: None,
            heavy_byte_pattern_profile: None,
            heavy_backend_profile: None,
        }
    }

    pub fn with_heavy_temp_file_materialization(mut self) -> Self {
        self.heavy_materialization_mode = Some(HeavyBlobFixtureMaterializationMode::TempFile);
        self
    }

    pub fn with_heavy_byte_pattern_profile(
        mut self,
        profile: DeterministicBytePatternProfile,
    ) -> Self {
        self.heavy_byte_pattern_profile = Some(profile);
        self
    }

    pub fn with_non_canonical_chaos_stress(mut self) -> Self {
        self.heavy_byte_pattern_profile = Some(DeterministicBytePatternProfile::AmbientChaosCorpus);
        self.heavy_backend_profile = Some(HeavyFixtureBackendProfile::NonCanonicalChaosCorpus);
        self
    }
}

impl BlobHarnessExecutedWitness {
    pub const fn executed_topology(&self) -> BlobHarnessChunkTopology { self.executed_topology }
    pub const fn declared_topology(&self) -> BlobHarnessChunkTopology { self.declared_topology }
    pub const fn allocation_bytes(&self) -> u64 { self.allocation_bytes }
    pub const fn observed_yieldpoint(&self) -> BlobHarnessObservedYieldpoint { self.observed_yieldpoint }
    pub const fn export_declared_chunk_count(&self) -> u64 { self.export_declared_chunk_count }
    pub const fn export_declared_total_bytes(&self) -> u64 { self.export_declared_total_bytes }
    pub const fn export_logical_digest_matches_lifecycle(&self) -> bool { self.export_logical_digest_matches_lifecycle }
    pub const fn export_checksum_distinct_from_stored_digest(&self) -> bool { self.export_checksum_distinct_from_stored_digest }
    pub const fn reachability_reference_edges(&self) -> u64 { self.reachability_reference_edges }
    pub const fn reachability_stored_digest_matches_lifecycle(&self) -> bool { self.reachability_stored_digest_matches_lifecycle }
    pub const fn cross_scope_dedupe_denied(&self) -> bool { self.cross_scope_dedupe_denied }
    pub fn heavy_fixture_evidence(&self) -> Option<&HeavyBlobFixtureExecutionEvidence> { self.heavy_fixture_evidence.as_ref() }

    pub const fn closeout_evidence(&self) -> &S7ExecutedLifecycleEvidenceBundle {
        &self.closeout_evidence
    }

    pub(crate) fn into_closeout_evidence(self) -> S7ExecutedLifecycleEvidenceBundle {
        self.closeout_evidence
    }
}

pub fn execute_s7_blob_harness(input: BlobHarnessExecutionInput) -> BlobHarnessExecutedWitness {
    let case = request_identity(&input);
    let heavy_fixture_plan = canonical_heavy_fixture_plan(&input);
    let generated = build_chunk_sequence(
        &case,
        input.security_scope_class,
        input.topology,
        heavy_fixture_plan.as_ref(),
    );
    let publication = BlobChunkRootPublication::publish(generated.sequence.clone()).expect("publication");
    let lane = execute_lifecycle(&case, input.security_scope_class, &publication, &generated);
    let placed_lane = ExecutedBlobLane {
        placement: admit_placement(&case, &lane.reachability, input.placement_class),
        ..lane
    };
    let export_bundle = publish_export_bundle(&case, &placed_lane, &publication, &generated);
    let export_logical_digest_matches_lifecycle =
        export_bundle.digest_evidence().logical_content_digest() == publication.logical_content_digest();
    let export_checksum_distinct_from_stored_digest = export_bundle
        .offline_declarations()
        .first()
        .map(|chunk| chunk.checksum_digest() != chunk.stored_digest())
        .unwrap_or(false);
    let reachability_reference_edges = placed_lane.reachability.counters().reference_edges();
    let reachability_stored_digest_matches_lifecycle =
        placed_lane.reachability.stored_digest()
            == generated.sequence.proof_frontier().ordered_leaves()[0].stored_digest();
    let cross_scope_dedupe_denied = observe_cross_scope_dedupe(&case, input.security_scope_class);
    let closeout_evidence = S7ExecutedLifecycleEvidenceBundle::from_harness_execution(
        input.security_scope_class,
        input.placement_class,
        input.access_mode,
        input.failure_point,
        input.actor_mix,
        observed_yieldpoint(input.failure_point),
        generated.declared_topology,
        generated.executed_topology,
        generated.peak_window_bytes,
        placed_lane.lifecycle.declaration().clone(),
        placed_lane.lifecycle.counters(),
        placed_lane.reachability.clone(),
        placed_lane.placement.clone(),
        publication.clone(),
        export_bundle,
        cross_scope_dedupe_denied,
        generated.heavy_fixture_evidence.clone(),
    );

    BlobHarnessExecutedWitness {
        executed_topology: generated.executed_topology,
        declared_topology: generated.declared_topology,
        allocation_bytes: generated.peak_window_bytes,
        observed_yieldpoint: observed_yieldpoint(input.failure_point),
        export_declared_chunk_count: closeout_evidence.export_declared_chunk_count(),
        export_declared_total_bytes: closeout_evidence.export_declared_total_bytes(),
        export_logical_digest_matches_lifecycle,
        export_checksum_distinct_from_stored_digest,
        reachability_reference_edges,
        reachability_stored_digest_matches_lifecycle,
        cross_scope_dedupe_denied,
        heavy_fixture_evidence: generated.heavy_fixture_evidence,
        closeout_evidence,
    }
}

fn canonical_heavy_fixture_plan(input: &BlobHarnessExecutionInput) -> Option<HeavyBlobFixturePlan> {
    let mut plan = if let Some(mode) = input.heavy_materialization_mode {
        Some(
            HeavyBlobFixturePlan::canonical_for_profile(input.size_class, input.topology)
                .unwrap_or_else(|| HeavyBlobFixturePlan::temp_file_smoke_for_topology(input.topology))
                .with_materialization_mode(mode),
        )
    } else {
        HeavyBlobFixturePlan::canonical_for_profile(input.size_class, input.topology)
    }?;
    if let Some(profile) = input.heavy_byte_pattern_profile {
        plan = plan.with_byte_pattern_profile(profile);
    }
    if let Some(backend_profile) = input.heavy_backend_profile {
        plan = plan.with_backend_profile(backend_profile);
    }
    Some(plan)
}

fn request_identity(input: &BlobHarnessExecutionInput) -> String {
    format!(
        "s7-blob-harness.{:?}.{:?}.{:?}.{:?}.{:?}.{:?}.{:?}.{}",
        input.profile,
        input.size_class,
        input.placement_class,
        input.security_scope_class,
        input.access_mode,
        input.failure_point,
        input.actor_mix,
        input.topology.chunk_count()
    )
}

const fn observed_yieldpoint(failure_point: BlobHarnessFailurePoint) -> BlobHarnessObservedYieldpoint {
    match failure_point {
        BlobHarnessFailurePoint::NoFaultSeed => BlobHarnessObservedYieldpoint::MemoryPressureBoundary,
        BlobHarnessFailurePoint::AfterChunkWrite => BlobHarnessObservedYieldpoint::WalAppendBeforeFlush,
        BlobHarnessFailurePoint::AfterSessionCheckpoint => BlobHarnessObservedYieldpoint::FreshRuntimeReplayOpen,
        BlobHarnessFailurePoint::AfterRootPublication => BlobHarnessObservedYieldpoint::RootPublicationBeforeObserve,
        BlobHarnessFailurePoint::DuringTierMove => BlobHarnessObservedYieldpoint::IoPressureBoundary,
        BlobHarnessFailurePoint::DuringExport => BlobHarnessObservedYieldpoint::OfflineVerifierLayoutWalkBeforeRuntimeRecovery,
        BlobHarnessFailurePoint::DuringReclaim => BlobHarnessObservedYieldpoint::ShortcutRejectionBoundary,
    }
}

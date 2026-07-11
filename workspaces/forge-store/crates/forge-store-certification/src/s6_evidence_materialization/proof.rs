use forge_proof::{
    prelude::{
        recipe, AuthorityMarker, AuthorityWitness, BasisPostureDxExt, BasisPostureKind,
        CapabilityMarker, CapabilityWitness, CheckedExecutionReadyRecipeDxExt,
        CheckedLoweredRecipeDxExt, CheckedResolvedRecipeDxExt, CheckedUnresolvedRecipeDxExt,
        LoweredBridgedRecipeDxExt, ProofOutcomeKind, RecipeStageDxExt, RecipeStageKind,
    },
    Artifact, PhaseMarker, TransitionOutcome,
};
use forge_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};
use forge_store_physical_certification::IoPressureEvidenceMaturity;
use forge_store_readiness::S6ReadinessCertificationProofTopology;

use super::S6CertificationEvidenceSources;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S6CertificationProjectionPhase;
impl PhaseMarker for S6CertificationProjectionPhase {}

pub type S6ProofProjectionArtifact =
    Artifact<S6CertificationProjectionPhase, S6CertificationProofProgression>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6CertificationProofProgression {
    execution_identity_tag: u64,
    lane_binding_mask: u16,
    backend_profile: BackendTargetProfile,
    backend_evidence_class: CapabilityEvidenceClass,
    harness_maturity: IoPressureEvidenceMaturity,
    queue_replay_rows: usize,
    flush_rows: usize,
    qualification_rows: usize,
    access_policy_rows: usize,
    secure_io_scope_checks: u64,
    post_admission_violation_rows: usize,
    readmission_boundaries: usize,
    checked_store_progression: S6CheckedStoreProofProgression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6CertificationProofTrace {
    projection: S6ProofProjectionArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct S6CheckedStoreProofProgression {
    resolution_outcome: ProofOutcomeKind,
    lowering_outcome: ProofOutcomeKind,
    readiness_outcome: ProofOutcomeKind,
    execution_outcome: ProofOutcomeKind,
    resolved_basis_posture: BasisPostureKind,
    lowered_basis_posture: BasisPostureKind,
    readmitted_basis_posture: BasisPostureKind,
    ready_stage: RecipeStageKind,
    executed_stage: RecipeStageKind,
    executed_basis_posture: BasisPostureKind,
    resolved_execution_identity_tag: u64,
    lowered_lane_binding_mask: u16,
    readiness_readmission_boundaries: usize,
    executed_readmission_boundaries: usize,
    freshness_readmitted_boundaries: usize,
}

impl S6CertificationProofTrace {
    pub(crate) fn from_sources(sources: &S6CertificationEvidenceSources) -> Self {
        let executed_projection = checked_projection_payload_from_sources(sources);
        Self {
            projection: Artifact::new(executed_projection),
        }
    }

    pub const fn projection(&self) -> &S6ProofProjectionArtifact {
        &self.projection
    }

    pub fn is_checked_from_executed_store_law(&self) -> bool {
        let payload = self.projection.payload();
        payload.execution_identity_tag > 0
            && payload.lane_binding_mask.count_ones() == 10
            && payload.queue_replay_rows > 0
            && payload.flush_rows > 0
            && payload.qualification_rows > 0
            && payload.access_policy_rows > 0
            && payload.secure_io_scope_checks > 0
            && payload.post_admission_violation_rows > 0
            && payload.readmission_boundaries == 0
            && payload.checked_store_progression.is_checked_for(payload)
    }
}

impl S6CertificationProofProgression {
    pub const fn execution_identity_tag(&self) -> u64 {
        self.execution_identity_tag
    }

    pub const fn lane_binding_mask(&self) -> u16 {
        self.lane_binding_mask
    }

    pub const fn backend_profile(&self) -> BackendTargetProfile {
        self.backend_profile
    }

    pub const fn backend_evidence_class(&self) -> CapabilityEvidenceClass {
        self.backend_evidence_class
    }

    pub const fn harness_maturity(&self) -> IoPressureEvidenceMaturity {
        self.harness_maturity
    }

    pub const fn access_policy_rows(&self) -> usize {
        self.access_policy_rows
    }

    pub const fn post_admission_violation_rows(&self) -> usize {
        self.post_admission_violation_rows
    }

    pub const fn readmission_boundaries(&self) -> usize {
        self.readmission_boundaries
    }

    pub fn checked_execution(&self) -> bool {
        self.checked_store_progression.is_checked_for(self)
    }

    pub fn readiness_proof_topology(&self) -> S6ReadinessCertificationProofTopology {
        self.checked_store_progression
            .readiness_proof_topology(self)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct S6CertificationProjectionResolutionAuthority;
impl AuthorityMarker for S6CertificationProjectionResolutionAuthority {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct S6CertificationProjectionLoweringCapability;
impl CapabilityMarker for S6CertificationProjectionLoweringCapability {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct S6CertificationProjectionReadinessAuthority;
impl AuthorityMarker for S6CertificationProjectionReadinessAuthority {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct S6CertificationProjectionReadmissionAuthority;
impl AuthorityMarker for S6CertificationProjectionReadmissionAuthority {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct S6CertificationProjectionBasis {
    execution_identity_tag: u64,
    lane_binding_mask: u16,
    readmission_boundaries: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct S6CertificationProjectionRuntime {
    readmission_boundaries: usize,
}

fn checked_projection_payload_from_sources(
    sources: &S6CertificationEvidenceSources,
) -> S6CertificationProofProgression {
    let unchecked_projection = S6CertificationProofProgression {
        execution_identity_tag: sources.binding().execution_identity_tag(),
        lane_binding_mask: sources.binding().required_lane_mask(),
        backend_profile: sources.backend_admission().profile(),
        backend_evidence_class: sources.backend_admission().evidence_class(),
        harness_maturity: sources.harness_closeout().evidence_maturity(),
        queue_replay_rows: sources
            .queue_execution()
            .counter_backed_receipt()
            .counter_rows()
            .len(),
        flush_rows: sources.flush_durability().len(),
        qualification_rows: sources.qualification_matrix().row_count(),
        access_policy_rows: sources.access_policy_rows().len(),
        secure_io_scope_checks: sources.secure_io_preservation().counters().scope_checks(),
        post_admission_violation_rows: sources.post_admission_violations().len(),
        readmission_boundaries: 0,
        checked_store_progression: S6CheckedStoreProofProgression::unchecked(),
    };
    let basis = S6CertificationProjectionBasis {
        execution_identity_tag: unchecked_projection.execution_identity_tag,
        lane_binding_mask: unchecked_projection.lane_binding_mask,
        readmission_boundaries: unchecked_projection.readmission_boundaries,
    };
    let runtime = S6CertificationProjectionRuntime {
        readmission_boundaries: unchecked_projection.readmission_boundaries,
    };
    let resolved_outcome = recipe(unchecked_projection).try_resolve_ready(
        basis,
        AuthorityWitness::from_authority_marker(S6CertificationProjectionResolutionAuthority),
    );
    let resolution_outcome = resolved_outcome.kind();
    let resolved = match resolved_outcome.into_raw() {
        TransitionOutcome::Success(resolved) => resolved,
        _ => unreachable!("ready resolution cannot produce a non-success outcome"),
    };
    let resolved_basis_posture = resolved.basis_posture();
    let lowered_outcome = resolved.try_lower_ready(CapabilityWitness::from_capability_marker(
        S6CertificationProjectionLoweringCapability,
    ));
    let lowering_outcome = lowered_outcome.kind();
    let lowered = match lowered_outcome.into_raw() {
        TransitionOutcome::Success(lowered) => lowered,
        _ => unreachable!("ready lowering cannot produce a non-success outcome"),
    };
    let lowered_basis_posture = lowered.basis_posture();
    let readmitted = lowered.bridge_trust_boundary().readmit_with(
        AuthorityWitness::from_authority_marker(S6CertificationProjectionReadmissionAuthority),
        basis,
    );
    let readmitted_basis_posture = readmitted.basis_posture();
    let ready_outcome = readmitted.try_ready_now(
        runtime,
        AuthorityWitness::from_authority_marker(S6CertificationProjectionReadinessAuthority),
    );
    let readiness_outcome = ready_outcome.kind();
    let ready = match ready_outcome.into_raw() {
        TransitionOutcome::Success(ready) => ready,
        _ => unreachable!("ready admission cannot produce a non-success outcome"),
    };
    let ready_stage = ready.stage();
    let executed_outcome = ready.try_execute();
    let execution_outcome = executed_outcome.kind();
    let executed = match executed_outcome.into_raw() {
        TransitionOutcome::Success(executed) => executed,
        _ => unreachable!("ready execution cannot produce a non-success outcome"),
    };
    let executed_stage = executed.stage();
    let executed_basis_posture = executed.basis_posture();
    let mut checked = executed.payload().clone();
    checked.checked_store_progression = S6CheckedStoreProofProgression::from_executed_recipe(
        S6CheckedForgeProofOutcomes {
            resolution_outcome,
            lowering_outcome,
            readiness_outcome,
            execution_outcome,
            resolved_basis_posture,
            lowered_basis_posture,
            readmitted_basis_posture,
            ready_stage,
            executed_stage,
            executed_basis_posture,
        },
        *executed.strong_basis().value(),
        runtime,
        executed.payload().readmission_boundaries,
    );
    debug_assert!(checked.checked_store_progression.is_checked_for(&checked));
    checked
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct S6CheckedForgeProofOutcomes {
    resolution_outcome: ProofOutcomeKind,
    lowering_outcome: ProofOutcomeKind,
    readiness_outcome: ProofOutcomeKind,
    execution_outcome: ProofOutcomeKind,
    resolved_basis_posture: BasisPostureKind,
    lowered_basis_posture: BasisPostureKind,
    readmitted_basis_posture: BasisPostureKind,
    ready_stage: RecipeStageKind,
    executed_stage: RecipeStageKind,
    executed_basis_posture: BasisPostureKind,
}

impl S6CheckedStoreProofProgression {
    const fn unchecked() -> Self {
        Self {
            resolution_outcome: ProofOutcomeKind::Failed,
            lowering_outcome: ProofOutcomeKind::Failed,
            readiness_outcome: ProofOutcomeKind::Failed,
            execution_outcome: ProofOutcomeKind::Failed,
            resolved_basis_posture: BasisPostureKind::None,
            lowered_basis_posture: BasisPostureKind::None,
            readmitted_basis_posture: BasisPostureKind::None,
            ready_stage: RecipeStageKind::Unresolved,
            executed_stage: RecipeStageKind::Unresolved,
            executed_basis_posture: BasisPostureKind::None,
            resolved_execution_identity_tag: 0,
            lowered_lane_binding_mask: 0,
            readiness_readmission_boundaries: 0,
            executed_readmission_boundaries: 0,
            freshness_readmitted_boundaries: 0,
        }
    }

    const fn from_executed_recipe(
        outcomes: S6CheckedForgeProofOutcomes,
        basis: S6CertificationProjectionBasis,
        runtime: S6CertificationProjectionRuntime,
        executed_readmission_boundaries: usize,
    ) -> Self {
        Self {
            resolution_outcome: outcomes.resolution_outcome,
            lowering_outcome: outcomes.lowering_outcome,
            readiness_outcome: outcomes.readiness_outcome,
            execution_outcome: outcomes.execution_outcome,
            resolved_basis_posture: outcomes.resolved_basis_posture,
            lowered_basis_posture: outcomes.lowered_basis_posture,
            readmitted_basis_posture: outcomes.readmitted_basis_posture,
            ready_stage: outcomes.ready_stage,
            executed_stage: outcomes.executed_stage,
            executed_basis_posture: outcomes.executed_basis_posture,
            resolved_execution_identity_tag: basis.execution_identity_tag,
            lowered_lane_binding_mask: basis.lane_binding_mask,
            readiness_readmission_boundaries: runtime.readmission_boundaries,
            executed_readmission_boundaries,
            freshness_readmitted_boundaries: basis.readmission_boundaries,
        }
    }

    fn is_checked_for(self, payload: &S6CertificationProofProgression) -> bool {
        self.resolution_outcome == ProofOutcomeKind::Success
            && self.lowering_outcome == ProofOutcomeKind::Success
            && self.readiness_outcome == ProofOutcomeKind::Success
            && self.execution_outcome == ProofOutcomeKind::Success
            && self.resolved_basis_posture == BasisPostureKind::CurrentValidity
            && self.lowered_basis_posture == BasisPostureKind::CurrentValidity
            && self.readmitted_basis_posture == BasisPostureKind::CurrentValidity
            && self.ready_stage == RecipeStageKind::ExecutionReady
            && self.executed_stage == RecipeStageKind::Executed
            && self.executed_basis_posture == BasisPostureKind::CurrentValidity
            && self.resolved_execution_identity_tag == payload.execution_identity_tag
            && self.lowered_lane_binding_mask == payload.lane_binding_mask
            && self.readiness_readmission_boundaries == payload.readmission_boundaries
            && self.executed_readmission_boundaries == payload.readmission_boundaries
            && self.freshness_readmitted_boundaries == payload.readmission_boundaries
            && self.freshness_readmitted_boundaries == payload.readmission_boundaries
    }

    fn readiness_proof_topology(
        self,
        payload: &S6CertificationProofProgression,
    ) -> S6ReadinessCertificationProofTopology {
        S6ReadinessCertificationProofTopology::new(
            self.resolution_outcome == ProofOutcomeKind::Success,
            self.lowering_outcome == ProofOutcomeKind::Success,
            self.readiness_outcome == ProofOutcomeKind::Success,
            self.execution_outcome == ProofOutcomeKind::Success,
            self.resolved_basis_posture == BasisPostureKind::CurrentValidity,
            self.lowered_basis_posture == BasisPostureKind::CurrentValidity,
            self.readmitted_basis_posture == BasisPostureKind::CurrentValidity,
            self.ready_stage == RecipeStageKind::ExecutionReady,
            self.executed_stage == RecipeStageKind::Executed,
            self.executed_basis_posture == BasisPostureKind::CurrentValidity,
            self.resolved_execution_identity_tag == payload.execution_identity_tag,
            self.lowered_lane_binding_mask == payload.lane_binding_mask,
            self.readiness_readmission_boundaries,
            self.executed_readmission_boundaries,
            self.freshness_readmitted_boundaries,
        )
    }
}

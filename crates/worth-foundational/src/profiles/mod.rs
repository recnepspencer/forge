mod attachments;
mod certification;
mod composition;
mod difference;
mod families;
mod front_doors;
mod identity;
mod materialization;
mod progression;
mod progression_resolution;
mod readiness;
mod resolution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileProductionTestReady;
impl worth_proof::PhaseMarker for FoundationalProfileProductionTestReady {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileProductionReadinessCertified;
impl worth_proof::ProofMarker for FoundationalProfileProductionReadinessCertified {}

pub use attachments::{
    attach_boundary_profiled_artifact, attach_proof_bearing_profiled_artifact,
    attach_support_profiled_artifact, BoundaryArtifactTarget, BoundaryProfiledArtifact,
    FoundationalProfileAttachmentDenial, FoundationalProfileAttachmentOutcome,
    FoundationalProfileAttachmentTargetKind, FoundationalProfileAttachmentTargetMarker,
    FoundationalProfiledArtifact, ProofBearingArtifactTarget, ProofBearingProfiledArtifact,
    SupportArtifactTarget, SupportProfiledArtifact,
};
pub use certification::{
    bridge_evidence_backed_proof_bearing_artifact_trust_boundary,
    bridge_production_certified_proof_bearing_artifact_trust_boundary,
    certify_evidence_backed_proof_bearing_artifact,
    certify_production_certified_proof_bearing_artifact,
    foundational_profile_certification_authority, foundational_profile_certification_proof_lane,
    foundational_profile_certification_readmission_authority,
    readmit_evidence_backed_proof_bearing_artifact_after_boundary,
    readmit_production_certified_proof_bearing_artifact_after_boundary,
    BoundaryBridgedEvidenceBackedCertifiedProofBearingArtifact,
    BoundaryBridgedProductionCertifiedProofBearingArtifact,
    EvidenceBackedCertifiedProofBearingArtifact, FoundationalProfileCertificationAuthority,
    FoundationalProfileCertificationDenial, FoundationalProfileCertificationOutcome,
    FoundationalProfileCertificationProofLane,
    FoundationalProfileCertificationReadmissionAuthority, ProductionCertifiedProofBearingArtifact,
};
pub use composition::{
    FoundationalProfileCompositionDenial, FoundationalProfileSet, FoundationalProfileSetInput,
};
pub use difference::{
    compare_foundational_profiles, FoundationalProfileCompatibilityClass,
    FoundationalProfileDifferenceReport,
};
pub use families::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, ExecutionObjectiveProfile, ObservationActivationProfile,
    RetentionDeliveryProfile, SupportPostureProfile,
};
pub use front_doors::{
    profiles, FoundationalProfileAttachmentFrontDoor, FoundationalProfileCertificationFrontDoor,
    FoundationalProfileFrontDoorConstructionDenial, FoundationalProfileFrontDoorFamily,
    FoundationalProfileMaterializationFrontDoor, FoundationalProfileProgressionFrontDoor,
    FoundationalProfileSetFrontDoor, MaterializedBoundaryArtifactStep,
    MaterializedProofBearingArtifactStep, MaterializedSupportArtifactStep, ProfilesFrontDoor,
};
pub use identity::{
    classify_foundational_profile_compatibility, compare_foundational_profile_identities,
    derive_foundational_profile_identity, foundational_profile_canonical_basis_entries,
    prepare_admitted_foundational_profile_for_canonical_basis, FoundationalProfileIdentity,
    FoundationalProfileIdentityDenial,
};
pub use materialization::{
    boundary_artifact_surface_inventory, foundational_profile_applicability,
    plan_foundational_profile_materialization,
    plan_foundational_profile_materialization_with_elision,
    plan_selected_foundational_profile_materialization,
    plan_selected_foundational_profile_materialization_with_disposition,
    proof_bearing_artifact_surface_inventory, support_artifact_surface_inventory,
    FoundationalDescriptiveElisionProfile, FoundationalDescriptiveSurface,
    FoundationalMaterializationCost, FoundationalMaterializationPlanningDenial,
    FoundationalObservationActivationScope, FoundationalObservationDisposition,
    FoundationalProfileApplicability, FoundationalProfileDecisionKind, FoundationalProfileFamily,
    FoundationalProfileMaterializationPlan, FoundationalSurfaceAbsenceCause,
    FoundationalSurfaceAvailabilityDecision, FoundationalTargetSurfaceInventory,
};
pub use progression::{
    admit_requested_foundational_profile, foundational_profile_progression_authority,
    materialize_admitted_foundational_profile, request_foundational_profile_set,
    AdmittedFoundationalProfileArtifact, AdmittedFoundationalProfilePhase,
    AdmittedFoundationalProfileSet, FoundationalProfileNarrowingKind,
    FoundationalProfileNarrowingRecord, FoundationalProfileProgressionAuthority,
    FoundationalProfileProgressionDeferred, FoundationalProfileProgressionDenial,
    FoundationalProfileProgressionFailure, FoundationalProfileProgressionOutcome,
    FoundationalProfileProgressionRebindRequired, FoundationalProfileProgressionStale,
    MaterializedFoundationalProfileArtifact, MaterializedFoundationalProfilePhase,
    MaterializedFoundationalProfileSet, RequestedFoundationalProfileArtifact,
    RequestedFoundationalProfilePhase, RequestedFoundationalProfileSet,
};
pub use progression_resolution::{
    admit_requested_foundational_profile_with_resolutions,
    materialize_admitted_foundational_profile_with_resolutions,
};
pub use readiness::{
    certify_foundational_profile_milestone3_production_test_readiness,
    foundational_profile_milestone10_readiness_report,
    foundational_profile_milestone3_readiness_report,
    require_foundational_profile_milestone3_production_test_readiness,
    FoundationalProfileCertifiedSurface, FoundationalProfileCertifiedSurfaceEvidence,
    FoundationalProfileCompileFailBoundary, FoundationalProfileMilestone10PhaseGate,
    FoundationalProfileMilestone10ReadinessReport, FoundationalProfileMilestone3PhaseGate,
    FoundationalProfilePhaseGateEvidence, FoundationalProfileProductionReadinessAuthority,
    FoundationalProfileProductionReadinessReport, FoundationalProfileProductionReadinessScope,
    FoundationalProfileProductionTestReadyArtifact, FoundationalProfileResidualDebt,
    FoundationalProfileRuntimeAssumption, FoundationalProfileRuntimeNonAssumption,
    FoundationalProfileSyntheticRuntimePressure, FoundationalProfileWORTHProofApi,
    FoundationalProfileWORTHProofForbiddenSurface, FoundationalProfileWORTHProofSurface,
};
pub use resolution::{
    FoundationalProfileResolutionFamily, FoundationalProfileResolutionLedger,
    FoundationalProfileResolutionLedgerDenial, FoundationalProfileResolutionRecord,
    FoundationalProfileResolutionRelation,
};

use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "profiles",
        "typed profile families, sealed composed profile meaning, profile progression, profile identity, descriptive materialization planning, proof-bearing certification strengthening, and milestone readiness reporting",
        "runtime policy execution",
    )
}

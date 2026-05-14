mod authority;
mod basis;
mod categories;
mod current_basis;
mod materialization;
mod planned;
mod readiness;
mod reserved_authority_transition;
mod roles;
mod same_family;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryArtifactProductionTestReady;
impl forge_proof::PhaseMarker for FoundationalBoundaryArtifactProductionTestReady {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryArtifactProductionReadinessCertified;
impl forge_proof::ProofMarker for FoundationalBoundaryArtifactProductionReadinessCertified {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryAuthorityAdmitted;
impl forge_proof::ProofMarker for FoundationalBoundaryAuthorityAdmitted {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryCurrentBasisCertified;
impl forge_proof::ProofMarker for FoundationalBoundaryCurrentBasisCertified {}

pub use authority::{
    admit_authoritative_current_boundary_surface, foundational_boundary_authority_admission,
    FoundationalAuthoritativeBoundaryClaim, FoundationalBoundaryAuthorityAdmission,
    FoundationalBoundaryAuthorityAdmissionBasis,
};
pub use basis::{
    foundational_boundary_canonical_basis_entries,
    prepare_materialized_boundary_artifact_for_canonical_basis,
    prepare_materialized_boundary_bundle_for_canonical_basis,
};
pub use categories::{
    boundary_artifact_category_definitions, boundary_artifact_category_of,
    boundary_artifact_surface_definition, boundary_receipt_category_of,
    boundary_receipt_definition, boundary_report_category_of, boundary_report_definition,
    boundary_summary_category_of, boundary_summary_definition, ArtifactCategory,
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactSurface,
    FoundationalBoundaryCategoryConstructionDenial, FoundationalBoundaryCategoryDefinition,
    FoundationalBoundaryCategoryMarker, FoundationalBoundaryCategorySurface,
    FoundationalBoundaryReceiptSurface, FoundationalBoundaryReportSurface,
    FoundationalBoundarySummarySurface, ReceiptCategory, ReportCategory, SummaryCategory,
};
pub use current_basis::{
    admit_current_basis_boundary_artifact, admit_current_basis_boundary_bundle,
    bridge_current_basis_boundary_artifact_trust_boundary,
    bridge_current_basis_boundary_bundle_trust_boundary,
    foundational_boundary_current_basis_authority, foundational_boundary_current_basis_proof_lane,
    foundational_boundary_current_basis_readmission_authority,
    readmit_current_basis_boundary_artifact_after_boundary,
    readmit_current_basis_boundary_bundle_after_boundary,
    BoundaryBridgedCurrentBasisBoundaryArtifact, BoundaryBridgedCurrentBasisBoundaryBundle,
    CurrentBasisBoundaryArtifact, CurrentBasisBoundaryArtifactPhase, CurrentBasisBoundaryBundle,
    FoundationalBoundaryCurrentBasisAuthority, FoundationalBoundaryCurrentBasisProofLane,
    FoundationalBoundaryCurrentBasisReadmissionAuthority,
};
pub use materialization::{
    evaluate_boundary_surface_disposition_legality, materialize_authoritative_boundary_surface,
    materialize_descriptive_boundary_surface, plan_artifact_boundary_bundle,
    plan_authoritative_boundary_materialization, plan_descriptive_boundary_materialization,
    FoundationalBoundaryAttachmentPoint, FoundationalBoundaryAvailability,
    FoundationalBoundaryBundleMaterializationCost, FoundationalBoundaryBundleMaterializationDenial,
    FoundationalBoundaryBundlePlanningDenial, FoundationalBoundaryDecisionCause,
    FoundationalBoundaryDecisionSubject, FoundationalBoundaryDeliveryClass,
    FoundationalBoundaryMaterializationAttachment, FoundationalBoundaryMaterializationBundle,
    FoundationalBoundaryMaterializationBundlePlan, FoundationalBoundaryMaterializationCost,
    FoundationalBoundaryMaterializationDecisionRow, FoundationalBoundaryMaterializationDenial,
    FoundationalBoundaryMaterializationInput, FoundationalBoundaryMaterializationPlan,
    FoundationalBoundaryMaterializationSeam, FoundationalBoundaryMaterializationSource,
    FoundationalBoundaryPlanningDenial, FoundationalBoundarySurfaceDisposition,
    FoundationalBoundarySurfaceDispositionDenial, FoundationalBoundarySurfaceDispositionLegality,
    FoundationalMaterializedBoundaryArtifact,
};
pub use planned::{
    admit_planned_work_boundary_artifact, FoundationalPlannedWorkBoundaryArtifact,
    FoundationalPlannedWorkBoundaryArtifactDenial,
};
pub use readiness::{
    certify_foundational_boundary_artifact_milestone4_production_test_readiness,
    foundational_boundary_artifact_milestone4_readiness_report,
    require_foundational_boundary_artifact_milestone4_production_test_readiness,
    FoundationalBoundaryArtifactCertifiedSurface,
    FoundationalBoundaryArtifactCertifiedSurfaceEvidence,
    FoundationalBoundaryArtifactCompileFailBoundary, FoundationalBoundaryArtifactForgeProofApi,
    FoundationalBoundaryArtifactForgeProofForbiddenSurface,
    FoundationalBoundaryArtifactForgeProofSurface, FoundationalBoundaryArtifactMilestone4PhaseGate,
    FoundationalBoundaryArtifactPhaseGateEvidence,
    FoundationalBoundaryArtifactProductionReadinessAuthority,
    FoundationalBoundaryArtifactProductionReadinessReport,
    FoundationalBoundaryArtifactProductionReadinessScope,
    FoundationalBoundaryArtifactProductionTestReadyArtifact,
    FoundationalBoundaryArtifactResidualDebt, FoundationalBoundaryArtifactRuntimeAssumption,
    FoundationalBoundaryArtifactRuntimeNonAssumption,
    FoundationalBoundaryArtifactSyntheticRuntimePressure,
};
pub use reserved_authority_transition::{
    evaluate_planned_work_reserved_authority_transition_legality,
    evaluate_same_family_reserved_authority_transition_legality,
    FoundationalReservedAuthorityTransitionDenial, FoundationalReservedAuthorityTransitionKind,
};
pub use roles::{
    boundary_role_definitions, claim_derived_projection_boundary_surface,
    claim_planned_work_boundary_surface, claim_receipt_evidence_boundary_surface,
    claim_support_only_boundary_surface, evaluate_boundary_role_claim_legality,
    AuthoritativeCurrentRole, DerivedProjectionRole, FoundationalBoundaryArtifactRole,
    FoundationalBoundaryRoleClaim, FoundationalBoundaryRoleClaimDenial,
    FoundationalBoundaryRoleDefinition, FoundationalBoundaryRoleMarker,
    FoundationalDerivedProjectionBoundaryClaim, FoundationalPlannedWorkBoundaryClaim,
    FoundationalReceiptEvidenceBoundaryClaim, FoundationalSupportOnlyBoundaryClaim,
    PlannedWorkRole, ReceiptEvidenceRole, SupportOnlyRole,
};
pub use same_family::{
    admit_same_family_boundary_artifact, derive_same_family_boundary_identity,
    prepare_same_family_boundary_artifact_for_canonical_basis,
    FoundationalSameFamilyBoundaryArtifact, FoundationalSameFamilyBoundaryArtifactDenial,
    FoundationalSameFamilyBoundaryFamily, FoundationalSameFamilyBoundaryFamilyDenial,
    FoundationalSameFamilyBoundaryIdentity,
};

use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "boundary_artifacts",
        "typed boundary artifact category vocabulary, role law, authority admission, explicit materialization seams, coordinated bundle emission, canonical basis lowering, proof-bearing current-basis boundary surfaces, and descriptive extension law",
        "Milestone 5 authority-transition ontology",
    )
}

mod basis;
mod categories;
mod certified;
mod materialization;
mod outcomes;
mod primitives;
mod readiness;
mod rows;
mod subjects;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticProductionTestReady;
impl worth_proof::PhaseMarker for FoundationalDiagnosticProductionTestReady {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticProductionReadinessCertified;
impl worth_proof::ProofMarker for FoundationalDiagnosticProductionReadinessCertified {}

pub use basis::{
    compare_diagnostic_explanation_bundles, compare_diagnostic_support_reports,
    foundational_diagnostic_canonical_basis_entries,
    prepare_diagnostic_explanation_bundle_for_canonical_basis,
    prepare_diagnostic_support_report_for_canonical_basis, FoundationalDiagnosticComparisonBundle,
    FoundationalDiagnosticComparisonDenial,
};
pub use categories::{
    diagnostic_artifact_kind_definitions, diagnostic_comparison_bundle_definition,
    diagnostic_explanation_bundle_definition, diagnostic_failure_bundle_definition,
    diagnostic_report_definition, diagnostic_summary_definition,
    diagnostic_support_report_definition, evaluate_diagnostic_materialization_legality,
    foundational_comparison_bundle_artifact_kind, foundational_explanation_bundle_artifact_kind,
    foundational_failure_bundle_artifact_kind, foundational_report_artifact_kind,
    foundational_summary_artifact_kind, foundational_support_report_artifact_kind,
    FoundationalComparisonBundleArtifactKind, FoundationalDiagnosticArtifactKind,
    FoundationalDiagnosticArtifactKindDefinition, FoundationalDiagnosticArtifactKindMarker,
    FoundationalDiagnosticAvailability, FoundationalDiagnosticDeliveryClass,
    FoundationalDiagnosticMaterializationLegalityDenial, FoundationalExplanationBundleArtifactKind,
    FoundationalFailureBundleArtifactKind, FoundationalReportArtifactKind,
    FoundationalSummaryArtifactKind, FoundationalSupportReportArtifactKind,
};
pub use certified::{
    bridge_certified_diagnostic_bundle_trust_boundary, certify_current_basis_diagnostic_bundle,
    certify_diagnostic_bundle_with_source_basis,
    foundational_diagnostic_certified_attachment_authority,
    foundational_diagnostic_certified_readmission_authority,
    readmit_certified_diagnostic_bundle_after_boundary, BoundaryBridgedCertifiedDiagnosticBundle,
    FoundationalCertifiedDiagnosticBundle, FoundationalCertifiedDiagnosticPayload,
    FoundationalCertifiedDiagnosticProvenanceHook, FoundationalCertifiedDiagnosticSource,
    FoundationalCertifiedDiagnosticSourceKind, FoundationalDiagnosticCertified,
    FoundationalDiagnosticCertifiedAttachmentAuthority,
    FoundationalDiagnosticCertifiedAttachmentDenial, FoundationalDiagnosticCertifiedCoverageClass,
    FoundationalDiagnosticCertifiedCoverageDenial, FoundationalDiagnosticCertifiedPhase,
    FoundationalDiagnosticCertifiedReadmissionAuthority,
    FoundationalDiagnosticCoverageFamilyStatus, FoundationalDiagnosticCoverageMatrix,
};
pub use materialization::{
    materialize_diagnostic_explanation_bundle, materialize_diagnostic_support_report,
    plan_diagnostic_explanation_bundle, plan_diagnostic_support_report,
    FoundationalDiagnosticAssemblyDebt, FoundationalDiagnosticAssemblyDebtClass,
    FoundationalDiagnosticCounterSnapshot, FoundationalDiagnosticExplanationBundle,
    FoundationalDiagnosticExplanationInput, FoundationalDiagnosticGapClass,
    FoundationalDiagnosticGapClosurePosture, FoundationalDiagnosticGapTarget,
    FoundationalDiagnosticMaterializationDenial, FoundationalDiagnosticMaterializationPlan,
    FoundationalDiagnosticNamedGap, FoundationalDiagnosticPartiality,
    FoundationalDiagnosticSupportClaimStrength, FoundationalDiagnosticSupportInput,
    FoundationalDiagnosticSupportReport, FoundationalDiagnosticSurfaceAvailability,
};
pub use outcomes::{FoundationalDiagnosticAbsenceCause, FoundationalDiagnosticOutcomeKind};
pub use primitives::{
    foundational_diagnostic_code, foundational_diagnostic_scope, FoundationalDiagnosticBreachClass,
    FoundationalDiagnosticCodeId, FoundationalDiagnosticDenialClass,
    FoundationalDiagnosticEvidencePosture, FoundationalDiagnosticPrimitiveConstructionDenial,
    FoundationalDiagnosticScopeId, FoundationalDiagnosticSeverity,
};
pub use readiness::{
    certify_foundational_diagnostic_milestone6_production_test_readiness,
    foundational_diagnostic_milestone6_readiness_report,
    require_foundational_diagnostic_milestone6_production_test_readiness,
    FoundationalDiagnosticAdoptionShapedFollowthrough,
    FoundationalDiagnosticCanonicalGoldenArtifact,
    FoundationalDiagnosticCanonicalGoldenArtifactEvidence, FoundationalDiagnosticCertifiedSurface,
    FoundationalDiagnosticCertifiedSurfaceEvidence, FoundationalDiagnosticCompileFailBoundary,
    FoundationalDiagnosticCompileFailEvidence, FoundationalDiagnosticHarnessExpansionEvidence,
    FoundationalDiagnosticHarnessExpansionPoint, FoundationalDiagnosticMilestone6PhaseGate,
    FoundationalDiagnosticPhaseGateEvidence, FoundationalDiagnosticProductionReadinessAuthority,
    FoundationalDiagnosticProductionReadinessReport,
    FoundationalDiagnosticProductionReadinessScope,
    FoundationalDiagnosticProductionTestReadyArtifact, FoundationalDiagnosticPropertySeed,
    FoundationalDiagnosticPropertySeedEvidence, FoundationalDiagnosticResidualDebt,
    FoundationalDiagnosticRuntimeAdoptionFailurePressure, FoundationalDiagnosticRuntimeAssumption,
    FoundationalDiagnosticRuntimeNonAssumption, FoundationalDiagnosticSyntheticPressureEvidence,
    FoundationalDiagnosticSyntheticRuntimePressure, FoundationalDiagnosticWORTHProofApi,
    FoundationalDiagnosticWORTHProofApiEvidence, FoundationalDiagnosticWORTHProofForbiddenSurface,
    FoundationalDiagnosticWORTHProofSurface,
};
pub use rows::{
    sort_foundational_diagnostic_rows, FoundationalDiagnosticComparisonRow,
    FoundationalDiagnosticDecisionRow, FoundationalDiagnosticFailureRow,
    FoundationalDiagnosticLocalityClaim, FoundationalDiagnosticProvenanceReadyRow,
    FoundationalDiagnosticRow, FoundationalDiagnosticRowFamily,
    FoundationalDiagnosticSemanticLabelSet, FoundationalDiagnosticSupportEvidencePosture,
    FoundationalDiagnosticSupportRow, FoundationalDiagnosticWidenedFalloutPosture,
};
pub use subjects::{
    foundational_diagnostic_boundary_artifact_subject,
    foundational_diagnostic_branch_candidate_subject,
    foundational_diagnostic_branch_discard_subject, foundational_diagnostic_commit_receipt_subject,
    foundational_diagnostic_committed_authority_subject,
    foundational_diagnostic_locator_boundary_artifact, foundational_diagnostic_locator_mismatch,
    foundational_diagnostic_locator_source, foundational_diagnostic_locator_transition,
    foundational_diagnostic_merge_verdict_subject, FoundationalDiagnosticLocator,
    FoundationalDiagnosticSubject,
};

use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "diagnostics",
        "diagnostic primitive vocabulary, artifact-kind category law, outcome and row topology, descriptive explanation/support planning and materialization boundaries, canonical basis/comparison bundle law, proof-bearing certified attachment compatibility, and diagnostics readiness closeout",
        "transition authority meaning, one generic diagnostics row bag, or one diagnostics runtime/store",
    )
}

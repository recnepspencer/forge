mod basis;
mod certified;
mod claims;
mod front_doors;
mod layouts;
mod legality;
mod policy;
mod primitives;
mod readiness;
mod receipts;
mod reports;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformanceProductionTestReady;
impl forge_proof::PhaseMarker for FoundationalPerformanceProductionTestReady {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformanceProductionReadinessCertified;
impl forge_proof::ProofMarker for FoundationalPerformanceProductionReadinessCertified {}

pub use basis::{
    compare_performance_bundles, foundational_performance_canonical_basis_entries,
    performance_basis_rule_version, performance_bundle,
    prepare_counter_backed_performance_receipt_for_canonical_basis,
    prepare_materialized_performance_report_for_canonical_basis,
    prepare_performance_bundle_for_canonical_basis,
    FoundationalPerformanceAttachmentConstructionDenial, FoundationalPerformanceBundle,
    FoundationalPerformanceBundleBuilder, FoundationalPerformanceBundleConstructionDenial,
    FoundationalPerformanceComparison, FoundationalPerformanceContractName,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceMismatch,
    FoundationalPerformanceSupportingEvidenceCode, FoundationalPerformanceSupportingEvidenceRow,
};
pub use certified::{
    bridge_certified_performance_bundle_trust_boundary,
    certify_hot_path_counter_backed_performance_receipt,
    certify_support_expansion_performance_report,
    foundational_performance_certified_attachment_authority,
    foundational_performance_certified_readmission_authority,
    readmit_certified_performance_bundle_after_boundary, BoundaryBridgedCertifiedPerformanceBundle,
    FoundationalCertifiedPerformanceAttachmentDenial, FoundationalCertifiedPerformanceBundle,
    FoundationalCertifiedPerformanceClass, FoundationalCertifiedPerformancePayload,
    FoundationalCertifiedPerformanceSource, FoundationalCertifiedPerformanceSourceDigest,
    FoundationalCertifiedPerformanceSourceKind, FoundationalPerformanceCertified,
    FoundationalPerformanceCertifiedAttachmentAuthority, FoundationalPerformanceCertifiedPhase,
    FoundationalPerformanceCertifiedReadmissionAuthority,
};
pub use claims::{
    FoundationalAuthoritativePerformanceClaim, FoundationalAuthoritativePerformanceClaimBuilder,
    FoundationalPerformanceClaimAuthoringFrontDoor, FoundationalPerformanceClaimConstructionDenial,
    FoundationalPerformanceClaimSurface, FoundationalPolicyAdmissionPerformanceClaim,
    FoundationalPolicyAdmissionPerformanceClaimBuilder,
    FoundationalReplayMaterializationPerformanceClaim,
    FoundationalReplayMaterializationPerformanceClaimBuilder,
    FoundationalSupportDerivedPerformanceClaim, FoundationalSupportDerivedPerformanceClaimBuilder,
};
pub use front_doors::{performance, FoundationalPerformanceFrontDoor};
pub use layouts::{
    FoundationalLayoutAnnotatedClaim, FoundationalLayoutAnnotatedClaimConstructionDenial,
    FoundationalLayoutIntentClaim,
};
pub use legality::{
    evaluate_performance_primitive_legality, FoundationalPerformancePrimitiveLegalityDenial,
};
pub use policy::{
    foundational_performance_budget_definitions, policy_admission_receipt,
    FoundationalPerformanceBudgetDecision, FoundationalPerformanceBudgetDefinition,
    FoundationalPerformanceBudgetKind, FoundationalPolicyAdmissionReceipt,
    FoundationalPolicyAdmissionReceiptBuilder,
    FoundationalPolicyAdmissionReceiptConstructionDenial,
};
pub use primitives::{
    foundational_performance_access_pattern_definitions,
    foundational_performance_allocation_definitions, foundational_performance_boundary_definitions,
    foundational_performance_breadth_locality_definitions,
    foundational_performance_evidence_strength_definitions,
    foundational_performance_execution_temperature_definitions,
    foundational_performance_fallback_debt_definitions,
    foundational_performance_freshness_retention_definitions,
    foundational_performance_layout_intent_definitions,
    foundational_performance_work_class_definitions,
    FoundationalPerformanceAccessPatternDefinition, FoundationalPerformanceAccessPatternPosture,
    FoundationalPerformanceAllocationDefinition, FoundationalPerformanceAllocationPosture,
    FoundationalPerformanceBoundary, FoundationalPerformanceBoundaryDefinition,
    FoundationalPerformanceBreadthLocalityDefinition,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceEvidenceStrengthDefinition, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceExecutionTemperatureDefinition,
    FoundationalPerformanceFallbackDebtDefinition, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionDefinition,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceLayoutIntent,
    FoundationalPerformanceLayoutIntentDefinition, FoundationalPerformancePrimitiveDefinition,
    FoundationalPerformanceWorkClass, FoundationalPerformanceWorkClassDefinition,
};
pub use readiness::{
    certify_foundational_performance_milestone8_production_test_readiness,
    foundational_performance_milestone8_readiness_report,
    require_foundational_performance_milestone8_production_test_readiness,
    FoundationalPerformanceCertifiedSurface, FoundationalPerformanceCertifiedSurfaceEvidence,
    FoundationalPerformanceCompileFailBoundary, FoundationalPerformanceForgeProofApi,
    FoundationalPerformanceForgeProofForbiddenSurface, FoundationalPerformanceForgeProofSurface,
    FoundationalPerformanceHarnessExpansionPoint, FoundationalPerformanceMilestone8PhaseGate,
    FoundationalPerformancePhaseGateEvidence, FoundationalPerformanceProductionReadinessAuthority,
    FoundationalPerformanceProductionReadinessReport,
    FoundationalPerformanceProductionReadinessScope,
    FoundationalPerformanceProductionTestReadyArtifact,
    FoundationalPerformancePublicSurfaceDocumentationCoverage, FoundationalPerformanceResidualDebt,
    FoundationalPerformanceRuntimeAdoptionPressure,
    FoundationalPerformanceRuntimeAdoptionPressureEvidence,
    FoundationalPerformanceRuntimeAssumption, FoundationalPerformanceRuntimeNonAssumption,
    FoundationalPerformanceSyntheticRuntimePressure,
};
pub use receipts::{
    counter_backed_performance_receipt, FoundationalCounterBackedPerformanceReceipt,
    FoundationalCounterBackedPerformanceReceiptBuilder,
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
};
pub use reports::{
    attach_counter_backed_performance_receipt, attach_performance_bundle,
    attach_policy_admission_receipt, foundational_performance_attachment_target_kind_definitions,
    plan_performance_report, FoundationalAttachedCounterBackedPerformanceReceipt,
    FoundationalAttachedPerformanceBundle, FoundationalAttachedPolicyAdmissionReceipt,
    FoundationalMaterializedPerformanceReport, FoundationalPerformanceAttachmentDenial,
    FoundationalPerformanceAttachmentTargetKind,
    FoundationalPerformanceReportMaterializationBoundary, FoundationalPerformanceReportPlan,
    FoundationalPerformanceReportRequest, FoundationalPerformanceReportSection,
    FoundationalPerformanceReportSectionDecision,
    FoundationalPerformanceReportSectionDecisionCause,
};

use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "performance",
        "performance primitive families for layout intent, boundary, evidence strength, breadth/locality, allocation, access pattern, execution temperature, freshness/retention, fallback/debt, and work disclosure, the minimum legality floor that rejects obvious hot-path, support, replay, and debt-shape contradictions, common-path claim builders with mechanically distinct lowered claim families for authoritative execution, support-derived, replay/materialization, and policy-admission performance meaning, layout-intent disclosure surfaces that attach representation and allocation posture without changing claim meaning, explicit policy-admission receipts with budget and fallback disclosure that remain visibly pre-execution, lower-lane canonical bundle, comparison, and exact counter-backed receipt surfaces, typed attachment targets and explicit performance report request, planning, and materialization surfaces, plus stronger proof-bearing certified performance bundles and readmission over current-basis hot-path receipts and support-expansion reports",
        "one shared telemetry runtime or one shared hot-path container",
    )
}

mod attachment_front_doors;
mod attachments;
mod front_doors;
mod legality;
mod lineage;
mod lineage_front_doors;
mod primitives;
mod provenance;
mod provenance_front_doors;
mod readiness;
mod receipt_front_doors;
mod receipts;
mod support;
mod support_front_doors;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceProductionTestReady;
impl forge_proof::PhaseMarker for FoundationalBoundaryEvidenceProductionTestReady {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceProductionReadinessCertified;
impl forge_proof::ProofMarker for FoundationalBoundaryEvidenceProductionReadinessCertified {}

pub use attachment_front_doors::FoundationalBoundaryEvidenceAttachmentFrontDoor;
pub use attachments::{
    admit_current_basis_boundary_evidence_attachment_bundle,
    admit_support_basis_boundary_evidence_attachment_bundle,
    bridge_current_basis_boundary_evidence_attachment_bundle_trust_boundary,
    bridge_support_basis_boundary_evidence_attachment_bundle_trust_boundary,
    derive_boundary_evidence_attachment_bundle_digest,
    foundational_boundary_evidence_attachment_readmission_authority,
    foundational_boundary_evidence_attachment_target_kind_definitions,
    foundational_boundary_evidence_continuity_attachment_scope_definitions,
    foundational_boundary_evidence_materialization_profile_definitions,
    foundational_boundary_evidence_support_readmission_authority,
    prepare_boundary_evidence_attachment_bundle_for_canonical_basis,
    readmit_current_basis_boundary_evidence_attachment_bundle_after_boundary,
    readmit_support_basis_boundary_evidence_attachment_bundle_after_boundary,
    BoundaryBridgedCurrentBasisBoundaryEvidenceAttachmentBundle,
    BoundaryBridgedSupportBasisBoundaryEvidenceAttachmentBundle,
    CurrentBasisBoundaryEvidenceAttachmentBundle, FoundationalBoundaryEvidenceAttachmentBundle,
    FoundationalBoundaryEvidenceAttachmentDigestDerivationDenial,
    FoundationalBoundaryEvidenceAttachmentReadmissionAuthority,
    FoundationalBoundaryEvidenceAttachmentTarget, FoundationalBoundaryEvidenceAttachmentTargetKind,
    FoundationalBoundaryEvidenceContinuityAttachmentScope,
    FoundationalBoundaryEvidenceDiagnosticAttachment,
    FoundationalBoundaryEvidenceLocatorContinuityAttachment,
    FoundationalBoundaryEvidenceMaterializationProfile,
    FoundationalBoundaryEvidenceObjectContinuityAttachment,
    FoundationalBoundaryEvidenceSupportAttachment,
    FoundationalBoundaryEvidenceSupportReadmissionAuthority,
    FoundationalBoundaryEvidenceSupportReadmissionDenial,
    FoundationalDiagnosticBundleAttachmentBundle,
    FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
    SupportBasisBoundaryEvidenceAttachmentBundle,
};
pub use front_doors::{
    attachment, boundary_evidence, lineage, provenance, receipt, support, BoundaryEvidenceFrontDoor,
};
pub use legality::{
    evaluate_boundary_evidence_primitive_legality,
    FoundationalBoundaryEvidencePrimitiveLegalityDenial,
};
pub use lineage::{
    foundational_boundary_evidence_branch_divergence_posture_definitions,
    foundational_boundary_evidence_lineage_outcome_kind_definitions,
    foundational_boundary_evidence_lineage_partiality_posture_definitions,
    foundational_boundary_evidence_promotion_posture_definitions,
    FoundationalBoundaryEvidenceAttestedLineageArtifact,
    FoundationalBoundaryEvidenceBranchDivergencePosture,
    FoundationalBoundaryEvidenceBranchLocalLineageArtifact,
    FoundationalBoundaryEvidenceLineageConstructionDenial,
    FoundationalBoundaryEvidenceLineageOutcomeKind,
    FoundationalBoundaryEvidenceLineagePartialityPosture,
    FoundationalBoundaryEvidenceLineageSubject, FoundationalBoundaryEvidenceLineageSubjectSet,
    FoundationalBoundaryEvidencePartialLineageArtifact,
    FoundationalBoundaryEvidencePromotedLineageArtifact,
    FoundationalBoundaryEvidencePromotionPosture,
    FoundationalBoundaryEvidenceReconstructedEquivalenceArtifact,
    FoundationalBoundaryEvidenceReplayDerivedLineageArtifact,
    FoundationalBoundaryEvidenceRestoredLineageArtifact,
};
pub use lineage_front_doors::FoundationalBoundaryEvidenceLineageFrontDoor;
pub use primitives::{
    foundational_boundary_evidence_category_definitions,
    foundational_boundary_evidence_descriptive_role_definitions,
    foundational_boundary_evidence_execution_posture_definitions,
    foundational_boundary_evidence_freshness_posture_definitions,
    foundational_boundary_evidence_locality_definitions, FoundationalBoundaryEvidenceCategory,
    FoundationalBoundaryEvidenceDescriptiveRole, FoundationalBoundaryEvidenceExecutionPosture,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceLocality,
    FoundationalBoundaryEvidencePrimitiveDefinition,
};
pub use provenance::{
    foundational_boundary_evidence_provenance_layer_definitions,
    foundational_boundary_evidence_source_basis_kind_definitions,
    FoundationalBoundaryEvidenceAuthorityPath, FoundationalBoundaryEvidenceCanonicalDigestBasis,
    FoundationalBoundaryEvidenceComparisonBasis, FoundationalBoundaryEvidenceProfileBasis,
    FoundationalBoundaryEvidenceProvenanceArtifact,
    FoundationalBoundaryEvidenceProvenanceConstructionDenial,
    FoundationalBoundaryEvidenceProvenanceLayerKind, FoundationalBoundaryEvidenceSourceBasis,
    FoundationalBoundaryEvidenceSourceBasisKind, FoundationalBoundaryEvidenceStrategyBasis,
    FoundationalBoundaryEvidenceSupportContextAttachment,
};
pub use provenance_front_doors::FoundationalBoundaryEvidenceProvenanceFrontDoor;
pub use readiness::{
    certify_foundational_boundary_evidence_milestone7_production_test_readiness,
    foundational_boundary_evidence_milestone7_readiness_report,
    require_foundational_boundary_evidence_milestone7_production_test_readiness,
    FoundationalBoundaryEvidenceCertifiedSurface,
    FoundationalBoundaryEvidenceCertifiedSurfaceEvidence,
    FoundationalBoundaryEvidenceCompileFailBoundary, FoundationalBoundaryEvidenceGoldenArtifact,
    FoundationalBoundaryEvidenceHarnessExpansionPoint,
    FoundationalBoundaryEvidenceMilestone7PhaseGate, FoundationalBoundaryEvidencePhaseGateEvidence,
    FoundationalBoundaryEvidenceProductionReadinessAuthority,
    FoundationalBoundaryEvidenceProductionReadinessReport,
    FoundationalBoundaryEvidenceProductionReadinessScope,
    FoundationalBoundaryEvidenceProductionTestReadyArtifact,
    FoundationalBoundaryEvidencePropertySeed, FoundationalBoundaryEvidencePropertySeedEvidence,
    FoundationalBoundaryEvidenceResidualDebt, FoundationalBoundaryEvidenceRuntimeAssumption,
    FoundationalBoundaryEvidenceRuntimeNonAssumption,
    FoundationalBoundaryEvidenceSyntheticRuntimePressure,
};
pub use receipt_front_doors::FoundationalBoundaryEvidenceReceiptFrontDoor;
pub use receipts::{
    foundational_boundary_evidence_closeout_disposition_definitions,
    foundational_boundary_evidence_receipt_kind_definitions,
    FoundationalBoundaryEvidenceCloseoutDisposition,
    FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    FoundationalBoundaryEvidencePlanningReceiptArtifact,
    FoundationalBoundaryEvidenceReceiptBoundary, FoundationalBoundaryEvidenceReceiptKind,
};
pub use support::{
    foundational_boundary_evidence_support_basis_disclosure_definitions,
    foundational_boundary_evidence_support_recovery_posture_definitions,
    foundational_boundary_evidence_support_residual_debt_kind_definitions,
    foundational_boundary_evidence_support_truth_kind_definitions,
    FoundationalBoundaryEvidencePublishedSupportArtifact,
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalBoundaryEvidenceSupportCloseoutArtifact,
    FoundationalBoundaryEvidenceSupportConstructionDenial,
    FoundationalBoundaryEvidenceSupportRecoveryPosture,
    FoundationalBoundaryEvidenceSupportResidualDebtKind,
    FoundationalBoundaryEvidenceSupportResidualDebtSet,
    FoundationalBoundaryEvidenceSupportTruthKind,
    FoundationalBoundaryEvidenceTransientLifecycleSupportArtifact,
};
pub use support_front_doors::FoundationalBoundaryEvidenceSupportFrontDoor;

use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "boundary_evidence",
        "lineage, provenance, receipt, and support-truth primitive category law, locality posture, planned-versus-executed and descriptive-role primitives, the minimum legality floor connecting them, typed provenance layering with explicit freshness posture and family-distinct source-basis roots, family-distinct planning-versus-completed receipt artifacts with explicit blocked and denied closeout truth, lineage outcome families covering attested continuity, branch-local divergence and promotion, replay-derived continuity, restored continuity, reconstructed equivalence, and explicit partiality postures, plus support-truth families for support publication, degraded closeout, transient lifecycle evidence, basis disclosure, recovery posture, and residual debt, along with attachment bundles, locator-level versus object-level continuity, materialization elision posture, canonical basis lowering, and current-basis plus support-basis readmission for attached evidence bundles",
        "support bundles or one generic history/provenance envelope",
    )
}

//! Public API boundary for `worth-query`.
//! External crates should import through this module rather than reaching into
//! internal crate structure directly.

mod exports_application;
mod exports_foundation;
mod exports_policy;
mod exports_runtime;
mod exports_runtime_phase_nine;

pub mod identity_authority {
    pub use crate::identity_authority::*;
}

pub mod consumer_kit {
    pub use crate::runtime::WorthQueryRuntimeFacadeFamily;
    pub use crate::{
        compare_test_backend_write_receipts, downstream_authority_adoption,
        evidence_report_adoption_audit, graph_obligation_consumer_kit, graph_read_bypass_adoption,
        graph_read_bypass_audit, hard_prohibition_boundary_audit,
        hard_prohibition_boundary_audit_coverage, hard_prohibition_compile_fail_fixtures,
        hard_prohibition_documentation_rows, hard_prohibition_documented_seam_keys,
        hard_prohibition_registry, hard_prohibition_seeded_consumer_sources,
        in_memory_test_runtime, load_support_pin_contract_terminal_json_document,
        load_support_snapshot_terminal_json_document, project_support_snapshot,
        project_workspace_support_snapshot, query_boundary_source_inventory,
        query_consumer_residue_audit, query_test_backend_residue_audit,
        render_hard_prohibition_reference, support_pinning_contract,
        worth_query_consumer_residue_certification_evidence, worth_query_consumer_residue_registry,
        worth_query_graph_read_bypass_registry, worth_query_test_backend_residue_classes,
        EvidenceReport, EvidenceReportDeclaration, EvidenceReportError, EvidenceReportErrorKind,
        EvidenceReportField, EvidenceReportFieldKind, EvidenceReportFieldParticipation,
        EvidenceReportFieldValue, EvidenceReportScope, WorthQueryBoundaryAuditCoverage,
        WorthQueryBoundaryAuditCoverageMechanism, WorthQueryBoundaryAuditCoverageRow,
        WorthQueryBoundaryAuditError, WorthQueryBoundaryAuditErrorKind,
        WorthQueryBoundaryAuditEvaluation, WorthQueryBoundaryAuditFailure,
        WorthQueryBoundaryAuditFinding, WorthQueryBoundaryAuditFindingKind,
        WorthQueryBoundaryAuditReport, WorthQueryBoundaryAuditSeededSource,
        WorthQueryBoundaryAuditSource, WorthQueryBoundaryAuditSourceInventory,
        WorthQueryBoundaryAuditSourceInventoryBuilder, WorthQueryBoundaryAuditSourceInventoryFile,
        WorthQueryBoundaryAuditSourceSet, WorthQueryBoundaryAuditSourceSite,
        WorthQueryBoundaryAuditSyntaxClass, WorthQueryConsumerResidueAudit,
        WorthQueryConsumerResidueCertificationCaseEvidence, WorthQueryConsumerResidueClass,
        WorthQueryConsumerResidueDetection, WorthQueryConsumerResidueFinding,
        WorthQueryConsumerResidueQueryOwnedRootAuthority, WorthQueryConsumerResidueRegistryRow,
        WorthQueryConsumerResidueReport, WorthQueryConsumerResidueSourceInventory,
        WorthQueryConsumerResidueSourceSite, WorthQueryDownstreamAuthorityAdoption,
        WorthQueryDownstreamAuthorityAdoptionManifest, WorthQueryDownstreamAuthorityAdoptionProof,
        WorthQueryEvidenceReportAdoptionAudit, WorthQueryEvidenceReportAdoptionError,
        WorthQueryEvidenceReportAdoptionErrorKind, WorthQueryEvidenceReportAdoptionEvaluation,
        WorthQueryEvidenceReportAdoptionFinding, WorthQueryEvidenceReportAdoptionFindingKind,
        WorthQueryEvidenceReportAdoptionReport,
        WorthQueryEvidenceReportAdoptionResidueClassification,
        WorthQueryEvidenceReportAdoptionResidueRow, WorthQueryEvidenceReportAdoptionSource,
        WorthQueryEvidenceReportAdoptionSourceSet, WorthQueryEvidenceReportAdoptionSyntaxClass,
        WorthQueryExternalSupportPinContractTerminalJsonDocument,
        WorthQueryExternalSupportSnapshotTerminalJsonDocument,
        WorthQueryGraphObligationAdoptionManifest, WorthQueryGraphObligationAdoptionProof,
        WorthQueryGraphObligationConsumerKit, WorthQueryGraphObligationConsumerKitError,
        WorthQueryGraphObligationConsumerKitErrorKind,
        WorthQueryGraphObligationConsumerRegistrationDeclaration,
        WorthQueryGraphObligationExecutionBackedAdoptionProof,
        WorthQueryGraphObligationExecutionProof, WorthQueryGraphObligationExecutionProofRow,
        WorthQueryGraphObligationInMemoryProof,
        WorthQueryGraphObligationInMemorySelectedObligation,
        WorthQueryGraphObligationInMemoryTestWorkspace,
        WorthQueryGraphObligationLocalCeremonyAudit, WorthQueryGraphObligationLocalCeremonyFinding,
        WorthQueryGraphObligationResidueCertification, WorthQueryGraphObligationResidueManifest,
        WorthQueryGraphObligationResidueRow, WorthQueryGraphObligationSelectorCoverageDeclaration,
        WorthQueryGraphObligationSelectorCoverageRow, WorthQueryGraphObligationSupportPin,
        WorthQueryGraphObligationSupportPinFinding, WorthQueryGraphReadBypassAdoption,
        WorthQueryGraphReadBypassAdoptionError, WorthQueryGraphReadBypassAdoptionErrorKind,
        WorthQueryGraphReadBypassAdoptionManifest, WorthQueryGraphReadBypassAdoptionProof,
        WorthQueryGraphReadBypassAudit, WorthQueryGraphReadBypassAuthorityViolation,
        WorthQueryGraphReadBypassClass, WorthQueryGraphReadBypassCounters,
        WorthQueryGraphReadBypassDetection, WorthQueryGraphReadBypassFinding,
        WorthQueryGraphReadBypassRegistryRow, WorthQueryGraphReadBypassReport,
        WorthQueryGraphReadBypassReportResidueCertification,
        WorthQueryGraphReadBypassResidueCertification, WorthQueryGraphReadBypassResidueError,
        WorthQueryGraphReadBypassResidueErrorKind, WorthQueryGraphReadBypassResidueManifest,
        WorthQueryGraphReadBypassResidueRow, WorthQueryHardProhibitionBoundaryAudit,
        WorthQueryHardProhibitionDocumentationRow, WorthQueryInMemoryTestRuntimeBuilder,
        WorthQueryObservedSupportPin, WorthQueryPinnedSupportStatus,
        WorthQueryPinnedTeachingPosture, WorthQueryProhibitedSeam,
        WorthQueryProhibitionCompileFailFixture, WorthQueryProhibitionEnforcementTier,
        WorthQueryProhibitionRegistry, WorthQueryProhibitionRegistryRow,
        WorthQuerySupportPinContract, WorthQuerySupportPinContractBuilder,
        WorthQuerySupportPinContractSchemaVersion,
        WorthQuerySupportPinContractTerminalJsonDocument, WorthQuerySupportPinDeclaration,
        WorthQuerySupportPinFinding, WorthQuerySupportPinFindingKind, WorthQuerySupportPinReport,
        WorthQuerySupportPinRequirement, WorthQuerySupportPinRequirementDraft,
        WorthQuerySupportPinningError, WorthQuerySupportPinningErrorKind,
        WorthQuerySupportSnapshot, WorthQuerySupportSnapshotError,
        WorthQuerySupportSnapshotErrorKind, WorthQuerySupportSnapshotRow,
        WorthQuerySupportSnapshotSchemaVersion, WorthQuerySupportSnapshotTerminalJsonDocument,
        WorthQueryTestBackendEquivalenceReport, WorthQueryTestBackendEquivalenceRow,
        WorthQueryTestBackendError, WorthQueryTestBackendErrorKind,
        WorthQueryTestBackendResidueAudit, WorthQueryTestBackendResidueFinding,
        WorthQueryTestBackendResidueReport, WorthQueryTestBackendSchema,
    };
}

pub mod foundation {
    pub use super::exports_application::*;
    pub use super::exports_foundation::*;
}

pub mod policy {
    pub use super::exports_policy::*;
    pub use crate::query_basis_lifecycle::{
        admit_basis_capability, AdmittedBasisCapability, BasisAuthorityPosture, BasisEligibility,
        BasisEligibilityCounters, BasisIntentDenial, BasisIntentDenialKind, BasisLifecyclePosture,
        DeniedBasisCapability, DeniedBasisCapabilityKind, NormalizedBasisIntent, RawBasisIntent,
        ScopedCertificationBasis, ScopedInspectionBasis, ScopedMaterializationBasis,
        ScopedMutationPreparationBasis, ScopedObservationBasis, ScopedPreviewCloseoutBasis,
        ScopedReplayBasis, ScopedSubscriptionActivationBasis, ScopedSubscriptionDeclarationBasis,
    };
}

pub mod runtime {
    pub use super::exports_runtime::*;
    pub use super::exports_runtime_phase_nine::*;
}

pub use exports_application::*;
pub use exports_foundation::*;
pub use exports_policy::*;
pub use exports_runtime::*;
pub use exports_runtime_phase_nine::*;
pub use identity_authority::*;

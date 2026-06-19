//! Public API boundary for `forge-query`.
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
    pub use crate::runtime::ForgeQueryRuntimeFacadeFamily;
    pub use crate::{
        compare_test_backend_write_receipts, evidence_report_adoption_audit,
        graph_obligation_consumer_kit, hard_prohibition_boundary_audit,
        hard_prohibition_boundary_audit_coverage, hard_prohibition_compile_fail_fixtures,
        hard_prohibition_documentation_rows, hard_prohibition_documented_seam_keys,
        hard_prohibition_registry, hard_prohibition_seeded_consumer_sources,
        in_memory_test_runtime, load_support_pin_contract_document, load_support_snapshot_document,
        project_support_snapshot, project_workspace_support_snapshot,
        query_boundary_source_inventory, query_test_backend_residue_audit,
        render_hard_prohibition_reference, support_pinning_contract, EvidenceReport,
        EvidenceReportDeclaration, EvidenceReportError, EvidenceReportErrorKind,
        EvidenceReportField, EvidenceReportFieldKind, EvidenceReportFieldParticipation,
        EvidenceReportFieldValue, EvidenceReportScope, ForgeQueryBoundaryAuditCoverage,
        ForgeQueryBoundaryAuditCoverageMechanism, ForgeQueryBoundaryAuditCoverageRow,
        ForgeQueryBoundaryAuditError, ForgeQueryBoundaryAuditErrorKind,
        ForgeQueryBoundaryAuditEvaluation, ForgeQueryBoundaryAuditFailure,
        ForgeQueryBoundaryAuditFinding, ForgeQueryBoundaryAuditFindingKind,
        ForgeQueryBoundaryAuditReport, ForgeQueryBoundaryAuditSeededSource,
        ForgeQueryBoundaryAuditSource, ForgeQueryBoundaryAuditSourceInventory,
        ForgeQueryBoundaryAuditSourceInventoryBuilder, ForgeQueryBoundaryAuditSourceInventoryFile,
        ForgeQueryBoundaryAuditSourceSet, ForgeQueryBoundaryAuditSourceSite,
        ForgeQueryBoundaryAuditSyntaxClass, ForgeQueryEvidenceReportAdoptionAudit,
        ForgeQueryEvidenceReportAdoptionError, ForgeQueryEvidenceReportAdoptionErrorKind,
        ForgeQueryEvidenceReportAdoptionEvaluation, ForgeQueryEvidenceReportAdoptionFinding,
        ForgeQueryEvidenceReportAdoptionFindingKind, ForgeQueryEvidenceReportAdoptionReport,
        ForgeQueryEvidenceReportAdoptionResidueClassification,
        ForgeQueryEvidenceReportAdoptionResidueRow, ForgeQueryEvidenceReportAdoptionSource,
        ForgeQueryEvidenceReportAdoptionSourceSet, ForgeQueryEvidenceReportAdoptionSyntaxClass,
        ForgeQueryGraphObligationAdoptionManifest, ForgeQueryGraphObligationAdoptionProof,
        ForgeQueryGraphObligationConsumerKit, ForgeQueryGraphObligationConsumerKitError,
        ForgeQueryGraphObligationConsumerKitErrorKind,
        ForgeQueryGraphObligationConsumerRegistrationDeclaration,
        ForgeQueryGraphObligationExecutionProof, ForgeQueryGraphObligationExecutionProofRow,
        ForgeQueryGraphObligationInMemoryProof,
        ForgeQueryGraphObligationInMemorySelectedObligation,
        ForgeQueryGraphObligationInMemoryTestWorkspace,
        ForgeQueryGraphObligationLocalCeremonyAudit, ForgeQueryGraphObligationLocalCeremonyFinding,
        ForgeQueryGraphObligationResidueCertification, ForgeQueryGraphObligationResidueManifest,
        ForgeQueryGraphObligationResidueRow, ForgeQueryGraphObligationSelectorCoverageDeclaration,
        ForgeQueryGraphObligationSelectorCoverageRow, ForgeQueryGraphObligationSupportPin,
        ForgeQueryGraphObligationSupportPinFinding, ForgeQueryHardProhibitionBoundaryAudit,
        ForgeQueryHardProhibitionDocumentationRow, ForgeQueryInMemoryTestRuntimeBuilder,
        ForgeQueryObservedSupportPin, ForgeQueryPinnedSupportStatus,
        ForgeQueryPinnedTeachingPosture, ForgeQueryProhibitedSeam,
        ForgeQueryProhibitionCompileFailFixture, ForgeQueryProhibitionEnforcementTier,
        ForgeQueryProhibitionRegistry, ForgeQueryProhibitionRegistryRow,
        ForgeQuerySupportPinContract, ForgeQuerySupportPinContractBuilder,
        ForgeQuerySupportPinContractDocument, ForgeQuerySupportPinContractSchemaVersion,
        ForgeQuerySupportPinDeclaration, ForgeQuerySupportPinFinding,
        ForgeQuerySupportPinFindingKind, ForgeQuerySupportPinReport,
        ForgeQuerySupportPinRequirement, ForgeQuerySupportPinRequirementDraft,
        ForgeQuerySupportPinningError, ForgeQuerySupportPinningErrorKind,
        ForgeQuerySupportSnapshot, ForgeQuerySupportSnapshotDocument,
        ForgeQuerySupportSnapshotError, ForgeQuerySupportSnapshotErrorKind,
        ForgeQuerySupportSnapshotRow, ForgeQuerySupportSnapshotSchemaVersion,
        ForgeQueryTestBackendEquivalenceReport, ForgeQueryTestBackendEquivalenceRow,
        ForgeQueryTestBackendError, ForgeQueryTestBackendErrorKind,
        ForgeQueryTestBackendResidueAudit, ForgeQueryTestBackendResidueFinding,
        ForgeQueryTestBackendResidueReport, ForgeQueryTestBackendSchema,
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

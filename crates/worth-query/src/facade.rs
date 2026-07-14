//! Public API boundary for `worth-query`.
//! External crates should import through this module rather than reaching into
//! internal crate structure directly.

mod exports_aggregate;
mod exports_application;
mod exports_certification;
mod exports_comparison;
mod exports_domain;
mod exports_foundation;
mod exports_history;
mod exports_inspection;
mod exports_live_capability;
mod exports_mutation;
mod exports_policy;
mod exports_preview;
mod exports_read;
mod exports_runtime;
mod exports_runtime_capabilities;
mod exports_runtime_core;
mod exports_runtime_phase_nine;
mod exports_runtime_products;
mod exports_workflow;

pub mod identity_authority {
    pub use crate::identity_authority::*;
}

pub mod consumer_kit {
    pub use crate::consumer_kit::{
        audit_domain_authority_sources, audit_public_authority_surface_symbols,
        compare_test_backend_write_receipts, current_domain_authority_inventory_audit,
        downstream_authority_adoption, evidence_report_adoption_audit,
        graph_obligation_consumer_kit, graph_read_bypass_adoption, graph_read_bypass_audit,
        hard_prohibition_boundary_audit, hard_prohibition_boundary_audit_coverage,
        hard_prohibition_compile_fail_fixtures, hard_prohibition_documentation_rows,
        hard_prohibition_documented_seam_keys, hard_prohibition_registry,
        hard_prohibition_seeded_consumer_sources, in_memory_test_runtime,
        load_support_pin_contract_terminal_json_document,
        load_support_snapshot_terminal_json_document, project_support_snapshot,
        project_workspace_support_snapshot, query_boundary_source_inventory,
        query_consumer_residue_audit, query_test_backend_residue_audit,
        render_hard_prohibition_reference, support_pinning_contract,
        worth_query_consumer_residue_certification_evidence, worth_query_consumer_residue_registry,
        worth_query_domain_authority_inventory_rows, worth_query_domain_installation_grammar,
        worth_query_graph_read_bypass_registry, worth_query_public_authority_surface_rows,
        worth_query_test_backend_residue_classes, EvidenceReport, EvidenceReportDeclaration,
        EvidenceReportError, EvidenceReportErrorKind, EvidenceReportField, EvidenceReportFieldKind,
        EvidenceReportFieldParticipation, EvidenceReportFieldValue, EvidenceReportScope,
        WorthQueryBoundaryAuditCoverage, WorthQueryBoundaryAuditCoverageMechanism,
        WorthQueryBoundaryAuditCoverageRow, WorthQueryBoundaryAuditError,
        WorthQueryBoundaryAuditErrorKind, WorthQueryBoundaryAuditEvaluation,
        WorthQueryBoundaryAuditFailure, WorthQueryBoundaryAuditFinding,
        WorthQueryBoundaryAuditFindingKind, WorthQueryBoundaryAuditReport,
        WorthQueryBoundaryAuditSeededSource, WorthQueryBoundaryAuditSource,
        WorthQueryBoundaryAuditSourceInventory, WorthQueryBoundaryAuditSourceInventoryBuilder,
        WorthQueryBoundaryAuditSourceInventoryFile, WorthQueryBoundaryAuditSourceSet,
        WorthQueryBoundaryAuditSourceSite, WorthQueryBoundaryAuditSyntaxClass,
        WorthQueryConsumerResidueAudit, WorthQueryConsumerResidueCertificationCaseEvidence,
        WorthQueryConsumerResidueClass, WorthQueryConsumerResidueDetection,
        WorthQueryConsumerResidueFinding, WorthQueryConsumerResidueQueryOwnedRootAuthority,
        WorthQueryConsumerResidueRegistryRow, WorthQueryConsumerResidueReport,
        WorthQueryConsumerResidueSourceInventory, WorthQueryConsumerResidueSourceSite,
        WorthQueryDomainAuthorityClass, WorthQueryDomainAuthorityFinding,
        WorthQueryDomainAuthorityFindingKind, WorthQueryDomainAuthorityInventoryAudit,
        WorthQueryDomainAuthorityInventoryRow, WorthQueryDomainAuthoritySource,
        WorthQueryDomainAuthoritySourceSite, WorthQueryDomainInstallationGrammar,
        WorthQueryDomainInstallationGrammarStage, WorthQueryDownstreamAuthorityAdoption,
        WorthQueryDownstreamAuthorityAdoptionManifest, WorthQueryDownstreamAuthorityAdoptionProof,
        WorthQueryDownstreamAuthorityDeletionReceipt, WorthQueryDownstreamAuthorityDeletionRow,
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
        WorthQueryPublicAuthorityOwner, WorthQueryPublicAuthoritySurfaceAudit,
        WorthQueryPublicAuthoritySurfaceClass, WorthQueryPublicAuthoritySurfaceFinding,
        WorthQueryPublicAuthoritySurfaceFindingKind, WorthQueryPublicAuthoritySurfaceRow,
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
    pub use crate::runtime::WorthQueryRuntimeFacadeFamily;
}

/// Certification, migration, manifest, audit, and hostile-test tooling.
///
/// This namespace is intentionally separate from the ordinary product facade.
/// Production consumers should depend on `foundation`, `policy`, or `runtime`.
pub mod certification {
    pub use super::exports_certification::*;
}

pub mod foundation {
    pub use super::exports_application::*;
    pub use super::exports_foundation::*;
}

pub mod policy {
    pub use super::exports_policy::*;
}

/// Declarative one-shot read capability.
///
/// Consumers author bounded read meaning and hand the resulting declaration to
/// Query. Query owns admission, planning, execution routing, and receipt
/// construction.
pub mod read {
    pub use super::exports_read::*;
}

/// Declarative bounded aggregate capability.
pub mod aggregate {
    pub use super::exports_aggregate::*;
}

/// Declarative framework-owned live capability.
pub mod live {
    pub use super::exports_live_capability::*;
}

/// Declarative historical read capability.
pub mod history {
    pub use super::exports_history::*;
}

/// Declarative diff, lineage, and correspondence capability.
pub mod comparison {
    pub use super::exports_comparison::*;
}

/// Declarative authoritative mutation capability.
pub mod mutation {
    pub use super::exports_mutation::*;
}

/// Declarative scoped preview capability.
pub mod preview {
    pub use super::exports_preview::*;
}

/// Declarative preview, promotion, and writeback workflow capability.
pub mod workflow {
    pub use super::exports_workflow::*;
}

/// Domain contribution contracts and their ordinary workflow journey.
pub mod domain {
    pub use super::exports_domain::*;
}

/// Common outcome navigation and declarative inspection capability.
pub mod inspection {
    pub use super::exports_inspection::*;
}

pub mod runtime {
    pub use super::exports_runtime::*;
    pub use super::exports_runtime_capabilities::*;
    pub use super::exports_runtime_core::*;
    pub use super::exports_runtime_phase_nine::*;
    pub use super::exports_runtime_products::*;
}

#[cfg(test)]
pub(crate) use crate::query_context::{
    admit_and_scope_legacy_query_basis_context_for_test, bind_legacy_query_basis_context,
    QueryBasisContextRequest,
};

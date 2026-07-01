use super::catalog_kernel;
use super::catalog_query;
use super::catalog_spatial;
use super::catalog_topo;
use super::row::{PlannerOwnedRoutingInventoryRow, PlannerOwnedRoutingSurfaceIdentity as Surface};

pub(super) fn current_rows() -> Vec<PlannerOwnedRoutingInventoryRow> {
    let mut rows = Vec::new();
    rows.extend(catalog_kernel::rows());
    rows.extend(catalog_topo::rows());
    rows.extend(catalog_spatial::rows());
    rows.extend(catalog_query::rows());
    rows
}

pub(super) fn required_surfaces() -> &'static [Surface] {
    use Surface as S;
    &[
        S::CurrentWorthTouchedGraphConflictPublicCloseout,
        S::WorthTouchedGraphConflictPublicCloseout,
        S::WorthTouchedGraphConflictArchitectureAlignmentReport,
        S::WorthTouchedGraphConflictArchitectureAlignmentReportRow,
        S::WorthTouchedGraphConflictDeletionAlignmentRow,
        S::CurrentPublicCloseoutConsumerResidueManifest,
        S::PublicCloseoutConsumerResidueBoundaryPosture,
        S::PublicCloseoutConsumerResidueDisposition,
        S::PublicCloseoutConsumerResidueOwner,
        S::PublicCloseoutConsumerResidueRow,
        S::WorthTouchedGraphConflictPublicCloseoutError,
        S::WorthTouchedGraphConflictPublicCloseoutErrorKind,
        S::WorthTouchedGraphConflictResidueChain,
        S::WorthTouchedGraphConflictResidueBoundaryPosture,
        S::WorthTouchedGraphConflictResidueRow,
        S::CurrentWorthTouchedGraphConflictMilestoneFifteenSeed,
        S::WorthTouchedGraphConflictMilestoneFifteenSeed,
        S::CurrentMilestoneFifteenPlannerProofInput,
        S::WorthTouchedGraphConflictMilestoneFifteenPlannerProofInput,
        S::WorthTouchedGraphConflictProofChain,
        S::CurrentWorthTouchedGraphConflictSourceFirewallReport,
        S::CurrentWorthTouchedGraphConflictSourceFirewallCloseout,
        S::WorthTouchedGraphConflictSourceFirewallCloseout,
        S::WorthTouchedGraphConflictSourceFirewallCloseoutError,
        S::WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind,
        S::WorthTouchedGraphConflictForbiddenSurface,
        S::WorthTouchedGraphConflictSourceFirewallRegionReport,
        S::WorthTouchedGraphConflictSourceFirewallReport,
        S::WorthTouchedGraphConflictSourceFirewallViolation,
        S::CurrentWorthWorkloadOrdinaryConsumerSweepCloseout,
        S::DiagnosticSurfaceDeterministicDigest,
        S::DerivedFallbackReport,
        S::DerivedInvalidationReport,
        S::DerivedRebuildReport,
        S::CurrentTopologyQueryBackedConsumerCutover,
        S::TopologyQueryBackedConsumerCutoverCurrentError,
        S::AdmitTopologyQueryBackedConsumerCutover,
        S::TopologyQueryBackedConsumerCutover,
        S::TopologyQueryBackedConsumerFamilyRow,
        S::TopologyReadModelReusePosture,
        S::CurrentQueryBackedConsumerResidueManifest,
        S::QueryBackedConsumerResidueDisposition,
        S::QueryBackedConsumerResidueOwner,
        S::QueryBackedConsumerResidueRow,
        S::DerivedReadDiagnostics,
        S::CurrentEvidenceLookupPublicCloseout,
        S::CurrentEvidenceLookupPublicCloseoutAssemblyInput,
        S::EvidenceLookupPublicCloseoutAssemblyInput,
        S::EvidenceLookupPublicCloseout,
        S::EvidenceLookupPublicCloseoutDisposition,
        S::EvidenceLookupPublicCloseoutFamilyStageRow,
        S::EvidenceLookupPublicCloseoutCounters,
        S::CurrentEvidenceLookupPublicCloseoutResidueManifest,
        S::EvidenceLookupPublicCloseoutResidueDisposition,
        S::EvidenceLookupPublicCloseoutResidueOwner,
        S::EvidenceLookupPublicCloseoutResidueRow,
        S::EvidenceLookupPublicCloseoutError,
        S::EvidenceLookupPublicCloseoutErrorKind,
        S::QueryWorkspacePublicSupportMatrix,
        S::QueryWorkspacePublicApiContract,
        S::QueryWorkspacePublicHandleContract,
        S::QueryWorkspaceAdmitPublicApiFamily,
        S::QueryProjectWorkspaceSupportSnapshot,
        S::QuerySupportPinningContract,
        S::QueryHardProhibitionBoundaryAudit,
        S::QueryConsumerResidueAudit,
        S::QueryConsumeProjectionFacts,
        S::QueryDeclareProjectionFactConsumption,
        S::QueryLowerRuntimeBoundaryEnvelopeSupport,
        S::QueryLowerRuntimeBoundarySourceSupport,
        S::QueryDeclarationScopedCapabilitySupport,
        S::QueryDeclarationScopedTraceabilitySupport,
        S::QueryDeclarationEnvelopeInput,
        S::QueryDeclarationEnvelope,
    ]
}

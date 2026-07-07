use super::classification::{
    PlannerOwnedRoutingDisplacedLane, PlannerOwnedRoutingDisposition,
    PlannerOwnedRoutingLifecycleRole, PlannerOwnedRoutingOwner, PlannerOwnedRoutingReplacementLane,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlannerOwnedRoutingSurfaceIdentity {
    CurrentWorthTouchedGraphConflictPublicCloseout,
    WorthTouchedGraphConflictPublicCloseout,
    WorthTouchedGraphConflictArchitectureAlignmentReport,
    WorthTouchedGraphConflictArchitectureAlignmentReportRow,
    WorthTouchedGraphConflictDeletionAlignmentRow,
    CurrentPublicCloseoutConsumerResidueManifest,
    PublicCloseoutConsumerResidueManifestError,
    PublicCloseoutConsumerResidueBoundaryPosture,
    PublicCloseoutConsumerResidueDisposition,
    PublicCloseoutConsumerResidueOwner,
    PublicCloseoutConsumerResidueRow,
    WorthTouchedGraphConflictResidueChain,
    WorthTouchedGraphConflictResidueBoundaryPosture,
    WorthTouchedGraphConflictResidueRow,
    CurrentWorthTouchedGraphConflictMilestoneFifteenSeed,
    WorthTouchedGraphConflictMilestoneFifteenSeed,
    WorthTouchedGraphConflictMilestoneFifteenPlannerProofInput,
    WorthTouchedGraphConflictProofChain,
    WorthTouchedGraphConflictPublicCloseoutError,
    WorthTouchedGraphConflictPublicCloseoutErrorKind,
    CurrentWorthTouchedGraphConflictSourceFirewallReport,
    CurrentWorthTouchedGraphConflictSourceFirewallCloseout,
    WorthTouchedGraphConflictSourceFirewallCloseout,
    WorthTouchedGraphConflictSourceFirewallCloseoutError,
    WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind,
    WorthTouchedGraphConflictForbiddenSurface,
    WorthTouchedGraphConflictSourceFirewallRegionReport,
    WorthTouchedGraphConflictSourceFirewallReport,
    WorthTouchedGraphConflictSourceFirewallViolation,
    CurrentWorthWorkloadOrdinaryConsumerSweepCloseout,
    CurrentTopologyQueryBackedConsumerCutover,
    TopologyQueryBackedConsumerCutoverCurrentError,
    TopologyQueryBackedConsumerCutover,
    TopologyQueryBackedConsumerFamilyRow,
    TopologyQueryBackedReadFamilySelectedRouteAuthority,
    AdmitCurrentTopologyQueryBackedConsumerCutoverWithSelectedRouteAuthority,
    AdmitTopologyQueryBackedConsumerCutover,
    TopologyReadModelReusePosture,
    CurrentQueryBackedConsumerResidueManifest,
    QueryBackedConsumerResidueDisposition,
    QueryBackedConsumerResidueOwner,
    QueryBackedConsumerResidueRow,
    DerivedFallbackReport,
    DerivedInvalidationReport,
    DerivedInvalidationTargetRow,
    DerivedRebuildReport,
    DerivedReadDiagnostics,
    DerivedValidationExecutionReport,
    BuildDerivedReadDiagnostics,
    DeriveTopologyValidationReport,
    CurrentEvidenceLookupPublicCloseout,
    CurrentEvidenceLookupPublicCloseoutRouteInput,
    CurrentEvidenceLookupPublicCloseoutAssemblyInput,
    EvidenceLookupPublicCloseoutAssemblyInput,
    EvidenceLookupPublicCloseoutRouteInput,
    EvidenceLookupPublicCloseout,
    EvidenceLookupPublicCloseoutDisposition,
    EvidenceLookupPublicCloseoutFamilyStageRow,
    EvidenceLookupPublicCloseoutCounters,
    CurrentEvidenceLookupPublicCloseoutResidueManifest,
    EvidenceLookupPublicCloseoutResidueDisposition,
    EvidenceLookupPublicCloseoutResidueOwner,
    EvidenceLookupPublicCloseoutResidueRow,
    EvidenceLookupPublicCloseoutError,
    EvidenceLookupPublicCloseoutErrorKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlannerOwnedRoutingInventoryRow {
    surface_identity: PlannerOwnedRoutingSurfaceIdentity,
    displaced_lane: PlannerOwnedRoutingDisplacedLane,
    source_path: &'static str,
    surface_name: &'static str,
    current_authority_sources: &'static [&'static str],
    lifecycle_role: PlannerOwnedRoutingLifecycleRole,
    disposition: PlannerOwnedRoutingDisposition,
    owner: PlannerOwnedRoutingOwner,
    replacement_lane: PlannerOwnedRoutingReplacementLane,
    blocker: &'static str,
    removal_trigger: &'static str,
    ordinary_path: bool,
    certification_only: bool,
    scan_token: &'static str,
}

impl PlannerOwnedRoutingInventoryRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        surface_identity: PlannerOwnedRoutingSurfaceIdentity,
        displaced_lane: PlannerOwnedRoutingDisplacedLane,
        source_path: &'static str,
        surface_name: &'static str,
        current_authority_sources: &'static [&'static str],
        lifecycle_role: PlannerOwnedRoutingLifecycleRole,
        disposition: PlannerOwnedRoutingDisposition,
        owner: PlannerOwnedRoutingOwner,
        replacement_lane: PlannerOwnedRoutingReplacementLane,
        blocker: &'static str,
        removal_trigger: &'static str,
        ordinary_path: bool,
        certification_only: bool,
        scan_token: &'static str,
    ) -> Self {
        Self {
            surface_identity,
            displaced_lane,
            source_path,
            surface_name,
            current_authority_sources,
            lifecycle_role,
            disposition,
            owner,
            replacement_lane,
            blocker,
            removal_trigger,
            ordinary_path,
            certification_only,
            scan_token,
        }
    }

    pub const fn surface_identity(&self) -> PlannerOwnedRoutingSurfaceIdentity {
        self.surface_identity
    }
    pub const fn displaced_lane(&self) -> PlannerOwnedRoutingDisplacedLane {
        self.displaced_lane
    }
    pub const fn source_path(&self) -> &'static str {
        self.source_path
    }
    pub const fn surface_name(&self) -> &'static str {
        self.surface_name
    }
    pub const fn current_authority_sources(&self) -> &'static [&'static str] {
        self.current_authority_sources
    }
    pub const fn lifecycle_role(&self) -> PlannerOwnedRoutingLifecycleRole {
        self.lifecycle_role
    }
    pub const fn disposition(&self) -> PlannerOwnedRoutingDisposition {
        self.disposition
    }
    pub const fn replacement_lane(&self) -> PlannerOwnedRoutingReplacementLane {
        self.replacement_lane
    }
    pub const fn blocker(&self) -> &'static str {
        self.blocker
    }
    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }
    pub const fn ordinary_path(&self) -> bool {
        self.ordinary_path
    }
    pub const fn scan_token(&self) -> &'static str {
        self.scan_token
    }
}

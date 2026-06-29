use crate::workload_composition::ConflictBatchAdmissionSurfaceIdentity;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthTouchedGraphConflictForbiddenSurface {
    EntityOnlyOverlapHelper,
    BroadTopologyScan,
    BroadEvidenceScan,
    LockFirstAdmission,
    SpeculativeRollbackAdmission,
    RollbackAdmission,
    CallerOwnedSerialization,
    CallerOwnedCompatibility,
    GenericOverlapSecondAuthorityLane,
}

impl WorthTouchedGraphConflictForbiddenSurface {
    pub const fn phase_twelve_relapse_categories() -> &'static [Self] {
        &[
            Self::EntityOnlyOverlapHelper,
            Self::BroadTopologyScan,
            Self::GenericOverlapSecondAuthorityLane,
            Self::BroadEvidenceScan,
            Self::LockFirstAdmission,
            Self::SpeculativeRollbackAdmission,
            Self::CallerOwnedCompatibility,
            Self::CallerOwnedSerialization,
        ]
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EntityOnlyOverlapHelper => "entity-only overlap helper",
            Self::BroadTopologyScan => "broad topology scan",
            Self::BroadEvidenceScan => "broad evidence scan",
            Self::LockFirstAdmission => "lock-first admission",
            Self::SpeculativeRollbackAdmission => "speculative rollback admission",
            Self::RollbackAdmission => "rollback admission",
            Self::CallerOwnedSerialization => "caller-owned serialization",
            Self::CallerOwnedCompatibility => "caller-owned compatibility",
            Self::GenericOverlapSecondAuthorityLane => "generic overlap second authority lane",
        }
    }

    pub(crate) const fn from_surface_identity(
        surface_identity: ConflictBatchAdmissionSurfaceIdentity,
    ) -> Option<Self> {
        use ConflictBatchAdmissionSurfaceIdentity as Surface;

        match surface_identity {
            Surface::WorkloadCatalogPlanarBooleanCoplanarOverlapPair
            | Surface::WorkloadCatalogCoplanarOverlapStorm
            | Surface::OperatorOutcomeFromCoplanarOverlapReceipt
            | Surface::OperatorReceiptSetFromCoplanarOverlapReceipt
            | Surface::CoplanarOverlapWorkloadOperator
            | Surface::CoplanarOverlapWorkloadOperatorFromStageLinks
            | Surface::CoplanarOverlapWorkloadOperatorWithCertificationContext
            | Surface::CoplanarOverlapWorkloadOperatorWithExtractionBundle
            | Surface::CoplanarOverlapWorkloadOperatorExecute
            | Surface::CoplanarOverlapOperatorReceipt
            | Surface::CertifiedProjectedOverlapBridgeAuthority
            | Surface::ProjectedOverlapFaceSet
            | Surface::ProjectedOverlapFaceGeometry
            | Surface::ProjectedOverlapCandidatePolicy
            | Surface::CertifiedProjectedOverlapFaceSet
            | Surface::CertifiedProjectedOverlapFace
            | Surface::ProjectedOverlapFaceDenial
            | Surface::ProjectedOverlapExtractionContracts
            | Surface::CoplanarOverlapExtractionBundle
            | Surface::PlanarBooleanRawEdgeSplitScheduleSetAssembleFromAdmittedCandidates
            | Surface::PlanarBooleanOverlapEdgeChainsModule
            | Surface::PlanarBooleanBuildOverlapEdgeChains
            | Surface::PlanarBooleanOverlapEdgeChainSet
            | Surface::PlanarBooleanOverlapEdgeChainSetCertifiesPreparedOverlapChains
            | Surface::PlanarBooleanOverlapEdgeChain
            | Surface::PlanarBooleanOverlapEdgeChainMember
            | Surface::PlanarBooleanOverlapEdgeChainCounters
            | Surface::PlanarBooleanOverlapEdgeChainCountersPartialOverlapChains
            | Surface::PlanarBooleanSplitChainValidationCountersOverlapChainsChecked
            | Surface::PlanarBooleanOverlapEdgeChainDenial
            | Surface::PlanarBooleanOverlapEdgeChainDenialKind
            | Surface::PlanarBooleanOverlapChainBoundaryRole
            | Surface::PlanarBooleanOverlapChainPosture
            | Surface::TopoEdgeSplitOverlapChainPublicContract => {
                Some(Self::GenericOverlapSecondAuthorityLane)
            }
            Surface::CertifiedProjectedOverlapCandidatePair
            | Surface::CertifiedProjectedOverlapCandidatePairs
            | Surface::OverlapChainIndexedInputs
            | Surface::FragmentOverlapsSubdivision => Some(Self::EntityOnlyOverlapHelper),
            Surface::TraversalViewsOldAuthorityResidue => Some(Self::BroadTopologyScan),
            Surface::ReplayScopeProductBroadReceiptScanCounter
            | Surface::UndoScopeProductBroadReceiptScanCounter
            | Surface::EvidenceLookupDiagnosticsHiddenBroadReceiptScanCounter
            | Surface::EvidenceLookupInventoryBroadScanRowCounter
            | Surface::EvidenceLookupSourceFirewallMentionsBroadReceiptScan
            | Surface::EvidenceLookupSourceFirewallBroadReceiptScanRowCounter
            | Surface::EvidenceLookupWorkloadCutoverBroadReceiptScanCounter
            | Surface::EvidenceLookupStageCutoverBroadReceiptScanCounter
            | Surface::EvidenceLookupStageCutoverWithTestBroadReceiptScanCount
            | Surface::EvidenceLookupPlanSelectionBroadReceiptScanCounter
            | Surface::ReplayUndoSpatialBoundaryFixtureWithTestBroadReceiptScanCount
            | Surface::EvidenceLookupConsumedWorkloadHandoffWithTestBroadReceiptScanCount
            | Surface::SpatialTouchBroadLedgerScanCounter => Some(Self::BroadEvidenceScan),
            Surface::SpatialRollbackBooleanEventLedgerAdmission
            | Surface::SpatialRollbackProjectionReceiptAdmission
            | Surface::TopologyRollbackTraversalViewsAdmission
            | Surface::TopologyRollbackMaterializedGraphAdmission => Some(Self::RollbackAdmission),
            Surface::HighValenceRebuildMotionCompatibilitySetter
            | Surface::HighValenceRequireRebuildMotionCompatibility
            | Surface::HighValenceRebuildMotionCompatibility
            | Surface::DuplicateSplitRejectContradictorySameParameterPoints
            | Surface::PointSplitCompatibilityBasis => Some(Self::CallerOwnedCompatibility),
            _ => None,
        }
    }
}

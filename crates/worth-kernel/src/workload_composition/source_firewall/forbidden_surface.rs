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
    DisplacedCacheKeyCarrier,
    LocalComparatorFolklore,
    CallerOwnedReuseDecision,
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

    pub const fn phase_fifteen_relapse_categories() -> &'static [Self] {
        &[
            Self::BroadTopologyScan,
            Self::BroadEvidenceScan,
            Self::CallerOwnedSerialization,
            Self::CallerOwnedCompatibility,
            Self::GenericOverlapSecondAuthorityLane,
            Self::DisplacedCacheKeyCarrier,
            Self::LocalComparatorFolklore,
            Self::CallerOwnedReuseDecision,
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
            Self::DisplacedCacheKeyCarrier => "displaced cache-key carrier",
            Self::LocalComparatorFolklore => "local comparator folklore",
            Self::CallerOwnedReuseDecision => "caller-owned reuse decision",
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
            Surface::ConflictInputLookupRouteDenial
            | Surface::LookupConsumedWorkloadReuseProductSerialization
            | Surface::LookupConsumedWorkloadRequireResolutionProductSerialization
            | Surface::LookupConsumedWorkloadMismatchLocusNameSerialization => {
                Some(Self::CallerOwnedSerialization)
            }
            Surface::HighValenceRebuildMotionCompatibilitySetter
            | Surface::HighValenceRequireRebuildMotionCompatibility
            | Surface::HighValenceRebuildMotionCompatibility
            | Surface::DuplicateSplitRejectContradictorySameParameterPoints
            | Surface::PointSplitCompatibilityBasis
            | Surface::LookupConsumedWorkloadReuseProductCompatibility
            | Surface::LookupConsumedWorkloadRequireResolutionProductCompatibility
            | Surface::LookupConsumedWorkloadMismatchLocusNameCompatibility => {
                Some(Self::CallerOwnedCompatibility)
            }
            Surface::TopologySelectedCompatibilityBasisIdentityStruct
            | Surface::TopologySelectedCompatibilityBasisIdentityIdentityDigest
            | Surface::SelectedTopologyEquivalenceFamilyCompatibilityBasisIdentity
            | Surface::SpatialSelectedCompatibilityBasisIdentityStruct
            | Surface::SpatialSelectedCompatibilityBasisIdentityIdentityDigest
            | Surface::SelectedSpatialEquivalenceFamilyCompatibilityBasisIdentity => {
                Some(Self::DisplacedCacheKeyCarrier)
            }
            Surface::SelectedTopologyEquivalenceFamilyCompatibilityPosture
            | Surface::TopologySelectedEquivalenceFamilyDeclarationCompatibilityPosture
            | Surface::TopologySelectedEquivalenceComparatorContractCompatibilityPosture
            | Surface::TopologyCompatibilityPostureEnum
            | Surface::SelectedSpatialEquivalenceFamilyCompatibilityPosture
            | Surface::SpatialSelectedEquivalenceFamilyDeclarationCompatibilityPosture
            | Surface::SpatialCompatibilityPostureEnum => Some(Self::LocalComparatorFolklore),
            Surface::TopologyDerivedReuseDecisionSelectedCompatibilityBasisIdentityDigest
            | Surface::TopologyDerivedRebuildDenialSelectedCompatibilityBasisIdentityDigest
            | Surface::TopologyDerivedReuseExecutionInputSelectedCompatibilityBasisIdentityDigest
            | Surface::DerivedEquivalenceContractReportSelectedCompatibilityBasisIdentityDigest
            | Surface::LookupConsumedReuseResolutionSelectedCompatibilityBasisIdentityDigest
            | Surface::EvidenceLookupIndexReuseDecisionSelectedCompatibilityBasisIdentityDigest
            | Surface::EvidenceLookupIndexRebuildDenialSelectedCompatibilityBasisIdentityDigest => {
                Some(Self::CallerOwnedReuseDecision)
            }
            _ => None,
        }
    }
}

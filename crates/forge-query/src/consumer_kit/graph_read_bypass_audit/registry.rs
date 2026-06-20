#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphReadBypassClass {
    ManualRelationRowLoop,
    PerNodeNeighborLookup,
    AdHocAdjacencyMap,
    ManualFrontierScan,
    ManualVisitedSetTraversal,
    SurfaceLocalGraphCache,
    BroadBooleanGraphScan,
    HiddenGraphReadFallback,
    RuntimeReadLoweringBypass,
    TestSupportClaimingProductionProof,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphReadBypassAuthorityViolation {
    CallerOwnedGraphTraversal,
    CallerOwnedIndexMaterialization,
    CallerOwnedGraphCache,
    HiddenFallbackAroundAccessPlanning,
    ProductionProofClaimWithoutAccessReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadBypassDetection {
    MaskedSourceSyntax,
    MaskedSourceExactText,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadBypassRegistryRow {
    class: ForgeQueryGraphReadBypassClass,
    authority_violation: ForgeQueryGraphReadBypassAuthorityViolation,
    detection: ForgeQueryGraphReadBypassDetection,
    detection_key: &'static str,
    explanation: &'static str,
    replacement_lane: &'static str,
    introduced_in: &'static str,
    default_residue_cap: usize,
}

impl ForgeQueryGraphReadBypassClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ManualRelationRowLoop => "manual-relation-row-loop",
            Self::PerNodeNeighborLookup => "per-node-neighbor-lookup",
            Self::AdHocAdjacencyMap => "ad-hoc-adjacency-map",
            Self::ManualFrontierScan => "manual-frontier-scan",
            Self::ManualVisitedSetTraversal => "manual-visited-set-traversal",
            Self::SurfaceLocalGraphCache => "surface-local-graph-cache",
            Self::BroadBooleanGraphScan => "broad-boolean-graph-scan",
            Self::HiddenGraphReadFallback => "hidden-graph-read-fallback",
            Self::RuntimeReadLoweringBypass => "runtime-read-lowering-bypass",
            Self::TestSupportClaimingProductionProof => "test-support-claiming-production-proof",
        }
    }
}

impl ForgeQueryGraphReadBypassAuthorityViolation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CallerOwnedGraphTraversal => "caller-owned-graph-traversal",
            Self::CallerOwnedIndexMaterialization => "caller-owned-index-materialization",
            Self::CallerOwnedGraphCache => "caller-owned-graph-cache",
            Self::HiddenFallbackAroundAccessPlanning => "hidden-fallback-around-access-planning",
            Self::ProductionProofClaimWithoutAccessReceipt => {
                "production-proof-claim-without-access-receipt"
            }
        }
    }

    pub fn is_graph_read_access_bypass(self) -> bool {
        true
    }
}

impl ForgeQueryGraphReadBypassDetection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MaskedSourceSyntax => "masked-source-syntax",
            Self::MaskedSourceExactText => "masked-source-exact-text",
        }
    }
}

impl ForgeQueryGraphReadBypassRegistryRow {
    pub(crate) const fn new(
        class: ForgeQueryGraphReadBypassClass,
        authority_violation: ForgeQueryGraphReadBypassAuthorityViolation,
        detection: ForgeQueryGraphReadBypassDetection,
        detection_key: &'static str,
        explanation: &'static str,
        replacement_lane: &'static str,
        default_residue_cap: usize,
    ) -> Self {
        Self {
            class,
            authority_violation,
            detection,
            detection_key,
            explanation,
            replacement_lane,
            introduced_in: "Milestone 9.10 Phase 15",
            default_residue_cap,
        }
    }

    pub fn class(&self) -> ForgeQueryGraphReadBypassClass {
        self.class
    }

    pub fn authority_violation(&self) -> ForgeQueryGraphReadBypassAuthorityViolation {
        self.authority_violation
    }

    pub fn detection(&self) -> ForgeQueryGraphReadBypassDetection {
        self.detection
    }

    pub fn detection_key(&self) -> &'static str {
        self.detection_key
    }

    pub fn explanation(&self) -> &'static str {
        self.explanation
    }

    pub fn replacement_lane(&self) -> &'static str {
        self.replacement_lane
    }

    pub fn introduced_in(&self) -> &'static str {
        self.introduced_in
    }

    pub fn default_residue_cap(&self) -> usize {
        self.default_residue_cap
    }
}

pub fn forge_query_graph_read_bypass_registry() -> &'static [ForgeQueryGraphReadBypassRegistryRow] {
    GRAPH_READ_BYPASS_REGISTRY
}

pub(crate) fn registry_row_for_class(
    class: ForgeQueryGraphReadBypassClass,
) -> &'static ForgeQueryGraphReadBypassRegistryRow {
    GRAPH_READ_BYPASS_REGISTRY
        .iter()
        .find(|row| row.class() == class)
        .expect("every graph-read bypass class must have a registry row")
}

const GRAPH_READ_BYPASS_REGISTRY: &[ForgeQueryGraphReadBypassRegistryRow] = &[
    registry_row(
        ForgeQueryGraphReadBypassClass::ManualRelationRowLoop,
        ForgeQueryGraphReadBypassAuthorityViolation::CallerOwnedGraphTraversal,
        ForgeQueryGraphReadBypassDetection::MaskedSourceSyntax,
        "manual-relation-row-loop",
        "consumer manually loops over relation rows instead of using graph-read access planning",
        "ForgeQueryGraphReadAccessAdmission",
        0,
    ),
    registry_row(
        ForgeQueryGraphReadBypassClass::PerNodeNeighborLookup,
        ForgeQueryGraphReadBypassAuthorityViolation::CallerOwnedGraphTraversal,
        ForgeQueryGraphReadBypassDetection::MaskedSourceSyntax,
        "per-node-neighbor-lookup",
        "consumer performs per-node neighbor lookup that can become hidden N+1 traversal",
        "ForgeQueryAdmittedGraphReadAccessPlan",
        0,
    ),
    registry_row(
        ForgeQueryGraphReadBypassClass::AdHocAdjacencyMap,
        ForgeQueryGraphReadBypassAuthorityViolation::CallerOwnedIndexMaterialization,
        ForgeQueryGraphReadBypassDetection::MaskedSourceSyntax,
        "ad-hoc-adjacency-map",
        "consumer builds local adjacency state instead of using admitted access structures",
        "ForgeQueryGraphIndexInventoryMatch",
        0,
    ),
    registry_row(
        ForgeQueryGraphReadBypassClass::ManualFrontierScan,
        ForgeQueryGraphReadBypassAuthorityViolation::CallerOwnedGraphTraversal,
        ForgeQueryGraphReadBypassDetection::MaskedSourceSyntax,
        "manual-frontier-scan",
        "consumer owns traversal frontier instead of entering streaming or planned access",
        "ForgeQueryGraphReadStreamingPlan",
        0,
    ),
    registry_row(
        ForgeQueryGraphReadBypassClass::ManualVisitedSetTraversal,
        ForgeQueryGraphReadBypassAuthorityViolation::CallerOwnedGraphTraversal,
        ForgeQueryGraphReadBypassDetection::MaskedSourceSyntax,
        "manual-visited-set-traversal",
        "consumer owns visited-set traversal breadth outside Query access counters",
        "ForgeQueryGraphReadAccessPlanConsumption",
        0,
    ),
    registry_row(
        ForgeQueryGraphReadBypassClass::SurfaceLocalGraphCache,
        ForgeQueryGraphReadBypassAuthorityViolation::CallerOwnedGraphCache,
        ForgeQueryGraphReadBypassDetection::MaskedSourceExactText,
        "local-graph-cache",
        "consumer keeps surface-local graph cache with no Query lifecycle receipt",
        "ForgeQueryPersistentGraphIndexRequirementDeclaration",
        0,
    ),
    registry_row(
        ForgeQueryGraphReadBypassClass::BroadBooleanGraphScan,
        ForgeQueryGraphReadBypassAuthorityViolation::CallerOwnedGraphTraversal,
        ForgeQueryGraphReadBypassDetection::MaskedSourceSyntax,
        "broad-boolean-graph-scan",
        "consumer scans graph rows with broad boolean filtering outside selectivity planning",
        "ForgeQueryBooleanSelectivityShape",
        0,
    ),
    registry_row(
        ForgeQueryGraphReadBypassClass::HiddenGraphReadFallback,
        ForgeQueryGraphReadBypassAuthorityViolation::HiddenFallbackAroundAccessPlanning,
        ForgeQueryGraphReadBypassDetection::MaskedSourceExactText,
        "hidden-graph-read-fallback",
        "consumer hides graph-read fallback instead of returning typed access denial",
        "ForgeQueryGraphReadAccessDenial",
        0,
    ),
    registry_row(
        ForgeQueryGraphReadBypassClass::RuntimeReadLoweringBypass,
        ForgeQueryGraphReadBypassAuthorityViolation::HiddenFallbackAroundAccessPlanning,
        ForgeQueryGraphReadBypassDetection::MaskedSourceExactText,
        "runtime-read-lowering-bypass",
        "consumer reaches runtime read lowering directly instead of consuming admitted plans",
        "ForgeQueryWorkspace::read_family_intent",
        0,
    ),
    registry_row(
        ForgeQueryGraphReadBypassClass::TestSupportClaimingProductionProof,
        ForgeQueryGraphReadBypassAuthorityViolation::ProductionProofClaimWithoutAccessReceipt,
        ForgeQueryGraphReadBypassDetection::MaskedSourceExactText,
        "claim-production-graph-read-proof-for-test",
        "test support claims production graph-read proof without an access receipt",
        "ForgeQueryGraphReadAccessReceiptSummary",
        0,
    ),
];

const fn registry_row(
    class: ForgeQueryGraphReadBypassClass,
    authority_violation: ForgeQueryGraphReadBypassAuthorityViolation,
    detection: ForgeQueryGraphReadBypassDetection,
    detection_key: &'static str,
    explanation: &'static str,
    replacement_lane: &'static str,
    default_residue_cap: usize,
) -> ForgeQueryGraphReadBypassRegistryRow {
    ForgeQueryGraphReadBypassRegistryRow::new(
        class,
        authority_violation,
        detection,
        detection_key,
        explanation,
        replacement_lane,
        default_residue_cap,
    )
}

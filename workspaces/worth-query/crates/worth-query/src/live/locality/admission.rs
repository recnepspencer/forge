#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LocalityAdmissionClass {
    DetailRegion,
    DetailPartition,
    OrderedCollectionPartition,
    BoundedMaterializationRegion,
}

impl LocalityAdmissionClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DetailRegion => "detail_region",
            Self::DetailPartition => "detail_partition",
            Self::OrderedCollectionPartition => "ordered_collection_partition",
            Self::BoundedMaterializationRegion => "bounded_materialization_region",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LocalitySemanticBasis {
    DetailProjectionFields,
    OrderedCollectionMembershipAndOrdering,
    BoundedTraversalMaterialization,
}

impl LocalitySemanticBasis {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DetailProjectionFields => "detail_projection_fields",
            Self::OrderedCollectionMembershipAndOrdering => {
                "ordered_collection_membership_and_ordering"
            }
            Self::BoundedTraversalMaterialization => "bounded_traversal_materialization",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LocalityScopeAdmission {
    RegionOnly,
    PartitionOnly,
    RegionOrPartition,
}

impl LocalityScopeAdmission {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RegionOnly => "region_only",
            Self::PartitionOnly => "partition_only",
            Self::RegionOrPartition => "region_or_partition",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StreamLoweringAdmissionClass {
    DetailCurrentStateOnly,
    CollectionCdcProjectedPatchOnly,
    DeferredBoundedMaterialization,
}

impl StreamLoweringAdmissionClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DetailCurrentStateOnly => "detail_current_state_only",
            Self::CollectionCdcProjectedPatchOnly => "collection_cdc_projected_patch_only",
            Self::DeferredBoundedMaterialization => "deferred_bounded_materialization",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LocalityCostPosture {
    SingleSliceNarrowing,
    PartitionScopedMembershipNarrowing,
    BoundedTraversalRegionNarrowing,
}

impl LocalityCostPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SingleSliceNarrowing => "single_slice_narrowing",
            Self::PartitionScopedMembershipNarrowing => "partition_scoped_membership_narrowing",
            Self::BoundedTraversalRegionNarrowing => "bounded_traversal_region_narrowing",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LocalityMaintenanceClass {
    NarrowPatch,
    OffRegionSuppression,
    WideningDenied,
    WideningAdmitted,
}

impl LocalityMaintenanceClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NarrowPatch => "narrow_patch",
            Self::OffRegionSuppression => "off_region_suppression",
            Self::WideningDenied => "widening_denied",
            Self::WideningAdmitted => "widening_admitted",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LocalityPerformanceStatus {
    VerifiedNarrowing,
}

impl LocalityPerformanceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::VerifiedNarrowing => "verified_narrowing",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LocalityBreadthBudget {
    pub(in crate::live) limit: usize,
}

impl LocalityBreadthBudget {
    pub fn limit(&self) -> usize {
        self.limit
    }
    pub(in crate::live) fn single_surface() -> Self {
        Self { limit: 1 }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LocalityWideningBudget {
    pub(in crate::live) limit: usize,
}

impl LocalityWideningBudget {
    pub fn limit(&self) -> usize {
        self.limit
    }
    pub(in crate::live) fn deny_all() -> Self {
        Self { limit: 0 }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LocalityWideningPolicy {
    DenyAll,
    AllowExactMatchWithSinglePeerSlice,
}

impl LocalityWideningPolicy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DenyAll => "deny_all",
            Self::AllowExactMatchWithSinglePeerSlice => "allow_exact_match_with_single_peer_slice",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StreamLoweringCostPosture {
    SingleDetailCurrentStateMember,
    CdcPatchWithProjectedDeltas,
    BoundedMaterializationDeferred,
}

impl StreamLoweringCostPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SingleDetailCurrentStateMember => "single_detail_current_state_member",
            Self::CdcPatchWithProjectedDeltas => "cdc_patch_with_projected_deltas",
            Self::BoundedMaterializationDeferred => "bounded_materialization_deferred",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StreamMemberWidthBudget {
    pub(in crate::live) limit: usize,
}

impl StreamMemberWidthBudget {
    pub fn limit(&self) -> usize {
        self.limit
    }
    pub(in crate::live) fn single_member() -> Self {
        Self { limit: 1 }
    }
    pub(in crate::live) fn cdc_projected_patch() -> Self {
        Self { limit: 2 }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StreamWindowWidthBudget {
    pub(in crate::live) limit: usize,
}

impl StreamWindowWidthBudget {
    pub fn limit(&self) -> usize {
        self.limit
    }
    pub(in crate::live) fn single_window() -> Self {
        Self { limit: 1 }
    }
}

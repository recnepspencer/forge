#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRegionSummumBonumSubcaseKind {
    BoundaryOnlyCoincidentEdgesDoNotAdmitArea,
    OppositeSenseSameAreaOverlapHasStableWinding,
    NestedOverlapIslandsPreserveRegionIdentity,
    MixedBoundaryAndAreaContactDoesNotCollapse,
    BenignLoopOrderVariationPreservesLedgerDigest,
    CheckpointReplayPreservesRegionIdentityAndNames,
    OverlapStormUsesIndexNotPairwiseRediscovery,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionSummumBonumSubcaseRow {
    kind: PlanarBooleanOverlapRegionSummumBonumSubcaseKind,
    detail: String,
}

impl PlanarBooleanOverlapRegionSummumBonumSubcaseKind {
    pub fn spec_name(self) -> &'static str {
        match self {
            Self::BoundaryOnlyCoincidentEdgesDoNotAdmitArea => {
                "boundary_only_coincident_edges_do_not_admit_area"
            }
            Self::OppositeSenseSameAreaOverlapHasStableWinding => {
                "opposite_sense_same_area_overlap_has_stable_winding"
            }
            Self::NestedOverlapIslandsPreserveRegionIdentity => {
                "nested_overlap_islands_preserve_region_identity"
            }
            Self::MixedBoundaryAndAreaContactDoesNotCollapse => {
                "mixed_boundary_and_area_contact_does_not_collapse"
            }
            Self::BenignLoopOrderVariationPreservesLedgerDigest => {
                "benign_loop_order_variation_preserves_ledger_digest"
            }
            Self::CheckpointReplayPreservesRegionIdentityAndNames => {
                "checkpoint_replay_preserves_region_identity_and_names"
            }
            Self::OverlapStormUsesIndexNotPairwiseRediscovery => {
                "overlap_storm_uses_index_not_pairwise_rediscovery"
            }
        }
    }
}

impl PlanarBooleanOverlapRegionSummumBonumSubcaseRow {
    pub fn new(
        kind: PlanarBooleanOverlapRegionSummumBonumSubcaseKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanOverlapRegionSummumBonumSubcaseKind {
        self.kind
    }

    pub fn spec_name(&self) -> &'static str {
        self.kind.spec_name()
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

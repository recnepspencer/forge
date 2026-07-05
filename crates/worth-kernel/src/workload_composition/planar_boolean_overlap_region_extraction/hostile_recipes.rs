#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRegionMetabossSubcase {
    BoundaryOnlyCoincidentEdgesDoNotAdmitArea,
    OppositeSenseSameAreaOverlapHasStableWinding,
    NestedOverlapIslandsPreserveRegionIdentity,
    MixedBoundaryAndAreaContactDoesNotCollapse,
    BenignLoopOrderVariationPreservesLedgerDigest,
    SyntheticOverlapLedgerIsRejected,
    SyntheticReadinessOrMismatchedLoopLedgerIsRejected,
    CheckpointReplayPreservesRegionIdentityAndNames,
    OverlapStormUsesIndexNotPairwiseRediscovery,
}

impl PlanarBooleanOverlapRegionMetabossSubcase {
    pub fn all() -> [Self; 9] {
        [
            Self::BoundaryOnlyCoincidentEdgesDoNotAdmitArea,
            Self::OppositeSenseSameAreaOverlapHasStableWinding,
            Self::NestedOverlapIslandsPreserveRegionIdentity,
            Self::MixedBoundaryAndAreaContactDoesNotCollapse,
            Self::BenignLoopOrderVariationPreservesLedgerDigest,
            Self::SyntheticOverlapLedgerIsRejected,
            Self::SyntheticReadinessOrMismatchedLoopLedgerIsRejected,
            Self::CheckpointReplayPreservesRegionIdentityAndNames,
            Self::OverlapStormUsesIndexNotPairwiseRediscovery,
        ]
    }

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
            Self::SyntheticOverlapLedgerIsRejected => "synthetic_overlap_ledger_is_rejected",
            Self::SyntheticReadinessOrMismatchedLoopLedgerIsRejected => {
                "synthetic_readiness_or_mismatched_loop_ledger_is_rejected"
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

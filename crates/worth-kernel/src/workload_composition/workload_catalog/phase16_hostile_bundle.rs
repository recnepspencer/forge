use super::{WorkloadCatalog, WorkloadCatalogBooleanOperandPairRecipe};

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

    pub fn admitted_operand_pair_recipe(
        self,
        declaration: impl Into<String>,
    ) -> Option<WorkloadCatalogBooleanOperandPairRecipe> {
        let declaration = declaration.into();
        match self {
            Self::SyntheticOverlapLedgerIsRejected
            | Self::SyntheticReadinessOrMismatchedLoopLedgerIsRejected => None,
            Self::BoundaryOnlyCoincidentEdgesDoNotAdmitArea => Some(
                WorkloadCatalog::planar_boolean_boundary_only_coincident_pair()
                    .with_retained_replay_artifacts()
                    .declared(declaration),
            ),
            Self::MixedBoundaryAndAreaContactDoesNotCollapse => Some(
                WorkloadCatalog::planar_boolean_mixed_boundary_area_pair()
                    .with_retained_replay_artifacts()
                    .declared(declaration),
            ),
            Self::OverlapStormUsesIndexNotPairwiseRediscovery => Some(
                WorkloadCatalog::planar_boolean_event_extraction_metaboss_pair()
                    .with_retained_replay_artifacts()
                    .declared(declaration),
            ),
            _ => Some(
                WorkloadCatalog::planar_boolean_mixed_boundary_area_pair()
                    .with_retained_replay_artifacts()
                    .declared(declaration),
            ),
        }
    }
}

pub fn admitted_metaboss_bundle_operand_pair_recipe(
    declaration: impl Into<String>,
) -> WorkloadCatalogBooleanOperandPairRecipe {
    WorkloadCatalog::planar_boolean_mixed_boundary_area_pair()
        .with_retained_replay_artifacts()
        .declared(declaration)
}

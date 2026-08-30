use super::super::delivery::StreamLoweredDeliveryContract;
use super::super::identity::LiveProgressError;
use super::super::locality::LocalityMatchKind;
use super::super::patches::{
    BoundedMaterializationLiveOutcome, BoundedMaterializationPatchKind, DetailLiveOutcome,
    OrderedCollectionLiveOutcome, OrderedCollectionPatchKind,
};
use super::super::promotion::LivePromotionError;
use super::super::refresh::{
    CoalescingDecision, LiveCoalescingError, LiveRefreshError, PatchWidthAssessment,
    PatchWidthResolution, RefreshFallback,
};
#[cfg(test)]
use super::super::RegionScopedLiveError;
use super::LivePolicyCounters;

impl LivePolicyCounters {
    pub fn from_detail_outcome(outcome: &DetailLiveOutcome) -> Self {
        match outcome {
            DetailLiveOutcome::Patch(patch) => Self {
                live_invalidation_event_count: 1,
                live_relevance_match_count: 1,
                live_patch_count: 1,
                live_patch_delivery_count: 1,
                live_patch_field_delta_count: patch.field_deltas().len(),
                live_delivery_width: patch.field_deltas().len(),
                ..Self::default()
            },
            DetailLiveOutcome::Suppressed(_) => Self {
                live_invalidation_event_count: 1,
                live_irrelevant_suppression_count: 1,
                live_suppressed_update_count: 1,
                live_work_avoided_by_irrelevance_count: 1,
                ..Self::default()
            },
            DetailLiveOutcome::Refresh(_) => Self {
                live_invalidation_event_count: 1,
                live_relevance_match_count: 1,
                live_refresh_fallback_count: 1,
                live_refresh_cost_class_count: 1,
                ..Self::default()
            },
        }
    }

    pub fn from_ordered_collection_outcome(outcome: &OrderedCollectionLiveOutcome) -> Self {
        match outcome {
            OrderedCollectionLiveOutcome::Patch(patch) => {
                let mut counters = Self {
                    live_invalidation_event_count: 1,
                    live_relevance_match_count: 1,
                    live_patch_count: 1,
                    live_patch_delivery_count: 1,
                    live_patch_field_delta_count: patch.projected_field_deltas().len(),
                    live_delivery_width: patch.projected_field_deltas().len() + 1,
                    ..Self::default()
                };
                match patch.kind() {
                    OrderedCollectionPatchKind::Membership(_) => {
                        counters.live_collection_membership_change_count = 1;
                    }
                    OrderedCollectionPatchKind::Reordered(_) => {
                        counters.live_collection_reorder_count = 1;
                        counters.live_work_avoided_by_stable_ordering_count = 1;
                    }
                    OrderedCollectionPatchKind::RowUpdated => {}
                }
                counters
            }
            OrderedCollectionLiveOutcome::Suppressed(_) => Self {
                live_invalidation_event_count: 1,
                live_irrelevant_suppression_count: 1,
                live_suppressed_update_count: 1,
                live_work_avoided_by_irrelevance_count: 1,
                ..Self::default()
            },
            OrderedCollectionLiveOutcome::Refresh(_) => Self {
                live_invalidation_event_count: 1,
                live_relevance_match_count: 1,
                live_refresh_fallback_count: 1,
                live_refresh_cost_class_count: 1,
                ..Self::default()
            },
        }
    }

    pub fn from_bounded_materialization_outcome(
        outcome: &BoundedMaterializationLiveOutcome,
    ) -> Self {
        match outcome {
            BoundedMaterializationLiveOutcome::Patch(patch) => {
                let mut counters = Self {
                    live_invalidation_event_count: 1,
                    live_relevance_match_count: 1,
                    live_patch_count: 1,
                    live_patch_delivery_count: 1,
                    live_patch_field_delta_count: patch.projected_field_deltas().len(),
                    live_materialization_patch_count: 1,
                    live_delivery_width: patch.projected_field_deltas().len()
                        + patch.relation_deltas().len()
                        + 1,
                    ..Self::default()
                };
                match patch.kind() {
                    BoundedMaterializationPatchKind::Scope(_) => {
                        counters.live_work_avoided_by_scope_proof_count = 1;
                    }
                    BoundedMaterializationPatchKind::Membership(_) => {
                        counters.live_collection_membership_change_count = 1;
                    }
                    BoundedMaterializationPatchKind::Reordered(_) => {
                        counters.live_collection_reorder_count = 1;
                        counters.live_work_avoided_by_stable_ordering_count = 1;
                    }
                    BoundedMaterializationPatchKind::RowUpdated => {}
                }
                counters
            }
            BoundedMaterializationLiveOutcome::Suppressed(_) => Self {
                live_invalidation_event_count: 1,
                live_irrelevant_suppression_count: 1,
                live_suppressed_update_count: 1,
                live_work_avoided_by_irrelevance_count: 1,
                ..Self::default()
            },
            BoundedMaterializationLiveOutcome::Refresh(_) => Self {
                live_invalidation_event_count: 1,
                live_relevance_match_count: 1,
                live_refresh_fallback_count: 1,
                live_refresh_cost_class_count: 1,
                ..Self::default()
            },
        }
    }

    pub fn from_width_assessment(assessment: &PatchWidthAssessment) -> Self {
        match assessment.resolution() {
            PatchWidthResolution::Deliver => Self {
                live_delivery_width: assessment.measured_width(),
                ..Self::default()
            },
            PatchWidthResolution::Coalesce => Self {
                live_patch_width_overflow_count: 1,
                live_delivery_width: assessment.measured_width(),
                ..Self::default()
            },
            PatchWidthResolution::Refresh(_) => Self {
                live_patch_width_overflow_count: 1,
                live_refresh_fallback_count: 1,
                live_delivery_width: assessment.measured_width(),
                ..Self::default()
            },
            PatchWidthResolution::Reject => Self {
                live_patch_width_overflow_count: 1,
                live_delivery_width: assessment.measured_width(),
                ..Self::default()
            },
        }
    }

    pub fn from_coalescing_decision(decision: &CoalescingDecision) -> Self {
        match decision {
            CoalescingDecision::NotNeeded => Self::default(),
            CoalescingDecision::Admitted { bundle_count } => Self {
                live_coalesced_change_bundle_count: *bundle_count,
                ..Self::default()
            },
        }
    }

    pub fn from_coalescing_error(_error: &LiveCoalescingError) -> Self {
        Self {
            live_coalescing_denial_count: 1,
            ..Self::default()
        }
    }

    pub fn from_refresh_fallback(_fallback: &RefreshFallback) -> Self {
        Self {
            live_refresh_fallback_count: 1,
            live_refresh_cost_class_count: 1,
            ..Self::default()
        }
    }

    pub fn from_refresh_error(_error: &LiveRefreshError) -> Self {
        Self {
            live_refresh_denial_count: 1,
            ..Self::default()
        }
    }

    pub fn from_progress_advance() -> Self {
        Self {
            live_progress_advance_count: 1,
            ..Self::default()
        }
    }

    pub fn from_progress_error(error: &LiveProgressError) -> Self {
        match error {
            LiveProgressError::ChangeSequenceMismatch => Self::default(),
            LiveProgressError::ChangeSequenceGap { .. } => Self {
                live_change_sequence_gap_count: 1,
                ..Self::default()
            },
            LiveProgressError::NonMonotonicOrdinal { .. } => Self {
                live_non_monotonic_sequence_rejection_count: 1,
                ..Self::default()
            },
        }
    }

    pub fn from_promotion_error(error: &LivePromotionError) -> Self {
        match error {
            LivePromotionError::UnsupportedLiveCollectionFamily => Self {
                locality_unsupported_family_rejection_count: 1,
                ..Self::default()
            },
            LivePromotionError::UnsupportedPreflightRoute
            | LivePromotionError::PlanDescriptorMismatch
            | LivePromotionError::BasisPreflight(_)
            | LivePromotionError::Execution(_) => Self {
                live_invalid_promotion_rejection_count: 1,
                ..Self::default()
            },
        }
    }

    pub fn from_unsupported_patch_family() -> Self {
        Self {
            live_unsupported_patch_family_rejection_count: 1,
            ..Self::default()
        }
    }

    pub fn from_locality_match(kind: &LocalityMatchKind) -> Self {
        match kind {
            LocalityMatchKind::InRegionRegionScope => Self {
                locality_region_match_count: 1,
                locality_work_avoided_by_region_narrowing_count: 1,
                locality_work_avoided_vs_broad_control_count: 1,
                ..Self::default()
            },
            LocalityMatchKind::InRegionPartitionScope => Self {
                locality_partition_match_count: 1,
                locality_work_avoided_by_region_narrowing_count: 1,
                locality_work_avoided_vs_broad_control_count: 1,
                ..Self::default()
            },
            LocalityMatchKind::InRegionRegionScopeWithPeerWidening { .. } => Self {
                locality_region_match_count: 1,
                locality_widening_admission_count: 1,
                locality_work_avoided_by_region_narrowing_count: 1,
                locality_work_avoided_vs_broad_control_count: 1,
                ..Self::default()
            },
            LocalityMatchKind::InRegionPartitionScopeWithPeerWidening { .. } => Self {
                locality_partition_match_count: 1,
                locality_widening_admission_count: 1,
                locality_work_avoided_by_region_narrowing_count: 1,
                locality_work_avoided_vs_broad_control_count: 1,
                ..Self::default()
            },
            LocalityMatchKind::OffRegionSuppressed => Self {
                locality_off_region_suppression_count: 1,
                locality_irrelevant_broad_control_count: 1,
                live_suppressed_update_count: 1,
                locality_work_avoided_by_region_narrowing_count: 1,
                locality_work_avoided_vs_broad_control_count: 1,
                ..Self::default()
            },
        }
    }

    #[cfg(test)]
    pub fn from_region_scoped_error(error: &RegionScopedLiveError) -> Self {
        match error {
            RegionScopedLiveError::UnsupportedLocalityFamily => Self {
                locality_unsupported_family_rejection_count: 1,
                ..Self::default()
            },
            RegionScopedLiveError::UnsupportedLocalityPredicate => Self {
                locality_unsupported_predicate_rejection_count: 1,
                ..Self::default()
            },
            RegionScopedLiveError::LocalityBreadthBudgetExceeded { .. } => Self {
                locality_breadth_budget_cross_count: 1,
                ..Self::default()
            },
            RegionScopedLiveError::WideningDenied { .. } => Self {
                locality_widening_denial_count: 1,
                locality_widening_budget_cross_count: 1,
                ..Self::default()
            },
            RegionScopedLiveError::StreamWindowWidthBudgetExceeded { .. } => Self {
                stream_contract_denial_count: 1,
                stream_window_width_budget_cross_count: 1,
                ..Self::default()
            },
            RegionScopedLiveError::StreamMemberWidthBudgetExceeded { .. } => Self {
                stream_contract_denial_count: 1,
                stream_member_width_budget_cross_count: 1,
                ..Self::default()
            },
            RegionScopedLiveError::BridgeSliceIncompatibility => Self {
                locality_bridge_slice_incompatibility_count: 1,
                ..Self::default()
            },
            RegionScopedLiveError::UnsupportedStreamConsumerShape => Self {
                stream_contract_denial_count: 1,
                ..Self::default()
            },
            RegionScopedLiveError::LiveExecution(_) => Self::default(),
        }
    }

    pub fn from_stream_lowered_delivery(contract: &StreamLoweredDeliveryContract) -> Self {
        Self {
            stream_contract_admission_count: 1,
            stream_lowered_delivery_count: 1,
            stream_lowered_delivery_member_count: contract.member_count(),
            stream_lowered_delivery_window_width: contract.window_width(),
            stream_lowered_delivery_width: contract.delivery_width(),
            ..Self::default()
        }
    }

    pub(crate) fn add_replay_change_count(&mut self, replay_change_count: usize) {
        self.live_replay_change_count += replay_change_count;
    }
    #[cfg(test)]
    pub(crate) fn add_locality_replay_change_count(&mut self, replay_change_count: usize) {
        self.locality_replay_change_count += replay_change_count;
    }
}

use worth_store_tiering::ColdPlacementState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ClassifiedColdLaneOutcome {
    Allowed,
    Retry,
    DeniedUnavailable,
    DeniedStale,
    DeniedScope,
    RebindRequired,
}

pub(crate) const fn classify_cold_lane_outcome(
    state: ColdPlacementState,
) -> ClassifiedColdLaneOutcome {
    match state {
        ColdPlacementState::HotAvailable | ColdPlacementState::ColdAvailable => {
            ClassifiedColdLaneOutcome::Allowed
        }
        ColdPlacementState::ColdFetchRequired | ColdPlacementState::ColdFetchInProgress => {
            ClassifiedColdLaneOutcome::Retry
        }
        ColdPlacementState::ColdUnavailable => ClassifiedColdLaneOutcome::DeniedUnavailable,
        ColdPlacementState::ColdStale => ClassifiedColdLaneOutcome::DeniedStale,
        ColdPlacementState::ColdScopeDenied => ClassifiedColdLaneOutcome::DeniedScope,
        ColdPlacementState::ColdRebindRequired => ClassifiedColdLaneOutcome::RebindRequired,
    }
}

macro_rules! map_classified_cold_lane_outcome {
    ($classified:expr, $Allowed:path, $Retry:path, $DeniedUnavailable:path, $DeniedStale:path, $DeniedScope:path, $RebindRequired:path) => {
        match $classified {
            ClassifiedColdLaneOutcome::Allowed => $Allowed,
            ClassifiedColdLaneOutcome::Retry => $Retry,
            ClassifiedColdLaneOutcome::DeniedUnavailable => $DeniedUnavailable,
            ClassifiedColdLaneOutcome::DeniedStale => $DeniedStale,
            ClassifiedColdLaneOutcome::DeniedScope => $DeniedScope,
            ClassifiedColdLaneOutcome::RebindRequired => $RebindRequired,
        }
    };
}

pub(crate) use map_classified_cold_lane_outcome;

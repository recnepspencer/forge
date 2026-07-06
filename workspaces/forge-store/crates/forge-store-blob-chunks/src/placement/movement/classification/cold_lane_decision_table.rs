use forge_store_tiering::S7ColdPlacementState;

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
    state: S7ColdPlacementState,
) -> ClassifiedColdLaneOutcome {
    match state {
        S7ColdPlacementState::HotAvailable | S7ColdPlacementState::ColdAvailable => {
            ClassifiedColdLaneOutcome::Allowed
        }
        S7ColdPlacementState::ColdFetchRequired | S7ColdPlacementState::ColdFetchInProgress => {
            ClassifiedColdLaneOutcome::Retry
        }
        S7ColdPlacementState::ColdUnavailable => ClassifiedColdLaneOutcome::DeniedUnavailable,
        S7ColdPlacementState::ColdStale => ClassifiedColdLaneOutcome::DeniedStale,
        S7ColdPlacementState::ColdScopeDenied => ClassifiedColdLaneOutcome::DeniedScope,
        S7ColdPlacementState::ColdRebindRequired => ClassifiedColdLaneOutcome::RebindRequired,
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
use forge_store_tiering::ColdPlacementState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapsuleChunkAvailabilityPosture {
    HotReady,
    ColdReady,
    ColdFetchRequired,
    ColdUnavailable,
    ColdScopeDenied,
    ColdRebindRequired,
}

pub const fn classify_capsule_chunk_availability(
    cold_state: Option<ColdPlacementState>,
) -> CapsuleChunkAvailabilityPosture {
    match cold_state {
        None | Some(ColdPlacementState::HotAvailable) => CapsuleChunkAvailabilityPosture::HotReady,
        Some(ColdPlacementState::ColdAvailable) => CapsuleChunkAvailabilityPosture::ColdReady,
        Some(ColdPlacementState::ColdFetchRequired)
        | Some(ColdPlacementState::ColdFetchInProgress) => {
            CapsuleChunkAvailabilityPosture::ColdFetchRequired
        }
        Some(ColdPlacementState::ColdUnavailable) | Some(ColdPlacementState::ColdStale) => {
            CapsuleChunkAvailabilityPosture::ColdUnavailable
        }
        Some(ColdPlacementState::ColdScopeDenied) => {
            CapsuleChunkAvailabilityPosture::ColdScopeDenied
        }
        Some(ColdPlacementState::ColdRebindRequired) => {
            CapsuleChunkAvailabilityPosture::ColdRebindRequired
        }
    }
}

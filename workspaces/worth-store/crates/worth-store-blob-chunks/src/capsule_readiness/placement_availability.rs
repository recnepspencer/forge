use worth_store_tiering::ColdPlacementState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCapsulePlacementAvailability {
    HotReady,
    ColdReady,
    ColdFetchRequired,
    ColdUnavailable,
    ColdScopeDenied,
    ColdRebindRequired,
}

pub const fn classify_blob_capsule_placement_availability(
    cold_state: Option<ColdPlacementState>,
) -> BlobCapsulePlacementAvailability {
    match cold_state {
        None | Some(ColdPlacementState::HotAvailable) => BlobCapsulePlacementAvailability::HotReady,
        Some(ColdPlacementState::ColdAvailable) => BlobCapsulePlacementAvailability::ColdReady,
        Some(ColdPlacementState::ColdFetchRequired)
        | Some(ColdPlacementState::ColdFetchInProgress) => {
            BlobCapsulePlacementAvailability::ColdFetchRequired
        }
        Some(ColdPlacementState::ColdUnavailable) | Some(ColdPlacementState::ColdStale) => {
            BlobCapsulePlacementAvailability::ColdUnavailable
        }
        Some(ColdPlacementState::ColdScopeDenied) => {
            BlobCapsulePlacementAvailability::ColdScopeDenied
        }
        Some(ColdPlacementState::ColdRebindRequired) => {
            BlobCapsulePlacementAvailability::ColdRebindRequired
        }
    }
}

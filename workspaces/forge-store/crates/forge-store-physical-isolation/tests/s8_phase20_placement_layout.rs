use forge_store_physical_isolation::{
    MovablePhysicalRefKind, TierMovementReadInterlockPlan, UnsupportedTierMovementClaim,
    UnsupportedTierMovementRequest,
};

#[test]
fn unsupported_hardware_placement_claim_is_denied_by_owner_api() {
    let denial = TierMovementReadInterlockPlan::reject_unsupported_tier_movement(
        UnsupportedTierMovementRequest::new(
            MovablePhysicalRefKind::FutureChunk,
            UnsupportedTierMovementClaim::HardwareMediaPlacement,
        ),
    )
    .expect_err("projection shortcut must stay denied");

    assert_eq!(
        denial,
        forge_store_physical_isolation::TierMovementStabilityDenial::UnsupportedTierMovement
    );
}

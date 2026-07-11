use forge_store_layout_indexes::layout_strategy_admission::phase20_placement_rule;
use forge_store_physical_isolation::{
    AdmittedPlacementLayoutFamily, MovablePhysicalRefKind, MovableStabilityLayoutFamilyHome,
    PlacementResidencyMapState, UnsupportedTierMovementClaim, UnsupportedTierMovementRequest,
};

#[test]
fn phase20_placement_rule_opens_public_family_with_explicit_absence_and_denies_projection_shortcuts(
) {
    let rule = phase20_placement_rule().expect("phase-20 placement rule");
    let admission = MovableStabilityLayoutFamilyHome::s8()
        .admit(&rule)
        .expect("placement layout admission");
    let family = AdmittedPlacementLayoutFamily::new(admission);
    let report = family.placement_residency_map();
    let denial = family
        .reject_projection_as_data_authority(UnsupportedTierMovementRequest::new(
            MovablePhysicalRefKind::FutureChunk,
            UnsupportedTierMovementClaim::HardwareMediaPlacement,
        ))
        .expect_err("projection shortcut must stay denied");

    assert_eq!(
        report.placement_map_state(),
        PlacementResidencyMapState::UnmaterializedInPhase20
    );
    assert_eq!(report.counters().stability_admissions(), 1);
    assert_eq!(report.counters().chunk_placeholders(), 1);
    assert_eq!(
        denial,
        forge_store_physical_isolation::TierMovementStabilityDenial::UnsupportedTierMovement
    );
}

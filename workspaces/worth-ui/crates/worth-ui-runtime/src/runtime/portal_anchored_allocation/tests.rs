use super::*;

#[test]
fn rect_movement_does_not_participate_in_anchor_identity() {
    let report = crate::evidence::measurement::projection::fact_test_support::capability_report(77);
    let generation = worth_ui_inspection::UiEvidenceAuthorityGeneration::new(17);
    let before =
        crate::evidence::measurement::projection::fact_test_support::host_result_portal_anchor_at(
            1,
            44,
            [1.0, 2.0, 3.0, 4.0],
            &report,
            generation,
        );
    let after =
        crate::evidence::measurement::projection::fact_test_support::host_result_portal_anchor_at(
            2,
            44,
            [101.0, 202.0, 30.0, 40.0],
            &report,
            generation,
        );
    assert_eq!(
        UiPortalAnchorIdentity::from_measurement_result(&before),
        UiPortalAnchorIdentity::from_measurement_result(&after)
    );
}

#[test]
fn target_or_coordinate_space_change_replaces_anchor_identity() {
    let original = UiPortalAnchorIdentity::from_parts_for_test(
        worth_ui_host_contract::UiPortalAnchorTargetIdentity::new(44),
        crate::evidence::UiMeasurementCoordinateSpace::PortalLayer,
    );
    let target_changed = UiPortalAnchorIdentity::from_parts_for_test(
        worth_ui_host_contract::UiPortalAnchorTargetIdentity::new(45),
        crate::evidence::UiMeasurementCoordinateSpace::PortalLayer,
    );
    let coordinate_changed = UiPortalAnchorIdentity::from_parts_for_test(
        worth_ui_host_contract::UiPortalAnchorTargetIdentity::new(44),
        crate::evidence::UiMeasurementCoordinateSpace::Viewport,
    );
    assert_ne!(original.identity_digest(), target_changed.identity_digest());
    assert_ne!(
        original.identity_digest(),
        coordinate_changed.identity_digest()
    );
}

#[test]
fn anchor_identity_is_stable_while_observation_generation_remains_distinct() {
    let report = crate::evidence::measurement::projection::fact_test_support::capability_report(77);
    let first =
        crate::evidence::measurement::projection::fact_test_support::host_result_portal_anchor_at(
            1,
            44,
            [1.0, 2.0, 3.0, 4.0],
            &report,
            worth_ui_inspection::UiEvidenceAuthorityGeneration::new(17),
        );
    let later =
        crate::evidence::measurement::projection::fact_test_support::host_result_portal_anchor_at(
            1,
            44,
            [1.0, 2.0, 3.0, 4.0],
            &report,
            worth_ui_inspection::UiEvidenceAuthorityGeneration::new(18),
        );
    assert_eq!(
        UiPortalAnchorIdentity::from_measurement_result(&first),
        UiPortalAnchorIdentity::from_measurement_result(&later)
    );
    assert_ne!(first.authority_witness(), later.authority_witness());
}

#[test]
fn identity_transition_names_the_frozen_portal_rule() {
    let original = UiPortalAnchorIdentity::from_parts_for_test(
        worth_ui_host_contract::UiPortalAnchorTargetIdentity::new(44),
        crate::evidence::UiMeasurementCoordinateSpace::PortalLayer,
    );
    let moved = UiPortalAnchorIdentity::from_parts_for_test(
        worth_ui_host_contract::UiPortalAnchorTargetIdentity::new(44),
        crate::evidence::UiMeasurementCoordinateSpace::PortalLayer,
    );
    assert_eq!(
        UiPortalAnchorIdentityTransition::classify(original, moved),
        UiPortalAnchorIdentityTransition::Preserved { identity: original }
    );
    assert!(matches!(
        UiPortalAnchorIdentityTransition::classify(
            original,
            UiPortalAnchorIdentity::from_parts_for_test(
                worth_ui_host_contract::UiPortalAnchorTargetIdentity::new(45),
                crate::evidence::UiMeasurementCoordinateSpace::PortalLayer,
            ),
        ),
        UiPortalAnchorIdentityTransition::TargetReplaced { .. }
    ));
    assert!(matches!(
        UiPortalAnchorIdentityTransition::classify(
            original,
            UiPortalAnchorIdentity::from_parts_for_test(
                worth_ui_host_contract::UiPortalAnchorTargetIdentity::new(44),
                crate::evidence::UiMeasurementCoordinateSpace::Viewport,
            ),
        ),
        UiPortalAnchorIdentityTransition::CoordinateSpaceReplaced { .. }
    ));
}

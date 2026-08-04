use crate::live::*;
#[test]
fn replay_live_sequence_emits_step_bundles_and_advances_plan() {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let steps = vec![
        LiveReplayStepInput::new(
            BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
                "identity",
                "id",
                Some("user-1"),
                Some("user-2"),
            )),
            LiveChangeOrdinal::from_value(1),
            crate::harness::fixtures::resolved_bases::runtime_basis(
                &crate::harness::fixtures::validated_bundles::runtime_detail_bundle(),
                &crate::harness::fixtures::resolved_bases::alternate_snapshot_identity(),
            ),
        ),
        LiveReplayStepInput::new(
            BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
                "identity",
                "id",
                Some("user-2"),
                Some("user-3"),
            )),
            LiveChangeOrdinal::from_value(2),
            crate::harness::fixtures::resolved_bases::runtime_basis(
                &crate::harness::fixtures::validated_bundles::runtime_detail_bundle(),
                &crate::harness::fixtures::resolved_bases::relational_snapshot_identity(3, 1),
            ),
        ),
    ];

    let replay = replay_live_sequence(&live, &steps).expect("replay sequence should succeed");

    assert_eq!(replay.bundles().len(), 2);
    assert_eq!(
        replay.final_plan().progress_basis().last_ordinal().value(),
        2
    );
    assert_eq!(
        replay
            .final_plan()
            .progress_basis()
            .replay_digest()
            .as_str(),
        replay.bundles()[1].replay_digest()
    );
}

#[test]
fn standard_named_adapter_helpers_build_expected_lanes() {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let patch_change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "identity",
        "id",
        Some("user-1"),
        Some("user-2"),
    ));

    let patch_lane = MilestoneFiveLiveAdapter::detail_patch_lane(&live, &patch_change)
        .expect("detail patch lane should build");
    let refresh_lane = MilestoneFiveLiveAdapter::forbidden_refresh_rejection_lane(
        &live,
        RefreshAdmissionClass::WidthOverflow,
    )
    .expect("detail family should reject refresh admission");

    assert_eq!(patch_lane.lane_name(), "detail-live-patch-parity");
    assert_eq!(refresh_lane.lane_name(), "forbidden-refresh-escape-hatch");
}

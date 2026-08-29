#[test]
#[ignore = "closure-stress: milestone 3.15 RS-10 full service and mounted scale world"]
fn runtime_service_scale_has_named_local_work_and_exact_zero_residue() {
    let mounted_nodes = crate::host_platform::verify_4096_mounted_node_world();
    let evidence = worth_ui_test_support::runtime_service_scale_evidence();

    assert_eq!(mounted_nodes, 4_096);
    assert_eq!(evidence.service_neighborhoods(), 64);
    assert_eq!(evidence.commands(), 4_096);
    assert_eq!(evidence.focus_participants(), 128);
    assert_eq!(evidence.selection_keys(), 1_024);
    assert_eq!(evidence.scroll_owners(), 8);
    assert_eq!(evidence.portal_layers(), 4);
    assert_eq!(evidence.active_motion_tracks(), 64);

    assert_eq!(evidence.portal_neighborhoods_visited(), 4);
    assert_eq!(evidence.focus_participants_visited(), 1);
    assert_eq!(evidence.motion_tracks_sampled(), 64);
    assert_eq!(evidence.inactive_motion_tracks_sampled(), 0);
    assert_eq!(evidence.scroll_chain_depth_visited(), 8);
    assert_eq!(evidence.selection_keys_visited(), 1);
    assert_eq!(evidence.command_candidates_resolved(), 1);
    assert_eq!(evidence.proposal_requirements_visited(), 384);
    assert_eq!(evidence.unrelated_neighborhoods_touched(), 0);
    assert!(evidence.terminal_resources_zero());
}

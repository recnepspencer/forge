use worth_ui_certification::topology::audit_lane_extension_authority;

#[test]
fn spatial_and_realtime_hooks_remain_sealed_off_the_frame_path() {
    let violations = audit_lane_extension_authority(super::workspace_source_inventory());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

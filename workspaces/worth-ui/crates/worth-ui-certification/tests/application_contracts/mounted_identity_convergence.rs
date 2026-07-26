use worth_ui_runtime::facade::mounted::{
    UiHostSurfacePresentationMode, UiSurfaceBindingCoordinatePosture, UiSurfaceBindingProfile,
};
use worth_ui_test_support::WorthUiMountedIdentityCertificationExt;

use super::mounted_application_lifecycle::known_empty_surface_world::{active_session, first_node};
use super::mounted_identity_lifecycle::incarnation;

#[test]
fn equivalent_bases_ignore_visible_order_and_surface_geometry_posture() {
    let mut left = active_session();
    let mut right = active_session();
    let left_surface = surface_with_geometry(
        &mut left,
        UiSurfaceBindingCoordinatePosture::LogicalPoints,
        1_000,
    );
    let right_surface = surface_with_geometry(
        &mut right,
        UiSurfaceBindingCoordinatePosture::PhysicalPixels,
        2_000,
    );
    let left_node = first_node(&left);
    let right_node = first_node(&right);

    let left_first = left.mount_instance(left_node, left_surface).unwrap();
    let left_second = left.mount_instance(left_node, left_surface).unwrap();
    let right_first = right.mount_instance(right_node, right_surface).unwrap();
    let right_second = right.mount_instance(right_node, right_surface).unwrap();
    let left_basis = repeated_basis(&left, left_first);
    let right_basis = repeated_basis(&right, right_first);
    assert_eq!(left_basis, right_basis);
    let left_incarnations = [
        incarnation(&left, left_first),
        incarnation(&left, left_second),
    ];
    let right_incarnations = [
        incarnation(&right, right_first),
        incarnation(&right, right_second),
    ];

    left.reorder_mounted_instances(&[left_second, left_first])
        .unwrap();
    right
        .reorder_mounted_instances(&[right_first, right_second])
        .unwrap();
    left.advance_mounted_identity_frame().unwrap();
    right.advance_mounted_identity_frame().unwrap();

    assert_eq!(incarnation(&left, left_first), left_incarnations[0]);
    assert_eq!(incarnation(&left, left_second), left_incarnations[1]);
    assert_eq!(incarnation(&right, right_first), right_incarnations[0]);
    assert_eq!(incarnation(&right, right_second), right_incarnations[1]);
    assert_eq!(
        left.inspect_mounted_identity().frame_receipts().len(),
        right.inspect_mounted_identity().frame_receipts().len()
    );
}

fn repeated_basis(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    identity: worth_ui_runtime::facade::mounted::UiMountedInstanceIdentity,
) -> worth_ui::facade::graph::UiRepeatedInstanceBasis {
    session
        .inspect_mounted_identity()
        .mounted_instances()
        .iter()
        .find(|entry| entry.identity() == identity)
        .unwrap()
        .basis()
        .repeated_instance_basis()
        .clone()
}

fn surface_with_geometry(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    coordinate_posture: UiSurfaceBindingCoordinatePosture,
    device_scale_milli: u32,
) -> worth_ui_runtime::facade::mounted::UiSemanticSurfaceIdentity {
    let surface = session.create_semantic_surface().unwrap();
    let profile = UiSurfaceBindingProfile::new(device_scale_milli, coordinate_posture, 1).unwrap();
    session
        .register_host_surface(surface, UiHostSurfacePresentationMode::RecordOnly, profile)
        .unwrap();
    surface
}

use worth_ui_host_contract::{
    UiHostSurfacePresentationMode, UiMountedEffectFamily, UiMountedFrameIdentity,
};

use super::{rect_spec, MountedPresentationWorld, UiMountedPresentationState};

#[test]
fn native_initial_and_reconstruction_require_physical_paint() {
    let world = MountedPresentationWorld::new();
    let initial_projection = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [rect_spec(world.first_instance, 0.0)],
    );
    let initial_state =
        UiMountedPresentationState::from_projection(&initial_projection, world.requirement, None);
    let lease = super::super::UiMountedPresentationLeaseGate::default()
        .claim()
        .unwrap();
    let initial = initial_state.issue_initial(&lease, &initial_projection);
    assert!(initial_state
        .expected_completion_effects(None, &initial, UiHostSurfacePresentationMode::NativeDisplay,)
        .contains(&UiMountedEffectFamily::NativePaint));

    let successor_projection = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [rect_spec(world.first_instance, 0.0)],
    );
    let successor_state = UiMountedPresentationState::from_projection(
        &successor_projection,
        world.requirement,
        Some(initial_projection.frame()),
    );
    let reconstruction = successor_state.issue_reconstruction(
        &lease,
        &successor_projection,
        initial_projection.frame(),
    );
    assert!(successor_state
        .expected_completion_effects(
            Some(&initial_state),
            &reconstruction,
            UiHostSurfacePresentationMode::NativeDisplay,
        )
        .contains(&UiMountedEffectFamily::NativePaint));
    assert_eq!(
        successor_state.expected_completion_effects(
            Some(&initial_state),
            &reconstruction,
            UiHostSurfacePresentationMode::RecordOnly,
        ),
        vec![UiMountedEffectFamily::RecordedProjection]
    );
}

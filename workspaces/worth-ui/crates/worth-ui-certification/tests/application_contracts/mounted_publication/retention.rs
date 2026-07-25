use worth_ui::facade::mounted::{
    UiHostSurfacePresentationMode, UiMountWorkClass, UiMountedFrameOutcome,
    UiMountedFrameRetentionBudget, UiMountedFrameRetentionBudgetInput,
    UiMountedFrameRetentionDenial, UiMountedRetentionClass, UiMountedRetentionClassBudget,
    UiPresentationDeadline,
};

use crate::mounted_application_lifecycle::in_flight_presentation_world::prepared;
use crate::mounted_application_lifecycle::known_empty_surface_world::{
    first_node, mounted_application_with_host_and_retention_budget, profile,
};
use crate::mounted_host_protocol::scripted_host::ScriptedPresentationHost;

#[test]
fn retention_capacity_denies_the_frame_before_any_adapter_effect() {
    let host = ScriptedPresentationHost::default();
    let one_byte = UiMountedRetentionClassBudget::new(1, 1);
    let budget = UiMountedFrameRetentionBudget::new(UiMountedFrameRetentionBudgetInput {
        current: one_byte,
        in_flight: one_byte,
        observation_basis: UiMountedRetentionClassBudget::new(8, 1024),
        predecessor_inspection: UiMountedRetentionClassBudget::new(8, 1024),
        diagnostic: UiMountedRetentionClassBudget::new(0, 0),
        future_snapshot: UiMountedRetentionClassBudget::new(0, 0),
        expired_identity_limit: 64,
    });
    let mut session = mounted_application_with_host_and_retention_budget(
        "mounted-retention-pre-effect-denial",
        host.clone(),
        budget,
    )
    .launch()
    .expect("the real filesystem-authored application launches");
    let node = first_node(&session);
    let surface = session.create_semantic_surface().unwrap();
    session
        .register_host_surface(
            surface,
            UiHostSurfacePresentationMode::RecordOnly,
            profile(1),
        )
        .unwrap();
    session.mount_instance(node, surface).unwrap();
    let frame = prepared(&mut session);

    let rejection =
        match session.present_prepared_mounted_frame(frame, UiPresentationDeadline::at_tick(10), 0)
        {
            UiMountedFrameOutcome::RetentionDenied(rejection) => rejection,
            _ => panic!("a one-byte current-frame budget must deny retention"),
        };

    assert!(matches!(
        rejection.denial(),
        UiMountedFrameRetentionDenial::CapacityExceeded {
            class: UiMountedRetentionClass::Current,
            required_frames: 1,
            required_structural_bytes,
            budget: class_budget,
        } if required_structural_bytes > 1 && class_budget == one_byte
    ));
    assert_eq!(
        host.presentation_calls(),
        0,
        "retention denial must precede the first adapter call"
    );
    assert!(session.current_mounted_publication().is_none());
    assert_eq!(
        rejection.frame().manifest().surfaces().len(),
        1,
        "denial preserves the exact prepared frame for diagnosis or retry"
    );
    assert_eq!(
        rejection.frame().cost_report().work_class(),
        UiMountWorkClass::InitialMount,
        "the rejection retains owner-emitted preparation cost"
    );
}

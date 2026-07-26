use super::*;
use worth_ui::facade::observation_report::WorthUiHostObservationSessionExt;

#[test]
fn each_frame_evidence_byte_reserve_denies_at_its_real_boundary() {
    assert_in_flight_byte_reserve();
    assert_predecessor_byte_reserve();
    assert_observation_basis_byte_reserve();
}

fn assert_in_flight_byte_reserve() {
    let one_byte = UiMountedRetentionClassBudget::new(1, 1);
    let (mut session, host, _, _) = retention_world(
        "mounted-in-flight-byte-reserve",
        retention_budget(large_budget(), one_byte, large_budget(), large_budget()),
    );
    let candidate = prepared(&mut session);
    let rejection = match session.present_prepared_mounted_frame(
        candidate,
        UiPresentationDeadline::at_tick(10),
        0,
    ) {
        UiMountedFrameOutcome::RetentionDenied(rejection) => rejection,
        _ => panic!("the in-flight byte reserve must deny before presentation"),
    };
    assert_capacity_denial(
        rejection.denial(),
        UiMountedRetentionClass::InFlight,
        one_byte,
    );
    assert_eq!(host.presentation_calls(), 0);
}

fn assert_predecessor_byte_reserve() {
    let one_byte = UiMountedRetentionClassBudget::new(8, 1);
    let (mut session, host, binding, instance) = retention_world(
        "mounted-predecessor-byte-reserve",
        retention_budget(large_budget(), large_budget(), large_budget(), one_byte),
    );
    let current = publish(&mut session, &host, instance);
    retain_keyboard_report(&mut session, binding, current, 1);
    let candidate = prepared(&mut session);
    let rejection = match session.present_prepared_mounted_frame(
        candidate,
        UiPresentationDeadline::at_tick(20),
        1,
    ) {
        UiMountedFrameOutcome::RetentionDenied(rejection) => rejection,
        _ => panic!("the predecessor byte reserve must deny before successor effects"),
    };
    assert_capacity_denial(
        rejection.denial(),
        UiMountedRetentionClass::PredecessorInspection,
        one_byte,
    );
    assert_eq!(host.presentation_calls(), 1);
}

fn assert_observation_basis_byte_reserve() {
    let one_byte = UiMountedRetentionClassBudget::new(1, 1);
    let (mut session, host, binding, instance) = retention_world(
        "mounted-observation-basis-byte-reserve",
        retention_budget(large_budget(), large_budget(), one_byte, large_budget()),
    );
    let basis = publish(&mut session, &host, instance);
    let raw = batch(
        source(&session, binding, &basis),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report(
            1,
            UiHostObservationPayload::Tick { tick: 1 },
            &basis,
        )],
    );
    assert!(matches!(
        session.validate_host_observation_batch(raw),
        UiHostObservationReportOutcome::Denied(
            UiHostObservationReportDenial::ObservationBasisCapacityExceeded {
                required_leases: 1,
                required_structural_bytes,
                budget,
            }
        ) if required_structural_bytes > 1 && budget == one_byte
    ));
    assert_eq!(session.retained_host_observation_report_count(), 0);
}

fn assert_capacity_denial(
    denial: UiMountedFrameRetentionDenial,
    class: UiMountedRetentionClass,
    budget: UiMountedRetentionClassBudget,
) {
    assert!(matches!(
        denial,
        UiMountedFrameRetentionDenial::CapacityExceeded {
            class: denied_class,
            required_frames: 1,
            required_structural_bytes,
            budget: denied_budget,
        } if denied_class == class && required_structural_bytes > 1 && denied_budget == budget
    ));
}

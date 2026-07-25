use worth_ui::facade::mounted::{
    UiHostSurfacePresentationMode, UiMountWorkClass, UiMountedFrameOutcome,
    UiMountedFrameRetentionBudget, UiMountedFrameRetentionBudgetInput,
    UiMountedFrameRetentionDenial, UiMountedInspectionOmission, UiMountedInspectionReceipt,
    UiMountedInspectionRelation, UiMountedInspectionRequest, UiMountedInstanceIdentity,
    UiMountedRetentionClass, UiMountedRetentionClassBudget, UiPresentationDeadline,
    UiSurfaceBindingGeneration,
};
use worth_ui::facade::observation_report::{
    UiHostObservationLoss, UiHostObservationPayload, UiHostObservationReportDenial,
    UiHostObservationReportOutcome,
};
use worth_ui_test_support::WorthUiMountedPublicationCertificationExt;

use crate::host_observation_fixture::{batch, report, source};
use crate::mounted_application_lifecycle::in_flight_presentation_world::prepared;
use crate::mounted_application_lifecycle::known_empty_surface_world::{
    first_node, mounted_application_with_host_and_retention_budget, profile,
};
use crate::mounted_application_lifecycle::published_mounted_world::publish;
use crate::mounted_host_protocol::scripted_host::presented_completion;
use crate::mounted_host_protocol::scripted_host::ScriptedPresentationHost;

#[path = "retention/byte_reserves.rs"]
mod byte_reserves;
#[path = "retention/diagnostic_pressure.rs"]
mod diagnostic_pressure;

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
    let rejected_cost = UiMountedFrameOutcome::RetentionDenied(rejection)
        .cost_report()
        .expect("retention denial owns rejected preparation cost");
    assert_eq!(
        rejected_cost.work_class(),
        UiMountWorkClass::RejectedPreparation
    );
    assert_eq!(rejected_cost.named().rejected(), 1);
}

#[test]
fn compact_inspection_lease_protects_its_frame_and_releases_on_drop() {
    let (mut session, host, _, instance) = retention_world(
        "mounted-retention-inspection-priority",
        one_predecessor_budget(),
    );
    let first = publish(&mut session, &host, instance);
    let inspection = match session
        .inspect_mounted_frame(UiMountedInspectionRequest::current().for_instance(instance))
    {
        UiMountedInspectionReceipt::Available(inspection) => inspection,
        other => panic!("current frame must have compact inspection evidence: {other:?}"),
    };
    assert_eq!(inspection.frame(), first.frame);
    assert_eq!(inspection.relation(), UiMountedInspectionRelation::Current);
    assert_eq!(inspection.presented_binding_count(), 1);
    assert_eq!(inspection.mounted_instance_count(), 1);
    assert_eq!(inspection.selected_node_receipt(), Some(first.receipt));
    assert_eq!(inspection.frame_index_probes(), 1);
    assert!((1..=2).contains(&inspection.instance_index_probes()));
    assert!(inspection.retained_structural_bytes() > 0);

    let second = publish(&mut session, &host, instance);
    publish(&mut session, &host, instance);
    let retained = session
        .mounted_retention_report()
        .class(UiMountedRetentionClass::PredecessorInspection)
        .clone();
    assert_eq!(retained.retained_items(), 1);
    assert_eq!(retained.active_leases(), 1);
    assert!(retained.lease_charged_structural_bytes() > 0);
    assert_expired(&session, second.frame);

    drop(inspection);
    publish(&mut session, &host, instance);
    assert_expired(&session, first.frame);
    let report = session.mounted_retention_report();
    let released = report.class(UiMountedRetentionClass::PredecessorInspection);
    assert_eq!(released.active_leases(), 0);
    assert_eq!(released.lease_charged_structural_bytes(), 0);
}

#[test]
fn inspection_cannot_open_a_late_pin_after_successor_admission() {
    let (mut session, host, _, instance) =
        retention_world("mounted-retention-late-pin", one_predecessor_budget());
    publish(&mut session, &host, instance);
    let successor = prepared(&mut session);
    host.push_in_flight(
        vec![presented_completion()],
        worth_ui::facade::mounted::UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun,
    );
    let in_flight = match session.present_prepared_mounted_frame(
        successor,
        UiPresentationDeadline::at_tick(1_000),
        0,
    ) {
        UiMountedFrameOutcome::InFlight(in_flight) => in_flight,
        _ => panic!("scripted successor must remain in flight"),
    };

    assert!(matches!(
        session.inspect_mounted_frame(UiMountedInspectionRequest::current()),
        UiMountedInspectionReceipt::Omitted(UiMountedInspectionOmission::FrameTransitionInFlight)
    ));
    assert!(matches!(
        session.complete_mounted_presentation(in_flight, 1),
        UiMountedFrameOutcome::Published(_)
    ));
}

#[test]
fn observation_pins_deny_a_successor_before_adapter_effects() {
    let (mut session, host, binding, instance) = retention_world(
        "mounted-retention-observation-pins",
        one_predecessor_budget(),
    );
    let first = publish(&mut session, &host, instance);
    retain_keyboard_report(&mut session, binding, first, 1);
    let second = publish(&mut session, &host, instance);
    retain_keyboard_report(&mut session, binding, second, 2);
    let successor = prepared(&mut session);

    let rejection = match session.present_prepared_mounted_frame(
        successor,
        UiPresentationDeadline::at_tick(1_000),
        0,
    ) {
        UiMountedFrameOutcome::RetentionDenied(rejection) => rejection,
        _ => panic!("two pinned predecessors must deny the successor"),
    };
    assert!(matches!(
        rejection.denial(),
        UiMountedFrameRetentionDenial::CapacityExceeded {
            class: UiMountedRetentionClass::PredecessorInspection,
            required_frames: 2,
            ..
        }
    ));
    assert_eq!(host.presentation_calls(), 2);
    let report = session.mounted_retention_report();
    assert_eq!(
        report
            .class(UiMountedRetentionClass::ObservationBasis)
            .retained_items(),
        2
    );
    assert_eq!(
        report
            .class(UiMountedRetentionClass::ObservationBasis)
            .active_leases(),
        2
    );
    assert_eq!(
        report
            .class(UiMountedRetentionClass::PredecessorInspection)
            .retained_items(),
        1
    );
}

fn retention_world(
    label: &str,
    budget: UiMountedFrameRetentionBudget,
) -> (
    worth_ui::facade::app::WorthUiActiveApplicationSession,
    ScriptedPresentationHost,
    UiSurfaceBindingGeneration,
    UiMountedInstanceIdentity,
) {
    let host = ScriptedPresentationHost::default();
    let mut session =
        mounted_application_with_host_and_retention_budget(label, host.clone(), budget)
            .launch()
            .expect("the real filesystem-authored application launches");
    let node = first_node(&session);
    let surface = session.create_semantic_surface().unwrap();
    let binding = session
        .register_host_surface(
            surface,
            UiHostSurfacePresentationMode::RecordOnly,
            profile(1),
        )
        .unwrap()
        .binding_generation();
    session.mount_instance(node, surface).unwrap();
    let instance = session.inspect_mounted_identity().mounted_instances()[0].identity();
    (session, host, binding, instance)
}

fn one_predecessor_budget() -> UiMountedFrameRetentionBudget {
    retention_budget(
        large_budget(),
        large_budget(),
        UiMountedRetentionClassBudget::new(8, 128 * 1024 * 1024),
        UiMountedRetentionClassBudget::new(1, 128 * 1024 * 1024),
    )
}

fn retention_budget(
    current: UiMountedRetentionClassBudget,
    in_flight: UiMountedRetentionClassBudget,
    observation_basis: UiMountedRetentionClassBudget,
    predecessor_inspection: UiMountedRetentionClassBudget,
) -> UiMountedFrameRetentionBudget {
    UiMountedFrameRetentionBudget::new(UiMountedFrameRetentionBudgetInput {
        current,
        in_flight,
        observation_basis,
        predecessor_inspection,
        diagnostic: UiMountedRetentionClassBudget::new(0, 0),
        future_snapshot: UiMountedRetentionClassBudget::new(0, 0),
        expired_identity_limit: 64,
    })
}

fn large_budget() -> UiMountedRetentionClassBudget {
    UiMountedRetentionClassBudget::new(8, 128 * 1024 * 1024)
}

fn retain_keyboard_report(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    binding: UiSurfaceBindingGeneration,
    basis: crate::mounted_application_lifecycle::published_mounted_world::PresentedObservationBasis,
    sequence: u64,
) {
    let raw = batch(
        source(session, binding, &basis),
        (sequence, sequence),
        UiHostObservationLoss::Complete,
        vec![report(
            sequence,
            UiHostObservationPayload::Keyboard {
                physical_key: u32::try_from(sequence).unwrap(),
                pressed: true,
                repeat: false,
            },
            &basis,
        )],
    );
    assert!(matches!(
        session.validate_host_observation_batch(raw),
        UiHostObservationReportOutcome::Validated(_)
    ));
}

fn assert_expired(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    frame: worth_ui::facade::mounted::UiMountedFrameIdentity,
) {
    assert!(matches!(
        session.inspect_mounted_frame(UiMountedInspectionRequest::frame(frame)),
        UiMountedInspectionReceipt::Omitted(UiMountedInspectionOmission::ExpiredFrame { .. })
    ));
}

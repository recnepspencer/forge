use worth_ui::facade::measurement_exchange::WorthUiHostMeasurementSessionExt;
use worth_ui::facade::measurement_exchange::{
    UiHostMeasurementCompletion, UiHostMeasurementOutcome,
};
use worth_ui::facade::observation_report::WorthUiHostObservationSessionExt;
use worth_ui::facade::observation_report::{
    UiHostObservationDisposition, UiHostObservationFamily, UiHostObservationFrameRelation,
    UiHostObservationLoss, UiHostObservationPayload, UiHostObservationReportDenial,
    UiHostObservationReportOutcome, UiHostObservationSequence, UiHostObservationSequenceRange,
};
use worth_ui_runtime::facade::mounted::{
    UiMountedFrameOutcome, UiMountedRetentionClass, UiPresentationDeadline,
};
use worth_ui_test_support::WorthUiMountedPublicationCertificationExt;

use super::host_measurement_fixture::{begin_viewport, measurement_host, viewport_observation};
use super::host_observation_fixture::{batch, pointer, report, source};
use super::mounted_application_lifecycle::in_flight_presentation_world::prepared;
use super::mounted_application_lifecycle::published_mounted_world::{
    multi_surface_observation_world, published_observation_world,
    published_observation_world_with_host,
};

mod arrival_schedule;
mod observation_basis_ownership;
mod quarantine_capacity;

#[test]
fn one_surface_partition_cannot_evict_another_surfaces_lossless_input() {
    let mut isolated = multi_surface_observation_world("observation-partition-isolation", 2);
    let (left_binding, left_basis) = isolated.surfaces[0];
    let (right_binding, right_basis) = isolated.surfaces[1];
    for sequence in 1..=64 {
        let raw = batch(
            source(&isolated.session, left_binding, &left_basis),
            (sequence, sequence),
            UiHostObservationLoss::Complete,
            vec![report(sequence, keyboard(sequence), &left_basis)],
        );
        assert!(matches!(
            isolated.session.validate_host_observation_batch(raw),
            UiHostObservationReportOutcome::Validated(_)
        ));
    }
    assert_eq!(
        isolated
            .session
            .mounted_retention_report()
            .class(UiMountedRetentionClass::ObservationBasis)
            .active_leases(),
        1,
        "report volume on one frame must share one mounted evidence pin"
    );
    let right = batch(
        source(&isolated.session, right_binding, &right_basis),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report(1, keyboard(1), &right_basis)],
    );
    assert!(matches!(
        isolated.session.validate_host_observation_batch(right),
        UiHostObservationReportOutcome::Validated(_)
    ));
    let overflow = batch(
        source(&isolated.session, left_binding, &left_basis),
        (65, 65),
        UiHostObservationLoss::Complete,
        vec![report(65, keyboard(65), &left_basis)],
    );
    assert_eq!(
        isolated.session.validate_host_observation_batch(overflow),
        UiHostObservationReportOutcome::Denied(
            UiHostObservationReportDenial::LocalCapacityExceeded(UiHostObservationFamily::Keyboard)
        )
    );
    assert_eq!(
        isolated.session.retained_host_observation_report_count(),
        65
    );
}

#[test]
fn global_count_and_byte_budgets_deny_without_partial_cross_partition_retention() {
    assert_global_report_count_budget();
    assert_global_byte_budget();
}

fn assert_global_report_count_budget() {
    let mut world = multi_surface_observation_world("observation-global-report-budget", 9);
    for (binding, basis) in world.surfaces.iter().take(8) {
        for sequence in 1..=64 {
            let raw = batch(
                source(&world.session, *binding, basis),
                (sequence, sequence),
                UiHostObservationLoss::Complete,
                vec![report(sequence, keyboard(sequence), basis)],
            );
            assert!(matches!(
                world.session.validate_host_observation_batch(raw),
                UiHostObservationReportOutcome::Validated(_)
            ));
        }
    }
    assert_eq!(world.session.retained_host_observation_report_count(), 512);
    assert_eq!(
        world
            .session
            .mounted_retention_report()
            .class(UiMountedRetentionClass::ObservationBasis)
            .active_leases(),
        1,
        "partitions on one frame share one mounted evidence pin"
    );

    let (binding, basis) = world.surfaces[8];
    let denied = batch(
        source(&world.session, binding, &basis),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report(1, keyboard(1), &basis)],
    );
    assert_eq!(
        world.session.validate_host_observation_batch(denied),
        UiHostObservationReportOutcome::Denied(
            UiHostObservationReportDenial::GlobalCapacityExceeded(
                UiHostObservationFamily::Keyboard
            )
        )
    );
    assert_eq!(world.session.retained_host_observation_report_count(), 512);
}

fn assert_global_byte_budget() {
    let mut world = multi_surface_observation_world("observation-global-byte-budget", 9);
    for (binding, basis) in world.surfaces.iter().take(8) {
        let raw = batch(
            source(&world.session, *binding, basis),
            (1, 1),
            UiHostObservationLoss::Complete,
            vec![report(1, text_composition(1), basis)],
        );
        assert!(matches!(
            world.session.validate_host_observation_batch(raw),
            UiHostObservationReportOutcome::Validated(_)
        ));
    }
    let retained_bytes = world.session.retained_host_observation_byte_count();
    assert!(retained_bytes < 128 * 1024);

    let (binding, basis) = world.surfaces[8];
    let denied = batch(
        source(&world.session, binding, &basis),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report(1, text_composition(1), &basis)],
    );
    assert_eq!(
        world.session.validate_host_observation_batch(denied),
        UiHostObservationReportOutcome::Denied(
            UiHostObservationReportDenial::GlobalCapacityExceeded(
                UiHostObservationFamily::TextComposition
            )
        )
    );
    assert_eq!(world.session.retained_host_observation_report_count(), 8);
    assert_eq!(
        world.session.retained_host_observation_byte_count(),
        retained_bytes
    );
}

#[test]
fn pointer_capture_button_and_discrete_transitions_cut_coalescing_ranges() {
    let mut world = published_observation_world("observation-pointer-policy");
    let trace = vec![
        report(1, pointer(1, 10), &world.current),
        report(2, pointer(2, 20), &world.current),
        report(
            3,
            UiHostObservationPayload::PointerButton {
                pointer: 7,
                capture_epoch: 3,
                button: 1,
                pressed: true,
            },
            &world.current,
        ),
        report(4, pointer(4, 40), &world.current),
        report(
            5,
            UiHostObservationPayload::PointerMotion {
                pointer: 7,
                capture_epoch: 4,
                pressed_buttons: 1,
                x_subpixels: 50,
                y_subpixels: 5,
            },
            &world.current,
        ),
    ];
    let outcome = world.session.validate_host_observation_batch(batch(
        source(&world.session, world.binding, &world.current),
        (1, 5),
        UiHostObservationLoss::Complete,
        trace,
    ));
    let validated = match outcome {
        UiHostObservationReportOutcome::Validated(validated) => validated,
        other => panic!("pointer trace must validate: {other:?}"),
    };
    assert_eq!(
        validated.reports()[1].disposition(),
        UiHostObservationDisposition::Coalesced {
            replaced: range(1, 1)
        }
    );
    for index in [0, 2, 3, 4] {
        assert_eq!(
            validated.reports()[index].disposition(),
            UiHostObservationDisposition::Retained
        );
    }
    assert_eq!(world.session.retained_host_observation_report_count(), 4);
}

#[test]
fn adapter_call_only_enqueues_pointer_focus_and_measurement_until_presentation_returns() {
    let mut world =
        published_observation_world_with_host("observation-non-reentrant", measurement_host());
    let setup_event_count = world.host.observation_events().len();
    let raw = batch(
        source(&world.session, world.binding, &world.current),
        (1, 2),
        UiHostObservationLoss::Complete,
        vec![
            report(1, pointer(1, 10), &world.current),
            report(
                2,
                UiHostObservationPayload::Focus { focused: true },
                &world.current,
            ),
        ],
    );
    let measurement_request = begin_viewport(&mut world.session, Some(world.binding), 100, 0);
    let measurement_ingress = world.session.host_measurement_ingress();
    let measurement_completion = UiHostMeasurementCompletion::new(
        viewport_observation(&measurement_request, 800.0, 600.0),
        1,
    );
    world.host.enqueue_observation_during_next_presentation(raw);
    world.host.enqueue_measurement_during_next_presentation(
        measurement_ingress.clone(),
        measurement_completion,
    );
    world.host.push_presented();
    let successor = prepared(&mut world.session);
    assert!(matches!(
        world.session.present_prepared_mounted_frame(
            successor,
            UiPresentationDeadline::at_tick(1_000),
            0,
        ),
        UiMountedFrameOutcome::Published(_)
    ));
    let published_before_drain = world
        .session
        .current_mounted_publication()
        .expect("explicit presentation published")
        .clone();
    assert_eq!(world.host.pending_observation_batch_count(), 1);
    assert_eq!(measurement_ingress.pending_completion_count(), 1);
    assert_eq!(world.session.retained_host_observation_report_count(), 0);
    assert_eq!(world.session.pending_host_measurement_count(), 1);
    let events = world.host.observation_events();
    assert_eq!(
        &events[setup_event_count..],
        vec![
            "presentation-enter",
            "observation-enqueued",
            "measurement-enqueued",
            "presentation-exit"
        ]
    );

    let outcomes = world
        .session
        .drain_and_validate_host_observation_batches()
        .expect("scripted adapter drain stays within the canonical bound");
    assert_eq!(outcomes.len(), 1);
    let relation = match &outcomes[0] {
        UiHostObservationReportOutcome::Validated(validated) => validated.frame_relation(),
        other => panic!("queued predecessor report must validate later: {other:?}"),
    };
    assert_eq!(
        relation,
        UiHostObservationFrameRelation::SupersededPresented
    );
    let measurements = world.session.complete_enqueued_host_measurements();
    assert!(matches!(
        measurements.as_ref(),
        [UiHostMeasurementOutcome::Completed(_)]
    ));
    assert_eq!(world.session.pending_host_measurement_count(), 0);
    assert_eq!(
        world.session.current_mounted_publication(),
        Some(&published_before_drain),
        "structural report validation cannot schedule publication"
    );
}

fn keyboard(sequence: u64) -> UiHostObservationPayload {
    UiHostObservationPayload::Keyboard {
        physical_key: u32::try_from(sequence).unwrap(),
        pressed: sequence.is_multiple_of(2),
        repeat: false,
    }
}

fn text_composition(revision: u64) -> UiHostObservationPayload {
    UiHostObservationPayload::TextComposition {
        revision,
        text: "x".repeat(15_000).into_boxed_str(),
    }
}

fn range(first: u64, last: u64) -> UiHostObservationSequenceRange {
    UiHostObservationSequenceRange::new(
        UiHostObservationSequence::new(first),
        UiHostObservationSequence::new(last),
    )
}

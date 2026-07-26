use worth_ui::facade::observation_report::WorthUiHostObservationSessionExt;
use worth_ui::facade::observation_report::{
    UiHostObservationCapacity, UiHostObservationCapacityInput, UiHostObservationFamily,
    UiHostObservationLoss, UiHostObservationPayload, UiHostObservationReportDenial,
    UiHostObservationReportOutcome,
};
use worth_ui_runtime::facade::mounted::{
    UiHostSurfaceCancellationOutcome, UiHostSurfacePresentationMode,
    UiHostSurfacePresentationOutcome, UiMountedFrameOutcome, UiMountedFrameRetentionBudget,
    UiMountedFrameRetentionBudgetInput, UiMountedInspectionOmission, UiMountedInspectionReceipt,
    UiMountedInspectionRelation, UiMountedInspectionRequest, UiMountedInstanceIdentity,
    UiMountedRetentionClass, UiMountedRetentionClassBudget, UiMountedRetentionEvictionPosture,
    UiPresentationDeadline, UiSurfaceBindingGeneration,
};
use worth_ui_test_support::WorthUiMountedIdentityCertificationExt;
use worth_ui_test_support::WorthUiMountedPublicationCertificationExt;

use crate::host_observation_fixture::{batch, report, source};
use crate::mounted_application_lifecycle::in_flight_presentation_world::prepared;
use crate::mounted_application_lifecycle::known_empty_surface_world::{
    first_node, mounted_application_with_host_and_capacities, profile,
};
use crate::mounted_application_lifecycle::published_mounted_world::{
    publish, PresentedObservationBasis,
};
use crate::mounted_host_protocol::scripted_host::{presented_completion, ScriptedPresentationHost};

const LARGE_STRUCTURAL_BUDGET: usize = 128 * 1024 * 1024;

#[test]
fn sustained_frame_host_and_report_pressure_preserves_bounded_interpretable_truth() {
    let mut world = retention_pressure_world();
    let current = drive_frame_churn(&mut world);
    drive_host_lag(&mut world, current);
    let current = publish(&mut world.session, &world.host, world.instance);
    fill_observation_capacity(&mut world, current);
    fill_quarantine_capacity(&mut world, current);
    assert_bounded_retention_truth(&world, current);
}

struct RetentionPressureWorld {
    session: worth_ui::facade::app::WorthUiActiveApplicationSession,
    host: ScriptedPresentationHost,
    binding: UiSurfaceBindingGeneration,
    instance: UiMountedInstanceIdentity,
}

fn retention_pressure_world() -> RetentionPressureWorld {
    let host = ScriptedPresentationHost::default();
    let mut session = mounted_application_with_host_and_capacities(
        "mounted-retention-saturation",
        host.clone(),
        retention_budget(),
        observation_capacity(),
    )
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
    RetentionPressureWorld {
        session,
        host,
        binding,
        instance,
    }
}

fn drive_frame_churn(world: &mut RetentionPressureWorld) -> PresentedObservationBasis {
    let mut current = publish(&mut world.session, &world.host, world.instance);
    for _ in 1..32 {
        current = publish(&mut world.session, &world.host, world.instance);
        let report = world.session.mounted_retention_report();
        assert_eq!(
            report
                .class(UiMountedRetentionClass::Current)
                .retained_items(),
            1
        );
        assert!(
            report
                .class(UiMountedRetentionClass::PredecessorInspection)
                .retained_items()
                <= 2
        );
        assert_current_is_inspectable(&world.session, current);
    }
    current
}

fn drive_host_lag(world: &mut RetentionPressureWorld, predecessor: PresentedObservationBasis) {
    let successor = prepared(&mut world.session);
    let successor_frame = successor.canonical_core().frame();
    world.host.push_in_flight(
        vec![presented_completion()],
        UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun,
    );
    let in_flight = match world.session.present_prepared_mounted_frame(
        successor,
        UiPresentationDeadline::at_tick(1_000),
        0,
    ) {
        UiMountedFrameOutcome::InFlight(in_flight) => in_flight,
        _ => panic!("scripted host lag must retain an in-flight frame"),
    };
    let report = world.session.mounted_retention_report();
    assert_eq!(
        report
            .class(UiMountedRetentionClass::InFlight)
            .retained_items(),
        1
    );
    assert_eq!(
        world
            .session
            .current_mounted_publication()
            .expect("the predecessor publication remains current during host lag")
            .frame(),
        predecessor.frame
    );
    assert!(matches!(
        world
            .session
            .inspect_mounted_frame(UiMountedInspectionRequest::current()),
        UiMountedInspectionReceipt::Omitted(UiMountedInspectionOmission::FrameTransitionInFlight)
    ));
    assert!(matches!(
        world.session.complete_mounted_presentation(in_flight, 1),
        UiMountedFrameOutcome::Published(receipt) if receipt.frame() == successor_frame
    ));
    assert_eq!(
        world
            .session
            .mounted_retention_report()
            .class(UiMountedRetentionClass::InFlight)
            .retained_items(),
        0
    );
    assert_current_frame_is_inspectable(&world.session, successor_frame);
}

fn fill_observation_capacity(world: &mut RetentionPressureWorld, basis: PresentedObservationBasis) {
    for sequence in 1..=2 {
        assert!(matches!(
            world
                .session
                .validate_host_observation_batch(observation_batch(
                    world,
                    basis,
                    sequence,
                    keyboard(sequence),
                )),
            UiHostObservationReportOutcome::Validated(_)
        ));
    }
    assert_eq!(
        world
            .session
            .validate_host_observation_batch(observation_batch(world, basis, 3, keyboard(3),)),
        UiHostObservationReportOutcome::Denied(
            UiHostObservationReportDenial::LocalCapacityExceeded(UiHostObservationFamily::Keyboard)
        )
    );
}

fn fill_quarantine_capacity(
    world: &mut RetentionPressureWorld,
    current: PresentedObservationBasis,
) {
    let candidate = prepared(&mut world.session);
    let indeterminate_frame = candidate.canonical_core().frame();
    world
        .host
        .push_presentation(UiHostSurfacePresentationOutcome::PresentationIndeterminate);
    assert!(matches!(
        world.session.present_prepared_mounted_frame(
            candidate,
            UiPresentationDeadline::at_tick(2_000),
            1,
        ),
        UiMountedFrameOutcome::PresentationIndeterminate(_)
    ));
    let basis = PresentedObservationBasis {
        frame: indeterminate_frame,
        instance: current.instance,
        receipt: current.receipt,
    };
    assert!(matches!(
        world
            .session
            .validate_host_observation_batch(observation_batch(
                world,
                basis,
                3,
                UiHostObservationPayload::Tick { tick: 3 },
            )),
        UiHostObservationReportOutcome::Quarantined(_)
    ));
    assert_eq!(
        world
            .session
            .validate_host_observation_batch(observation_batch(
                world,
                basis,
                4,
                UiHostObservationPayload::Tick { tick: 4 },
            )),
        UiHostObservationReportOutcome::Denied(
            UiHostObservationReportDenial::QuarantineCountCapacityExceeded
        )
    );
}

fn observation_batch(
    world: &RetentionPressureWorld,
    basis: PresentedObservationBasis,
    sequence: u64,
    payload: UiHostObservationPayload,
) -> worth_ui::facade::observation_report::UiHostObservationBatch {
    batch(
        source(&world.session, world.binding, &basis),
        (sequence, sequence),
        UiHostObservationLoss::Complete,
        vec![report(sequence, payload, &basis)],
    )
}

fn assert_bounded_retention_truth(
    world: &RetentionPressureWorld,
    current: PresentedObservationBasis,
) {
    let report = world.session.mounted_retention_report();
    assert_eq!(report.classes().len(), 7);
    assert_evidence_class_within_budget(&report, UiMountedRetentionClass::Current);
    assert_evidence_class_within_budget(&report, UiMountedRetentionClass::InFlight);
    assert_evidence_class_within_budget(&report, UiMountedRetentionClass::ObservationBasis);
    assert_evidence_class_within_budget(&report, UiMountedRetentionClass::PredecessorInspection);
    let observation = report.class(UiMountedRetentionClass::ObservationBasis);
    assert_eq!(observation.retained_items(), 2);
    assert_eq!(observation.active_leases(), 1);
    assert!(observation.retained_structural_bytes() > 0);
    let observation_queue = observation
        .queue_budget()
        .expect("observation retention exposes its independent queue budget");
    assert_eq!(observation_queue.item_limit(), 2);
    assert!(observation.retained_structural_bytes() <= observation_queue.structural_byte_limit());
    let diagnostic = report.class(UiMountedRetentionClass::Diagnostic);
    assert_eq!(
        diagnostic.posture(),
        UiMountedRetentionEvictionPosture::OmittedByPolicy
    );
    assert_eq!(diagnostic.retained_items(), 0);
    assert_eq!(diagnostic.retained_structural_bytes(), 0);
    assert_eq!(
        diagnostic.evidence_budget(),
        Some(UiMountedRetentionClassBudget::new(0, 0))
    );
    let quarantine = report.class(UiMountedRetentionClass::Quarantine);
    assert_eq!(
        quarantine.posture(),
        UiMountedRetentionEvictionPosture::AdmissionBounded
    );
    assert_eq!(quarantine.retained_items(), 1);
    assert!(quarantine.retained_structural_bytes() > 0);
    let quarantine_queue = quarantine
        .queue_budget()
        .expect("quarantine retention exposes its independent queue budget");
    assert_eq!(quarantine_queue.item_limit(), quarantine.retained_items());
    assert!(quarantine.retained_structural_bytes() <= quarantine_queue.structural_byte_limit());
    let future = report.class(UiMountedRetentionClass::FutureSnapshot);
    assert_eq!(
        future.posture(),
        UiMountedRetentionEvictionPosture::Reserved
    );
    assert_eq!(future.retained_items(), 0);
    assert_eq!(future.retained_structural_bytes(), 0);
    assert_eq!(
        future.evidence_budget(),
        Some(UiMountedRetentionClassBudget::new(0, 0))
    );
    assert_current_is_inspectable(&world.session, current);
}

fn assert_evidence_class_within_budget(
    report: &worth_ui_runtime::facade::mounted::UiMountedRetentionReport,
    class: UiMountedRetentionClass,
) {
    let row = report.class(class);
    let budget = row
        .evidence_budget()
        .expect("frame evidence classes expose their exact budget");
    assert!(row.retained_items() <= budget.frame_limit());
    assert!(row.retained_structural_bytes() <= budget.structural_byte_limit());
    assert!(row.active_leases() <= budget.frame_limit());
    assert!(row.lease_charged_structural_bytes() <= budget.structural_byte_limit());
}

fn assert_current_is_inspectable(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    expected: PresentedObservationBasis,
) {
    match session.inspect_mounted_frame(
        UiMountedInspectionRequest::current().for_instance(expected.instance),
    ) {
        UiMountedInspectionReceipt::Available(inspection) => {
            assert_eq!(inspection.frame(), expected.frame);
            assert_eq!(inspection.relation(), UiMountedInspectionRelation::Current);
            assert_eq!(inspection.selected_node_receipt(), Some(expected.receipt));
        }
        other => panic!("the current retained frame must stay interpretable: {other:?}"),
    }
}

fn assert_current_frame_is_inspectable(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    expected: worth_ui_runtime::facade::mounted::UiMountedFrameIdentity,
) {
    match session.inspect_mounted_frame(UiMountedInspectionRequest::current()) {
        UiMountedInspectionReceipt::Available(inspection) => {
            assert_eq!(inspection.frame(), expected);
            assert_eq!(inspection.relation(), UiMountedInspectionRelation::Current);
        }
        other => panic!("the settled current frame must be inspectable: {other:?}"),
    }
}

fn keyboard(sequence: u64) -> UiHostObservationPayload {
    UiHostObservationPayload::Keyboard {
        physical_key: u32::try_from(sequence).unwrap(),
        pressed: sequence.is_multiple_of(2),
        repeat: false,
    }
}

fn retention_budget() -> UiMountedFrameRetentionBudget {
    UiMountedFrameRetentionBudget::new(UiMountedFrameRetentionBudgetInput {
        current: UiMountedRetentionClassBudget::new(1, LARGE_STRUCTURAL_BUDGET),
        in_flight: UiMountedRetentionClassBudget::new(1, LARGE_STRUCTURAL_BUDGET),
        observation_basis: UiMountedRetentionClassBudget::new(2, LARGE_STRUCTURAL_BUDGET),
        predecessor_inspection: UiMountedRetentionClassBudget::new(2, LARGE_STRUCTURAL_BUDGET),
        diagnostic: UiMountedRetentionClassBudget::new(0, 0),
        future_snapshot: UiMountedRetentionClassBudget::new(0, 0),
        expired_identity_limit: 8,
    })
}

fn observation_capacity() -> UiHostObservationCapacity {
    UiHostObservationCapacity::new(UiHostObservationCapacityInput {
        local_reports: 2,
        local_bytes: 16 * 1024,
        global_reports: 2,
        global_bytes: 32 * 1024,
        quarantined_batches: 1,
        quarantined_bytes: 16 * 1024,
    })
}

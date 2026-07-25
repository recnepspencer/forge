use worth_ui::facade::mounted::{
    UiHostSurfacePresentationMode, UiMountedFrameRetentionBudget,
    UiMountedFrameRetentionBudgetInput, UiMountedInspectionOmission, UiMountedInspectionReceipt,
    UiMountedInspectionRequest, UiMountedInstanceIdentity, UiMountedRetentionClass,
    UiMountedRetentionClassBudget, UiSurfaceBindingGeneration,
};
use worth_ui::facade::observation_report::{
    UiHostObservationCapacity, UiHostObservationCapacityInput, UiHostObservationFamily,
    UiHostObservationLoss, UiHostObservationPayload, UiHostObservationReportDenial,
    UiHostObservationReportOutcome,
};

use crate::host_observation_fixture::{batch, pointer, report, source};
use crate::mounted_application_lifecycle::known_empty_surface_world::{
    first_node, mounted_application_with_host_and_capacities, profile,
};
use crate::mounted_application_lifecycle::published_mounted_world::{
    publish, PresentedObservationBasis,
};
use crate::mounted_host_protocol::scripted_host::ScriptedPresentationHost;

#[test]
fn report_capacity_denial_releases_a_new_frame_pin() {
    let (mut session, host, binding, instance) = observation_world(
        "observation-basis-denial-rollback",
        UiMountedFrameRetentionBudget::default(),
        observation_capacity(1),
    );
    let first = publish(&mut session, &host, instance);
    assert_validated(
        &mut session,
        binding,
        ObservationEmission::new(first, keyboard(1), 1),
    );
    let second = publish(&mut session, &host, instance);
    let denied = observation_batch(
        &session,
        binding,
        ObservationEmission::new(second, keyboard(2), 2),
    );
    assert_eq!(
        session.validate_host_observation_batch(denied),
        UiHostObservationReportOutcome::Denied(
            UiHostObservationReportDenial::GlobalCapacityExceeded(
                UiHostObservationFamily::Keyboard
            )
        )
    );

    let observation = session
        .mounted_retention_report()
        .class(UiMountedRetentionClass::ObservationBasis)
        .clone();
    assert_eq!(observation.retained_items(), 1);
    assert_eq!(observation.active_leases(), 1);
}

#[test]
fn cross_frame_coalescing_releases_the_replaced_frames_last_pin() {
    let budget = two_predecessor_budget();
    let (mut session, host, binding, instance) = observation_world(
        "observation-basis-cross-frame-release",
        budget,
        observation_capacity(8),
    );
    let first = publish(&mut session, &host, instance);
    let second = publish(&mut session, &host, instance);
    assert_validated(
        &mut session,
        binding,
        ObservationEmission::new(first, pointer(1, 10), 1),
    );
    publish(&mut session, &host, instance);
    assert_validated(
        &mut session,
        binding,
        ObservationEmission::new(second, pointer(2, 20), 2),
    );

    let observation = session
        .mounted_retention_report()
        .class(UiMountedRetentionClass::ObservationBasis)
        .clone();
    assert_eq!(observation.retained_items(), 1);
    assert_eq!(observation.active_leases(), 1);

    publish(&mut session, &host, instance);
    assert!(matches!(
        session.inspect_mounted_frame(UiMountedInspectionRequest::frame(first.frame)),
        UiMountedInspectionReceipt::Omitted(UiMountedInspectionOmission::ExpiredFrame { .. })
    ));
}

fn observation_world(
    label: &str,
    retention_budget: UiMountedFrameRetentionBudget,
    observation_capacity: UiHostObservationCapacity,
) -> (
    worth_ui::facade::app::WorthUiActiveApplicationSession,
    ScriptedPresentationHost,
    UiSurfaceBindingGeneration,
    UiMountedInstanceIdentity,
) {
    let host = ScriptedPresentationHost::default();
    let mut session = mounted_application_with_host_and_capacities(
        label,
        host.clone(),
        retention_budget,
        observation_capacity,
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
    (session, host, binding, instance)
}

fn assert_validated(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    binding: UiSurfaceBindingGeneration,
    emission: ObservationEmission,
) {
    let raw = observation_batch(session, binding, emission);
    assert!(matches!(
        session.validate_host_observation_batch(raw),
        UiHostObservationReportOutcome::Validated(_)
    ));
}

fn observation_batch(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    binding: UiSurfaceBindingGeneration,
    emission: ObservationEmission,
) -> worth_ui::facade::observation_report::UiHostObservationBatch {
    batch(
        source(session, binding, &emission.basis),
        (emission.sequence, emission.sequence),
        UiHostObservationLoss::Complete,
        vec![report(emission.sequence, emission.payload, &emission.basis)],
    )
}

struct ObservationEmission {
    basis: PresentedObservationBasis,
    payload: UiHostObservationPayload,
    sequence: u64,
}

impl ObservationEmission {
    fn new(
        basis: PresentedObservationBasis,
        payload: UiHostObservationPayload,
        sequence: u64,
    ) -> Self {
        Self {
            basis,
            payload,
            sequence,
        }
    }
}

fn keyboard(sequence: u64) -> UiHostObservationPayload {
    UiHostObservationPayload::Keyboard {
        physical_key: u32::try_from(sequence).unwrap(),
        pressed: true,
        repeat: false,
    }
}

fn observation_capacity(global_reports: usize) -> UiHostObservationCapacity {
    UiHostObservationCapacity::new(UiHostObservationCapacityInput {
        local_reports: 64,
        local_bytes: 16 * 1024,
        global_reports,
        global_bytes: 128 * 1024,
        quarantined_batches: 8,
        quarantined_bytes: 16 * 1024,
    })
}

fn two_predecessor_budget() -> UiMountedFrameRetentionBudget {
    const LARGE: usize = 128 * 1024 * 1024;
    UiMountedFrameRetentionBudget::new(UiMountedFrameRetentionBudgetInput {
        current: UiMountedRetentionClassBudget::new(1, LARGE),
        in_flight: UiMountedRetentionClassBudget::new(1, LARGE),
        observation_basis: UiMountedRetentionClassBudget::new(8, LARGE),
        predecessor_inspection: UiMountedRetentionClassBudget::new(2, LARGE),
        diagnostic: UiMountedRetentionClassBudget::new(0, 0),
        future_snapshot: UiMountedRetentionClassBudget::new(0, 0),
        expired_identity_limit: 64,
    })
}

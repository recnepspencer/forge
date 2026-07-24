use worth_ui::facade::host::{
    UiHostAdapterSessionAuthority, UiHostSessionReleaseOutcome, UiHostSessionReleaseReceipt,
    WorthUiOperationalHostAdapter,
};
use worth_ui::facade::mounted::{
    UiHostSurfacePresentationMode, UiMountedFrameOutcome, UiMountedIdentityDenial,
    UiMountedPresentationAdmissionDenial, UiPresentationDeadline,
};
use worth_ui::facade::observation_report::{
    UiHostObservationLoss, UiHostObservationPayload, UiHostObservationReportOutcome,
};
use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiHostSurfaceRegistrationDenial,
    UiHostSurfaceRegistrationRequest, WorthUiHostCapabilityReport, WorthUiHostContract,
    WorthUiMeasurementHostAdapter,
};

use super::mounted_application_lifecycle::known_empty_surface_world::{
    active_session, first_node, mounted_application_with_host, profile,
};
use super::mounted_host_protocol::scripted_host::ScriptedPresentationHost;
use super::mounted_identity_lifecycle::{incarnation, receipt_ids};
use super::{
    host_observation_fixture::{batch, report, source},
    mounted_application_lifecycle::{
        in_flight_presentation_world::prepared,
        published_mounted_world::{
            published_observation_world_with_host, PublishedObservationWorld,
        },
    },
};

#[test]
fn surface_recreation_preserves_semantic_instance_but_retires_frame_affinity() {
    let mut session = active_session();
    let semantic_surface = session.create_semantic_surface().unwrap();
    let first_binding = session
        .register_host_surface(
            semantic_surface,
            UiHostSurfacePresentationMode::RecordOnly,
            profile(1),
        )
        .unwrap();
    let node = first_node(&session);
    let instance = session.mount_instance(node, semantic_surface).unwrap();
    let stable_incarnation = incarnation(&session, instance);
    let first_frame = session.advance_mounted_identity_frame().unwrap();
    let first_receipt = receipt_ids(&session);

    let second_binding = session
        .rebind_host_surface(
            first_binding.binding_generation(),
            UiHostSurfacePresentationMode::RecordOnly,
            profile(2),
        )
        .unwrap();
    assert_eq!(
        first_binding.semantic_surface_identity(),
        second_binding.semantic_surface_identity()
    );
    assert_ne!(
        first_binding.host_surface_identity(),
        second_binding.host_surface_identity()
    );
    assert_ne!(
        first_binding.binding_generation(),
        second_binding.binding_generation()
    );
    assert_eq!(incarnation(&session, instance), stable_incarnation);
    assert_eq!(
        session.validate_current_mounted_frame(first_frame),
        Err(UiMountedIdentityDenial::FrameNotCurrent)
    );
    assert_eq!(
        session.validate_current_mounted_node_receipt(instance, first_receipt[0]),
        Err(UiMountedIdentityDenial::NodeReceiptNotCurrent)
    );
    assert_binding_truth(&session, second_binding);
    let second_frame = session.advance_mounted_identity_frame().unwrap();
    assert_ne!(first_frame, second_frame);
    assert_ne!(first_receipt, receipt_ids(&session));
}

#[test]
fn registration_without_known_empty_truth_is_denied_before_binding() {
    let app = mounted_application_with_host("mounted-no-baseline", NoBaselineHost::default());
    let mut session = app.launch().expect("runtime should launch");
    let surface = session.create_semantic_surface().unwrap();
    assert_eq!(
        session.register_host_surface(
            surface,
            UiHostSurfacePresentationMode::RecordOnly,
            profile(1),
        ),
        Err(UiMountedIdentityDenial::KnownEmptyBaselineUnavailable)
    );
    assert!(session
        .inspect_mounted_identity()
        .surface_bindings()
        .is_empty());
}

#[test]
fn wrong_registration_receipt_blocks_semantic_truth_until_exact_native_removal() {
    let host = ScriptedPresentationHost::default();
    let app = mounted_application_with_host("mounted-wrong-registration", host.clone());
    let mut session = app.launch().expect("runtime should launch");
    let surface = session.create_semantic_surface().unwrap();
    host.return_wrong_next_registration_receipt();

    assert_eq!(
        session.register_host_surface(
            surface,
            UiHostSurfacePresentationMode::RecordOnly,
            profile(1),
        ),
        Err(UiMountedIdentityDenial::HostSurfaceTruthIndeterminate)
    );
    assert_eq!(host.native_registration_count(), 1);
    assert!(session
        .inspect_mounted_identity()
        .surface_bindings()
        .is_empty());
    assert_eq!(
        session.register_host_surface(
            surface,
            UiHostSurfacePresentationMode::RecordOnly,
            profile(2),
        ),
        Err(UiMountedIdentityDenial::SurfaceRequiresReconciliation)
    );

    session
        .recover_indeterminate_host_surface(surface)
        .expect("exact native removal restores known-empty truth");
    assert_eq!(host.native_registration_count(), 0);
    assert!(session
        .register_host_surface(
            surface,
            UiHostSurfacePresentationMode::RecordOnly,
            profile(3),
        )
        .is_ok());
}

#[test]
fn wrong_deregistration_receipt_preserves_identity_but_blocks_effectful_consumers() {
    let host = ScriptedPresentationHost::default();
    let mut world =
        published_observation_world_with_host("mounted-wrong-deregistration", host.clone());
    let surface =
        world.session.inspect_mounted_identity().surface_bindings()[0].semantic_surface_identity();
    host.return_wrong_next_deregistration_receipt();
    assert_eq!(
        world.session.deregister_host_surface(world.binding),
        Err(UiMountedIdentityDenial::HostSurfaceTruthIndeterminate)
    );
    assert_eq!(host.native_registration_count(), 0);
    assert!(world
        .session
        .validate_current_surface_binding(world.binding)
        .is_ok());

    let calls_before = host.presentation_calls();
    let frame = prepared(&mut world.session);
    match world.session.present_prepared_mounted_frame(
        frame,
        UiPresentationDeadline::at_tick(100),
        0,
    ) {
        UiMountedFrameOutcome::AdmissionDenied(rejection) => assert_eq!(
            rejection.denial(),
            UiMountedPresentationAdmissionDenial::BindingRequiresReconciliation(world.binding)
        ),
        _ => panic!("indeterminate deregistration must deny before adapter presentation"),
    }
    assert_eq!(host.presentation_calls(), calls_before);

    let observation = batch(
        source(&world.session, world.binding, &world.current),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report(
            1,
            UiHostObservationPayload::Focus { focused: true },
            &world.current,
        )],
    );
    assert!(matches!(
        world.session.validate_host_observation_batch(observation),
        UiHostObservationReportOutcome::Quarantined(_)
    ));

    world
        .session
        .recover_indeterminate_host_surface(surface)
        .expect("exact native re-registration restores predecessor truth");
    assert_eq!(host.native_registration_count(), 1);
    host.push_presented();
    let recovered_frame = prepared(&mut world.session);
    assert!(matches!(
        world.session.present_prepared_mounted_frame(
            recovered_frame,
            UiPresentationDeadline::at_tick(100),
            1,
        ),
        UiMountedFrameOutcome::Published(_)
    ));
}

#[test]
fn native_recovery_cannot_erase_same_binding_presentation_uncertainty() {
    let host = ScriptedPresentationHost::default();
    let mut world =
        published_observation_world_with_host("mounted-compound-uncertainty", host.clone());
    let surface =
        world.session.inspect_mounted_identity().surface_bindings()[0].semantic_surface_identity();

    host.push_presentation(
        worth_ui_host_contract::UiHostSurfacePresentationOutcome::PresentationIndeterminate,
    );
    let uncertain_frame = prepared(&mut world.session);
    assert!(matches!(
        world.session.present_prepared_mounted_frame(
            uncertain_frame,
            UiPresentationDeadline::at_tick(100),
            1,
        ),
        UiMountedFrameOutcome::PresentationIndeterminate(_)
    ));

    host.return_wrong_next_deregistration_receipt();
    assert_eq!(
        world.session.deregister_host_surface(world.binding),
        Err(UiMountedIdentityDenial::HostSurfaceTruthIndeterminate)
    );
    world
        .session
        .recover_indeterminate_host_surface(surface)
        .expect("native recovery restores only native lifecycle truth");

    assert_presentation_still_blocked(&mut world, &host);

    let replacement = world
        .session
        .rebind_host_surface(
            world.binding,
            UiHostSurfacePresentationMode::RecordOnly,
            profile(2),
        )
        .expect("recovered native truth permits a replacement binding");
    host.push_presented();
    assert!(matches!(
        world
            .session
            .present_current_mounted_frame_for_reconciliation(
                &[
                    worth_ui::facade::mounted::UiMountedSurfaceReconciliationBinding::new(
                        world.binding,
                        replacement.binding_generation(),
                    )
                ],
                UiPresentationDeadline::at_tick(100),
                3,
            )
            .unwrap(),
        UiMountedFrameOutcome::Reconciled(_)
    ));
}

fn assert_presentation_still_blocked(
    world: &mut PublishedObservationWorld,
    host: &ScriptedPresentationHost,
) {
    let calls_before = host.presentation_calls();
    let still_blocked = prepared(&mut world.session);
    assert!(matches!(
        world.session.present_prepared_mounted_frame(
            still_blocked,
            UiPresentationDeadline::at_tick(100),
            2,
        ),
        UiMountedFrameOutcome::AdmissionDenied(rejection)
            if rejection.denial()
                == UiMountedPresentationAdmissionDenial::BindingRequiresReconciliation(
                    world.binding,
                )
    ));
    assert_eq!(host.presentation_calls(), calls_before);
}

fn assert_binding_truth(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    binding: worth_ui::facade::mounted::UiSurfaceBindingIdentityView,
) {
    let capability = session.host_measurement_capability();
    assert_eq!(
        binding.capability_observation_generation(),
        capability.observation_generation()
    );
    assert_eq!(
        binding.capability_profile_digest(),
        capability.capability_report().profile_identity_digest()
    );
    let registration = binding.baseline().registration();
    assert_eq!(
        registration.host_session_identity(),
        session.host_session_identity().as_u64()
    );
    assert_eq!(
        registration.semantic_surface_identity(),
        binding.semantic_surface_identity()
    );
    assert_eq!(
        registration.host_surface_identity(),
        binding.host_surface_identity()
    );
    assert_eq!(
        registration.presentation_mode(),
        binding.presentation_mode()
    );
}

#[derive(Clone, Copy, Default)]
struct NoBaselineHost;

impl WorthUiMeasurementHostAdapter for NoBaselineHost {
    fn observe_measurement(
        &self,
        _request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        unreachable!("no measurement capability")
    }
}

impl WorthUiOperationalHostAdapter for NoBaselineHost {
    fn operational_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::headless()
    }

    fn operational_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::available(Vec::new())
    }

    fn register_surface(
        &self,
        _authority: &UiHostAdapterSessionAuthority,
        _request: UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceRegistrationOutcome {
        worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(
            UiHostSurfaceRegistrationDenial::KnownEmptyBaselineUnavailable,
        )
    }

    fn release_host_session(
        &self,
        authority: &UiHostAdapterSessionAuthority,
    ) -> UiHostSessionReleaseOutcome {
        UiHostSessionReleaseOutcome::Released(UiHostSessionReleaseReceipt::released(
            authority.host_session_identity(),
            0,
        ))
    }
}

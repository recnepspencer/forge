use worth_ui_runtime::facade::mounted::{
    UiHostPresentationReconciliation, UiHostSurfaceCancellationOutcome,
    UiHostSurfacePresentationMode, UiMountedFrameOutcome, UiMountedPresentationAdmissionDenial,
    UiPresentationDeadline,
};
use worth_ui_test_support::WorthUiMountedIdentityCertificationExt;
use worth_ui_test_support::WorthUiMountedPublicationCertificationExt;

use super::super::mounted_application_lifecycle::in_flight_presentation_world::prepared;
use super::super::mounted_application_lifecycle::known_empty_surface_world::profile;
use super::super::mounted_host_protocol::scripted_host::{
    presented_completion, ScriptedPresentationHost,
    ScriptedSurfaceCompletion as UiHostSurfaceInFlightCompletion,
};
use super::super::mounted_presentation_model_trace::assert_model_outcome;
use super::super::mounted_protocol_model::{ModelCompletion, ModelPresentation, ModelSurfaceStart};
use super::{expect_in_flight, published};

pub(super) fn present_synchronously(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    host: &ScriptedPresentationHost,
) -> worth_ui_runtime::facade::mounted::UiMountedFramePublicationReceipt {
    let frame = prepared(session);
    host.push_presented();
    host.push_presented();
    let model =
        ModelPresentation::start(&[ModelSurfaceStart::Presented, ModelSurfaceStart::Presented]);
    let outcome =
        session.present_prepared_mounted_frame(frame, UiPresentationDeadline::at_tick(10), 0);
    assert_model_outcome(&model, &outcome);
    published(outcome)
}

pub(super) fn present_asynchronously(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    host: &ScriptedPresentationHost,
) -> worth_ui_runtime::facade::mounted::UiMountedFramePublicationReceipt {
    let frame = prepared(session);
    host.push_in_flight(
        vec![
            UiHostSurfaceInFlightCompletion::Pending,
            presented_completion(),
        ],
        UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun,
    );
    host.push_in_flight(
        vec![presented_completion()],
        UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun,
    );
    let mut model =
        ModelPresentation::start(&[ModelSurfaceStart::InFlight, ModelSurfaceStart::InFlight]);
    let first =
        session.present_prepared_mounted_frame(frame, UiPresentationDeadline::at_tick(10), 0);
    assert_model_outcome(&model, &first);
    let first_poll = expect_in_flight(first);
    assert_eq!(first_poll.pending_bindings().count(), 2);
    model.complete(0, ModelCompletion::Pending);
    model.complete(1, ModelCompletion::Presented);
    let second = session.complete_mounted_presentation(first_poll, 1);
    assert_model_outcome(&model, &second);
    let second_poll = expect_in_flight(second);
    assert_eq!(second_poll.pending_bindings().count(), 1);
    let duplicate = second_poll.clone();
    model.complete(0, ModelCompletion::Presented);
    let terminal = session.complete_mounted_presentation(second_poll, 2);
    assert_model_outcome(&model, &terminal);
    let receipt = published(terminal);
    assert!(matches!(
        session.complete_mounted_presentation(duplicate, 2),
        UiMountedFrameOutcome::CompletionDenied(
            worth_ui_runtime::facade::mounted::UiMountedPresentationCompletionDenial::UnknownAttempt
        )
    ));
    receipt
}

pub(super) fn publish_partial_effects(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    host: &ScriptedPresentationHost,
) -> Vec<worth_ui_host_contract::UiSurfaceBindingGeneration> {
    let frame = prepared(session);
    host.push_presented();
    host.push_rejected();
    let model = ModelPresentation::start(&[
        ModelSurfaceStart::Presented,
        ModelSurfaceStart::RejectedBeforeEffects,
    ]);
    let outcome =
        session.present_prepared_mounted_frame(frame, UiPresentationDeadline::at_tick(10), 0);
    assert_model_outcome(&model, &outcome);
    match outcome {
        UiMountedFrameOutcome::PresentationIndeterminate(value) => {
            let affected = value.report().affected_bindings().to_vec();
            assert_eq!(affected.len(), 2);
            affected
        }
        _ => panic!("success on one required surface plus rejection is indeterminate"),
    }
}

pub(super) fn rebind_and_reconcile_affected(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    affected: &[worth_ui_host_contract::UiSurfaceBindingGeneration],
) {
    let old_views = session
        .inspect_mounted_identity()
        .surface_bindings()
        .to_vec();
    for old in &old_views {
        session
            .rebind_host_surface(
                old.binding_generation(),
                UiHostSurfacePresentationMode::RecordOnly,
                profile(2),
            )
            .unwrap();
    }
    let replacements = session
        .inspect_mounted_identity()
        .surface_bindings()
        .to_vec();
    let blocked_frame = prepared(session);
    assert!(matches!(
        session.present_prepared_mounted_frame(
            blocked_frame,
            UiPresentationDeadline::at_tick(10),
            0
        ),
        UiMountedFrameOutcome::AdmissionDenied(rejection)
            if matches!(
                rejection.denial(),
                UiMountedPresentationAdmissionDenial::BindingRequiresReconciliation(_)
            )
    ));
    for affected_binding in affected {
        let semantic_surface = old_views
            .iter()
            .find(|view| view.binding_generation() == *affected_binding)
            .unwrap()
            .semantic_surface_identity();
        let replacement = replacements
            .iter()
            .find(|view| view.semantic_surface_identity() == semantic_surface)
            .unwrap()
            .to_owned();
        assert!(session.reconcile_mounted_presentation(
            UiHostPresentationReconciliation::KnownEmptyBaseline {
                affected_binding: *affected_binding,
                replacement,
            },
        ));
    }
}

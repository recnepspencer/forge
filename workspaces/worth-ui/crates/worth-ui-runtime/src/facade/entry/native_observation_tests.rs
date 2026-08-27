use crate::certification_support::ScriptedPresentationHost;
use crate::mounting::{UiMountedFrameOutcome, UiMountedFramePublicationReceipt};
use crate::runtime::tests::active_application_session_test_support::source_backed_focusable_component_app_with_host;
use worth_ui_host_contract::{
    UiHostObservationBatch, UiHostObservationBatchInput, UiHostObservationLoss,
    UiHostObservationPayload, UiHostObservationPresentationBasis, UiHostObservationReport,
    UiHostObservationSequence, UiHostObservationSequenceRange, UiHostObservationTimeBasis,
    UiHostPresentationEpoch, UiHostProtocolContract, UiHostProtocolNegotiation,
};

#[test]
fn native_observation_ready_path_drains_through_runtime_interaction_owner() {
    let host = ScriptedPresentationHost::native_display();
    host.push_native_display_presented();
    let mut shell = source_backed_focusable_component_app_with_host(host.clone())
        .launch_native_surface()
        .expect("native shell should launch");

    let first = published(shell.present_frame(100, 1), "first");
    assert_eq!(shell.session.focus.participant_count_for_test(), 1);
    let binding = *first.bindings().first().expect("native binding");
    let host_surface = shell.session.mounted.view().surface_bindings()[0].host_surface_identity();
    let batch = focus_batch(
        shell.session.host_session.identity().as_u64(),
        UiHostObservationPresentationBasis::new(
            host_surface,
            first.frame(),
            binding,
            UiHostPresentationEpoch::issued_by_host(1),
        ),
    );
    host.enqueue_observation_for_next_drain(batch);
    let settlement = shell.admit_native_observation_batches(Default::default());
    assert_eq!(settlement.counts(), (1, 0, 0, 0));
    assert_eq!(settlement.drain_denial(), None);
    let outcomes = settlement.into_outcomes();
    assert_eq!(outcomes.len(), 1);
    assert!(matches!(
        &outcomes[0],
        crate::facade::interaction::UiHostInteractionIngressOutcome::Applied(_)
    ));
    assert!(shell.session.focus.window_is_focused_for_test());

    let shutdown = shell.shutdown();
    assert!(shutdown.host_session_released());
}

fn published(
    outcome: Result<
        UiMountedFrameOutcome,
        crate::facade::entry::WorthUiMountedFrameExecutionStop<'_>,
    >,
    label: &str,
) -> UiMountedFramePublicationReceipt {
    let outcome = match outcome {
        Ok(outcome) => outcome,
        Err(_) => panic!("{label} frame should execute"),
    };
    match outcome {
        UiMountedFrameOutcome::Published(receipt)
        | UiMountedFrameOutcome::Unchanged(receipt)
        | UiMountedFrameOutcome::Reconciled(receipt) => receipt,
        UiMountedFrameOutcome::RejectedBeforeEffects(_) => {
            panic!("{label} frame was rejected before effects")
        }
        UiMountedFrameOutcome::InFlight(_) => panic!("{label} frame remained in flight"),
        UiMountedFrameOutcome::Superseded(_) => panic!("{label} frame was superseded"),
        UiMountedFrameOutcome::PresentationIndeterminate(_) => {
            panic!("{label} frame became indeterminate")
        }
        UiMountedFrameOutcome::RetentionDenied(_) => panic!("{label} frame retention was denied"),
        UiMountedFrameOutcome::AdmissionDenied(_) => panic!("{label} frame admission was denied"),
        UiMountedFrameOutcome::CompletionDenied(_) => panic!("{label} frame completion was denied"),
    }
}

fn focus_batch(
    host_session: u64,
    presentation: UiHostObservationPresentationBasis,
) -> UiHostObservationBatch {
    let protocol = match UiHostProtocolContract::current().negotiate() {
        UiHostProtocolNegotiation::Compatible(agreement) => agreement,
        UiHostProtocolNegotiation::Incompatible(_) => unreachable!(),
    };
    let sequence = UiHostObservationSequence::new(1);
    UiHostObservationBatch::new(UiHostObservationBatchInput {
        protocol,
        host_session,
        presentation,
        sequences: UiHostObservationSequenceRange::new(sequence, sequence),
        loss: UiHostObservationLoss::Complete,
        reports: vec![UiHostObservationReport::new(
            sequence,
            UiHostObservationTimeBasis::HostMonotonicMillis(2),
            UiHostObservationPayload::WindowFocus {
                surface: presentation.host_surface(),
                focused: true,
            },
        )],
    })
    .expect("focus observation batch should satisfy the host contract")
}

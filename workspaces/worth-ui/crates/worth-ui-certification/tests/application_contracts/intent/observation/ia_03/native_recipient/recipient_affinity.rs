use worth_ui::facade::interaction::{
    UiInteractionBatchReceipt, UiInteractionStop, UiInteractionTransition,
    UiLocalInputRecipientContract, UiLocalInputStopReason, UiSemanticInteraction,
};
use worth_ui::facade::observation_report::{
    UiHostObservationBatch, UiHostObservationBatchInput, UiHostObservationReport,
};
use worth_ui_host_contract::{
    UiHostInputRecipientAffinityReceipt, UiHostInputRecipientBindingInput,
    UiHostInputRecipientBindingReceipt, UiTextProfileGeneration,
};

use super::{applied, bind_native_draft, key, native_activation, InteractionWorld};

#[test]
fn delayed_report_for_replaced_recipient_cannot_mutate_successor() {
    let mut world = InteractionWorld::native();
    bind_native_draft(&mut world);
    let report_a = admit_text_and_capture_report(&mut world, "seed-a");

    let activation_b = native_activation(&mut world);
    let rebound = world
        .session
        .bind_local_input_recipient(activation_b, UiLocalInputRecipientContract::activation())
        .expect("the successor recipient installs before displacing its predecessor");
    assert_eq!(
        rebound.displaced_recipient().unwrap().reason(),
        UiLocalInputStopReason::RecipientReplaced
    );

    let carrier = world.retain_native_input(vec![egui::Event::WindowFocused(true)]);
    let delayed_batch = rewrite_report(take_one_batch(carrier.into_batches()), &report_a);
    let receipt = applied(world.admit_native_batch(delayed_batch));
    let stop = local_stop(&receipt);
    assert_eq!(
        stop.reason(),
        UiLocalInputStopReason::InputRecipientAffinityChanged
    );
    assert!(!stop.settled_recipient());
    assert_eq!(receipt.state().active_recipients(), 1);

    let current = world.native_input(vec![key(egui::Key::Enter, false)]);
    let receipt = applied(current.into_runtime().into_vec().remove(0));
    assert!(receipt.transitions().iter().any(|transition| matches!(
        transition,
        UiInteractionTransition::Semantic(UiSemanticInteraction::Activate(_))
    )));
}

#[test]
fn stale_text_profile_report_is_rejected_without_settling_current_recipient() {
    let mut world = InteractionWorld::native();
    bind_native_draft(&mut world);

    let retained = world.retain_native_input(vec![egui::Event::Text("stale".to_owned())]);
    let source = take_one_batch(retained.into_batches());
    let current = source.reports()[0]
        .input_affinity()
        .expect("the production translator stamps the installed draft affinity");
    let stale_profile =
        UiTextProfileGeneration::new(current.binding().text_profile().unwrap().get() + 1)
            .expect("the successor profile generation is nonzero");
    let stale = UiHostInputRecipientAffinityReceipt::at_event_time(
        binding_with_text_profile(current.binding(), stale_profile),
        current.presentation(),
    );
    let receipt = applied(world.admit_native_batch(rewrite_affinity(source, stale)));
    let stop = local_stop(&receipt);
    assert_eq!(
        stop.reason(),
        UiLocalInputStopReason::TextProfileGenerationChanged {
            expected: current.binding().text_profile().unwrap(),
            observed: Some(stale_profile),
        }
    );
    assert!(!stop.settled_recipient());
    assert_eq!(receipt.state().active_recipients(), 1);

    let committed = commit_text(&mut world, "current");
    assert_eq!(committed_text(&committed), "current");
}

fn admit_text_and_capture_report(
    world: &mut InteractionWorld,
    text: &str,
) -> UiHostObservationReport {
    let retained = world.retain_native_input(vec![egui::Event::Text(text.to_owned())]);
    let batch = take_one_batch(retained.into_batches());
    let report = batch.reports()[0].clone();
    assert!(report.input_affinity().is_some());
    let _ = applied(world.admit_native_batch(batch));
    report
}

fn commit_text(world: &mut InteractionWorld, text: &str) -> UiInteractionBatchReceipt {
    let ingress = world.native_input(vec![
        egui::Event::Text(text.to_owned()),
        key(egui::Key::Enter, false),
    ]);
    applied(ingress.into_runtime().into_vec().remove(0))
}

fn committed_text(receipt: &UiInteractionBatchReceipt) -> &str {
    receipt
        .transitions()
        .iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Semantic(UiSemanticInteraction::EditCommit(edit)) => {
                Some(edit.committed_text())
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("enter commit missing from receipt: {receipt:#?}"))
}

fn local_stop(
    receipt: &UiInteractionBatchReceipt,
) -> &worth_ui::facade::interaction::UiLocalInputStop {
    receipt
        .transitions()
        .iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Stopped(UiInteractionStop::LocalInput(stop)) => Some(stop),
            _ => None,
        })
        .expect("the rejected report emits a typed local-input stop")
}

fn take_one_batch(batches: Box<[UiHostObservationBatch]>) -> UiHostObservationBatch {
    assert_eq!(batches.len(), 1);
    batches.to_vec().remove(0)
}

fn rewrite_affinity(
    batch: UiHostObservationBatch,
    affinity: UiHostInputRecipientAffinityReceipt,
) -> UiHostObservationBatch {
    assert_eq!(batch.reports().len(), 1);
    let source = &batch.reports()[0];
    let mut report = UiHostObservationReport::new(
        source.sequence(),
        source.time_basis(),
        source.payload().clone(),
    );
    if let Some(mounted) = source.mounted_basis() {
        report = report.with_mounted_basis(mounted);
    }
    report = report.with_input_affinity(affinity);
    rebuild_batch(
        batch,
        report,
        "the adversarial batch remains structurally canonical",
    )
}

fn rewrite_report(
    batch: UiHostObservationBatch,
    source: &UiHostObservationReport,
) -> UiHostObservationBatch {
    let affinity = source
        .input_affinity()
        .expect("the delayed source report carries its event-time affinity");
    assert_eq!(batch.reports().len(), 1);
    let carrier = &batch.reports()[0];
    let report = UiHostObservationReport::new(
        carrier.sequence(),
        carrier.time_basis(),
        source.payload().clone(),
    )
    .with_input_affinity(affinity);
    rebuild_batch(
        batch,
        report,
        "the delayed adversarial batch remains structurally canonical",
    )
}

fn rebuild_batch(
    batch: UiHostObservationBatch,
    report: UiHostObservationReport,
    expectation: &str,
) -> UiHostObservationBatch {
    let core = batch.canonical_core();
    UiHostObservationBatch::new(UiHostObservationBatchInput {
        protocol: core.protocol(),
        host_session: core.host_session(),
        presentation: core.presentation(),
        sequences: core.sequences(),
        loss: core.loss(),
        reports: vec![report],
    })
    .unwrap_or_else(|_| panic!("{expectation}"))
}

fn binding_with_text_profile(
    binding: UiHostInputRecipientBindingReceipt,
    text_profile: UiTextProfileGeneration,
) -> UiHostInputRecipientBindingReceipt {
    UiHostInputRecipientBindingReceipt::new(UiHostInputRecipientBindingInput {
        host_session: binding.host_session(),
        application_generation: binding.application_generation(),
        recipient_generation: binding.recipient_generation(),
        family: binding.family(),
        draft_session: binding.draft_session(),
        surface: binding.surface(),
        binding: binding.binding(),
        mounted_instance: binding.mounted_instance(),
        node_receipt: binding.node_receipt(),
        text_profile: Some(text_profile),
    })
}

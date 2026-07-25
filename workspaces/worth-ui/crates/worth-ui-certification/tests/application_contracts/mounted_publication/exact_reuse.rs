use super::*;

#[test]
fn ordinary_publication_preserves_predecessor_and_exact_reuse_skips_the_adapter() {
    let host = ScriptedPresentationHost::default();
    let (mut session, _) = mounted_session(host.clone(), "mounted-publication-ordinary", 1);
    let request = UiMountedFrameRequest::all_bound_surfaces();
    host.push_presented();
    let first_frame = session
        .execute_framework_turn(|_| {})
        .unwrap()
        .into_execution()
        .unwrap_or_else(|_| panic!("empty source turn permits mounted preparation"))
        .prepare_mounted_frame(request.clone())
        .unwrap();
    let first = published(session.present_prepared_mounted_frame(
        first_frame,
        UiPresentationDeadline::at_tick(10),
        0,
    ));
    assert_eq!(
        session.inspect_mounted_identity().current_frame(),
        Some(first.frame())
    );
    assert_eq!(first.predecessor(), None);
    assert_eq!(first.cost_report().named().retained(), 1);

    let execution = session
        .execute_framework_turn(|_| {})
        .unwrap()
        .into_execution()
        .unwrap_or_else(|_| panic!("empty source turn carries exact reuse authority"));
    let witness = match execution.classify_mounted_frame_reuse(&request) {
        UiMountedFrameReuse::Exact(witness) => witness,
        UiMountedFrameReuse::ComparisonRequired(_) => {
            panic!("identical framework turn must carry exact reuse")
        }
    };
    drop(execution);
    let calls_before_reuse = host.presentation_calls();
    let outcome = session
        .reuse_current_mounted_frame(&witness)
        .expect("exact current witness reuses");
    assert_constant_unchanged_cost(outcome.cost_report().unwrap());
    let unchanged = match outcome {
        UiMountedFrameOutcome::Unchanged(receipt) => receipt,
        _ => panic!("exact reuse is classified as unchanged"),
    };
    assert_eq!(unchanged, first);
    assert_eq!(host.presentation_calls(), calls_before_reuse);

    host.push_rejected();
    let rejected = prepared(&mut session);
    let rejected_frame = rejected.canonical_core().frame();
    assert!(matches!(
        session.present_prepared_mounted_frame(rejected, UiPresentationDeadline::at_tick(20), 1,),
        UiMountedFrameOutcome::RejectedBeforeEffects(_)
    ));
    assert_ne!(rejected_frame, first.frame());
    assert_eq!(
        session.current_mounted_publication(),
        Some(&first),
        "effect-free rejection preserves predecessor publication"
    );

    let binding = session.inspect_mounted_identity().surface_bindings()[0];
    session
        .rebind_host_surface(
            binding.binding_generation(),
            binding.presentation_mode(),
            profile(2),
        )
        .unwrap();
    assert!(session.reuse_current_mounted_frame(&witness).is_none());
}

fn assert_constant_unchanged_cost(cost: worth_ui::facade::mounted::UiMountCostReport) {
    assert_eq!(
        cost.work_class(),
        worth_ui::facade::mounted::UiMountWorkClass::UnchangedReuse
    );
    assert_eq!(cost.initial_mounted_instances(), 0);
    assert_eq!(cost.changed_mounted_instances(), 0);
    assert_eq!(cost.index_entries_touched(), 0);
    assert_eq!(cost.replaced_batch_rows(), 0);
    assert_eq!(cost.replaced_batch_bytes(), 0);
    assert_eq!(cost.surface_instance_pairs(), 0);
    assert_eq!(cost.changed_binding_generations(), 0);
    assert_eq!(cost.adapter().presented_surfaces(), 0);
    assert_eq!(cost.named().considered(), 0);
    assert_eq!(cost.named().minted(), 0);
    assert_eq!(cost.named().reused(), 1);
}

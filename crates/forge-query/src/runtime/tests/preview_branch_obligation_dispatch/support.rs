pub(super) use crate::runtime::tests::support::*;

pub(super) fn runtime_with_obligation(
    label: &str,
    support_posture: ForgeQueryGraphObligationSupportPosture,
    world: ForgeQueryGraphObligationOperatingWorldSelector,
) -> ForgeQueryRuntime {
    runtime_with_registration(collection_registration(
        "Task",
        label,
        support_posture,
        world,
    ))
}

pub(super) fn intent_runtime_with_obligation(
    label: &str,
    support_posture: ForgeQueryGraphObligationSupportPosture,
    world: ForgeQueryGraphObligationOperatingWorldSelector,
) -> ForgeQueryRuntime {
    complete_backend_from_parts_builder()
        .intent_authority(TestIntentAuthority)
        .support_profile(intent_support_profile())
        .graph_obligation(collection_registration(
            "Task",
            label,
            support_posture,
            world,
        ))
        .build_backend_from_parts()
        .build()
        .expect("intent runtime should build with graph obligation")
}

pub(super) fn runtime_with_registration(
    registration: ForgeQueryGraphObligationRegistration,
) -> ForgeQueryRuntime {
    complete_backend_from_parts_builder()
        .intent_authority(TestIntentAuthority)
        .support_profile(intent_support_profile())
        .graph_obligation(registration)
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with graph obligation")
}

pub(super) fn collection_registration(
    collection: &str,
    label: &str,
    support_posture: ForgeQueryGraphObligationSupportPosture,
    world: ForgeQueryGraphObligationOperatingWorldSelector,
) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::schema_contract_validator(
        ForgeQueryGraphObligationRuleIdentity::new("test.preview-branch-obligation", label, "v1")
            .unwrap(),
        ForgeQueryGraphTouchSelector::collection(collection).unwrap(),
        world,
    )
    .with_support_posture(support_posture)
}

pub(super) fn touch_bearing_intent(
    name: &str,
    descriptor: ForgeQueryGraphTouchDescriptor,
) -> ForgeQueryTouchBearingIntentDeclaration {
    ForgeQueryTouchBearingIntentDeclaration::new(plain_intent(name), descriptor)
}

pub(super) fn touch_bearing_intent_declaration(name: &str) -> ForgeQueryIntentDeclaration {
    touch_bearing_intent(name, task_touch_descriptor(name)).into_declaration()
}

pub(super) fn plain_intent(name: &str) -> ForgeQueryIntentDeclaration {
    ForgeQueryIntentDeclaration::strategy_commit(
        name,
        "strategy.intent.reconcile",
        "1.0",
        "intent.reconcile.input.v1",
        test_intent_input([("entity", "task-1")]),
    )
}

pub(super) fn task_touch_descriptor(id: &str) -> ForgeQueryGraphTouchDescriptor {
    ForgeQueryGraphTouchDescriptor::from_mutation_command_batch(&[task_insert_command(id)])
        .expect("task command should derive graph touch descriptor")
}

pub(super) fn task_insert_command(id: &str) -> ForgeQueryWriteCommand {
    ForgeQueryWriteCommand::InsertAspects {
        collection: "Task".to_string(),
        aspects: vec![
            ForgeQueryAspectValue::new(
                test_aspect_touch("identity.id"),
                test_string_aspect_value(id),
            )
            .unwrap(),
            ForgeQueryAspectValue::new(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Preview task"),
            )
            .unwrap(),
        ],
        symbolic_aspect_references: Vec::new(),
        metadata: ForgeQueryMutationMetadata::new(),
        naming_intent: None,
        continuity_intent: None,
        symbolic_target_reference: None,
    }
}

pub(super) fn assert_zero_selection_dispatch(
    dispatch: &ForgeQueryAuthoritativeMutationObligationDispatch,
) {
    assert_eq!(dispatch.selection().matched_obligation_count(), 0);
    assert!(dispatch.envelope().is_none());
    assert_eq!(dispatch.execution_inputs().len(), 0);
    assert_eq!(
        dispatch
            .selection()
            .counters()
            .registration_full_scan_count(),
        0
    );
}

pub(super) fn only_projection_row(
    dispatch: &ForgeQueryAuthoritativeMutationObligationDispatch,
) -> ForgeQueryAuthoritativeMutationObligationDispatchProjectionRow {
    let projection = dispatch.evidence_projection();
    assert_eq!(projection.rows().len(), 1);
    projection.rows()[0].clone()
}

pub(super) fn selected_rule_identity_digests(
    dispatch: &ForgeQueryAuthoritativeMutationObligationDispatch,
) -> Vec<String> {
    dispatch
        .selection()
        .matched_registrations()
        .iter()
        .map(|registration| registration.rule_identity().identity_digest().to_string())
        .collect()
}

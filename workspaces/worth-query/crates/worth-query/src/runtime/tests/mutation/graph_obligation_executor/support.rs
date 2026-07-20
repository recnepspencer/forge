pub(super) use crate::runtime::tests::support::*;

pub(super) fn runtime_with_registration(
    registration: WorthQueryGraphObligationRegistration,
) -> WorthQueryRuntime {
    complete_backend_from_parts_builder()
        .graph_obligation(registration)
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with graph obligation")
}

pub(super) fn runtime_with_registrations(
    registrations: impl IntoIterator<Item = WorthQueryGraphObligationRegistration>,
) -> WorthQueryRuntime {
    let mut builder = complete_backend_from_parts_builder();
    for registration in registrations {
        builder = builder.graph_obligation(registration);
    }
    builder
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with graph obligations")
}

pub(super) fn task_insert_command(id: &str) -> WorthQueryWriteCommand {
    WorthQueryWriteCommand::InsertAspects {
        collection: crate::runtime::WorthQueryMutationTargetCollectionIdentity::new(
            "write-command-declared",
            "Task",
        ),
        aspects: vec![
            WorthQueryAuthoredAspectMutation::new(
                test_aspect_touch("identity.id"),
                test_string_aspect_value(id),
            )
            .unwrap(),
            WorthQueryAuthoredAspectMutation::new(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Phase 11 executor task"),
            )
            .unwrap(),
        ],
        symbolic_aspect_references: Vec::new(),
        metadata: WorthQueryMutationMetadata::new(),
        naming_intent: None,
        continuity_intent: None,
        symbolic_target_reference: None,
    }
}

pub(super) fn task_collection_registration(
    kind: WorthQueryGraphObligationKind,
    name: &str,
    support_posture: WorthQueryGraphObligationSupportPosture,
) -> WorthQueryGraphObligationRegistration {
    WorthQueryGraphObligationRegistration::new(
        kind,
        WorthQueryGraphObligationRuleIdentity::new(
            "test.phase-eleven.graph-obligation-executor",
            name,
            "v1",
        )
        .unwrap(),
        WorthQueryGraphTouchSelector::collection("Task").unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
    .with_support_posture(support_posture)
}

pub(super) fn supported_command_batch_posture() -> WorthQueryGraphObligationSupportPosture {
    WorthQueryGraphObligationSupportPosture::supported(
        WorthQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
    )
}

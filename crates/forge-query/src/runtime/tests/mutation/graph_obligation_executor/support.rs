pub(super) use crate::runtime::tests::support::*;

pub(super) fn runtime_with_registration(
    registration: ForgeQueryGraphObligationRegistration,
) -> ForgeQueryRuntime {
    complete_backend_from_parts_builder()
        .graph_obligation(registration)
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with graph obligation")
}

pub(super) fn runtime_with_registrations(
    registrations: impl IntoIterator<Item = ForgeQueryGraphObligationRegistration>,
) -> ForgeQueryRuntime {
    let mut builder = complete_backend_from_parts_builder();
    for registration in registrations {
        builder = builder.graph_obligation(registration);
    }
    builder
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with graph obligations")
}

pub(super) fn task_insert_command(id: &str) -> ForgeQueryWriteCommand {
    ForgeQueryWriteCommand::InsertAspects {
        collection: "Task".to_string(),
        aspects: vec![
            ForgeQueryAspectValue::new("identity.id", id).unwrap(),
            ForgeQueryAspectValue::new("title.value", "Phase 11 executor task").unwrap(),
        ],
        symbolic_aspect_references: Vec::new(),
        metadata: ForgeQueryMutationMetadata::new(),
        naming_intent: None,
        continuity_intent: None,
        symbolic_target_reference: None,
    }
}

pub(super) fn task_collection_registration(
    kind: ForgeQueryGraphObligationKind,
    name: &str,
    support_posture: ForgeQueryGraphObligationSupportPosture,
) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::new(
        kind,
        ForgeQueryGraphObligationRuleIdentity::new(
            "test.phase-eleven.graph-obligation-executor",
            name,
            "v1",
        )
        .unwrap(),
        ForgeQueryGraphTouchSelector::collection("Task").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
    .with_support_posture(support_posture)
}

pub(super) fn supported_command_batch_posture() -> ForgeQueryGraphObligationSupportPosture {
    ForgeQueryGraphObligationSupportPosture::supported(
        ForgeQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
    )
}

use forge_query::facade::{
    worth_ui_query_binding_evidence_identity, ForgeQueryEvidenceIdentity,
    ForgeQueryRuntimeAsyncResultState, ForgeQueryRuntimeAsyncResultStateKind,
};
use worth_ui::facade::{
    RuntimeOutcomeAffordance, RuntimeOutcomeDenialPosture, RuntimeOutcomeFamily,
    RuntimeOutcomePresentation, RuntimeOutcomeProjectionDescriptor, RuntimeOutcomeProjectionId,
    RuntimeOutcomeRecoveryPosture, RuntimeOutcomeSourceReference, RuntimeOutcomeTone,
};

pub(crate) fn denied_projection(id: &str) -> RuntimeOutcomeProjectionDescriptor {
    RuntimeOutcomeProjectionDescriptor::new(
        projection_id(id),
        RuntimeOutcomeFamily::denied(),
        denied_source_reference(),
    )
    .with_presentation(blocking_presentation("Needs permission"))
    .with_denial_posture(RuntimeOutcomeDenialPosture::structured_status())
}

pub(crate) fn failed_projection(id: &str) -> RuntimeOutcomeProjectionDescriptor {
    RuntimeOutcomeProjectionDescriptor::new(
        projection_id(id),
        RuntimeOutcomeFamily::failed(),
        failed_source_reference(),
    )
    .with_presentation(blocking_presentation("Failed"))
    .with_recovery_posture(RuntimeOutcomeRecoveryPosture::retry_hint())
}

pub(crate) fn ready_projection(id: &str) -> RuntimeOutcomeProjectionDescriptor {
    RuntimeOutcomeProjectionDescriptor::new(
        projection_id(id),
        RuntimeOutcomeFamily::ready(),
        RuntimeOutcomeSourceReference::from_query_async_result_state(&async_result_state(
            ForgeQueryRuntimeAsyncResultStateKind::Current,
            "ready-causality",
        )),
    )
    .with_presentation(
        RuntimeOutcomePresentation::new()
            .with_label("Ready")
            .with_tone(RuntimeOutcomeTone::positive())
            .with_affordance(RuntimeOutcomeAffordance::none()),
    )
}

pub(crate) fn denied_source_reference() -> RuntimeOutcomeSourceReference {
    RuntimeOutcomeSourceReference::from_query_async_result_state(&async_result_state(
        ForgeQueryRuntimeAsyncResultStateKind::Denied,
        "denied-causality",
    ))
}

pub(crate) fn failed_source_reference() -> RuntimeOutcomeSourceReference {
    RuntimeOutcomeSourceReference::from_query_async_result_state(&async_result_state(
        ForgeQueryRuntimeAsyncResultStateKind::Failed,
        "failed-causality",
    ))
}

pub(crate) fn blocking_presentation(label: &str) -> RuntimeOutcomePresentation {
    RuntimeOutcomePresentation::new()
        .with_label(label)
        .with_tone(RuntimeOutcomeTone::blocking())
        .with_affordance(RuntimeOutcomeAffordance::recoverable_action())
}

pub(crate) fn projection_id(raw_text: &str) -> RuntimeOutcomeProjectionId {
    RuntimeOutcomeProjectionId::new(raw_text).expect("valid runtime outcome projection id")
}

fn async_result_state(
    kind: ForgeQueryRuntimeAsyncResultStateKind,
    causality_digest: &str,
) -> ForgeQueryRuntimeAsyncResultState {
    ForgeQueryRuntimeAsyncResultState::new(
        kind,
        &fixture_async_evidence_identity(causality_digest),
        &fixture_async_evidence_identity("basis.digest"),
        &fixture_async_evidence_identity("generation.digest"),
    )
}

fn fixture_async_evidence_identity(label: &str) -> ForgeQueryEvidenceIdentity {
    worth_ui_query_binding_evidence_identity("runtime-outcome-fixture", &[label.to_string()])
}

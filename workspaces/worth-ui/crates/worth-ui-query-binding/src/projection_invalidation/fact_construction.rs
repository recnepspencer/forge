use worth_query::facade::runtime::{
    WorthQueryAsyncResultTransitionBatch, WorthQueryRuntimeAsyncResultState,
};

use crate::{
    UiNativeTextValue, UiProjectionAvailability, UiProjectionFactReceipt, UiProjectionFactStopKind,
    UiProjectionFactStopReceipt, UiScalarProjectionFactReceipt,
};

pub(super) struct StateFactContext<'a> {
    pub(super) binding: &'a crate::UiScalarProjectionBinding,
    pub(super) batch: &'a WorthQueryAsyncResultTransitionBatch,
    pub(super) state: &'a WorthQueryRuntimeAsyncResultState,
}

pub(super) fn state_fact(
    context: StateFactContext<'_>,
    availability: UiProjectionAvailability<UiNativeTextValue>,
) -> UiScalarProjectionFactReceipt {
    UiScalarProjectionFactReceipt::admitted(fact_core(&context), availability)
}

pub(super) fn state_stop(
    context: StateFactContext<'_>,
    predecessor: Option<&UiScalarProjectionFactReceipt>,
    kind: UiProjectionFactStopKind,
    summary: impl Into<String>,
) -> UiScalarProjectionFactReceipt {
    let attempt = context.state.result_state_identity().clone();
    let summary: String = summary.into();
    let stop = UiProjectionFactStopReceipt::query_issued(
        kind,
        attempt,
        predecessor.map(predecessor_identity),
        summary,
    );
    state_fact(context, UiProjectionAvailability::Stopped(stop))
}

pub(super) fn batch_stop(
    binding: &crate::UiScalarProjectionBinding,
    batch: &WorthQueryAsyncResultTransitionBatch,
    predecessor: Option<&UiScalarProjectionFactReceipt>,
    kind: UiProjectionFactStopKind,
    summary: impl Into<String>,
) -> UiScalarProjectionFactReceipt {
    let attempt = batch
        .states()
        .first()
        .map(WorthQueryRuntimeAsyncResultState::result_state_identity)
        .unwrap_or_else(|| batch.binding_identity())
        .clone();
    let source = batch
        .states()
        .first()
        .map(WorthQueryRuntimeAsyncResultState::basis_identity)
        .unwrap_or_else(|| batch.binding_identity())
        .clone();
    let core = UiProjectionFactReceipt::admitted(
        crate::projection_consumption::UiProjectionFactReceiptInput {
            projection_identity: binding.view_identity().clone(),
            observation_order: binding.issue_observation_order(),
            query_world_identity: binding.query_world_identity().clone(),
            binding_identity: batch.binding_identity().clone(),
            source_generation_identity: source,
            result_generation_identity: attempt.clone(),
        },
    );
    let summary: String = summary.into();
    let stop = UiProjectionFactStopReceipt::query_issued(
        kind,
        attempt,
        predecessor.map(predecessor_identity),
        summary,
    );
    UiScalarProjectionFactReceipt::admitted(core, UiProjectionAvailability::Stopped(stop))
}

fn fact_core(context: &StateFactContext<'_>) -> UiProjectionFactReceipt {
    UiProjectionFactReceipt::admitted(
        crate::projection_consumption::UiProjectionFactReceiptInput {
            projection_identity: context.binding.view_identity().clone(),
            observation_order: context.binding.issue_observation_order(),
            query_world_identity: context.binding.query_world_identity().clone(),
            binding_identity: context.batch.binding_identity().clone(),
            source_generation_identity: context.state.basis_identity().clone(),
            result_generation_identity: context.state.result_state_identity().clone(),
        },
    )
}

fn predecessor_identity(
    predecessor: &UiScalarProjectionFactReceipt,
) -> worth_query::facade::runtime::WorthQueryEvidenceIdentity {
    predecessor.core().result_generation_authority().clone()
}

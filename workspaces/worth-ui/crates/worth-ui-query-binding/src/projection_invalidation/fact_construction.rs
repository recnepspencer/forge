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
        binding.query_world_identity().clone(),
        batch.binding_identity().clone(),
        source,
        attempt.clone(),
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
        context.binding.query_world_identity().clone(),
        context.batch.binding_identity().clone(),
        context.state.basis_identity().clone(),
        context.state.result_state_identity().clone(),
    )
}

fn predecessor_identity(
    predecessor: &UiScalarProjectionFactReceipt,
) -> worth_query::facade::runtime::WorthQueryEvidenceIdentity {
    predecessor.core().result_generation_identity().clone()
}

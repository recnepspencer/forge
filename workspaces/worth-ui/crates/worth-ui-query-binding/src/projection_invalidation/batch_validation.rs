use worth_query::facade::runtime::{
    WorthQueryAsyncResultTransitionBatch, WorthQueryRuntimeAsyncResultState,
};

use crate::{UiProjectionFactStopKind, UiScalarProjectionFactReceipt};

pub(super) fn validate_batch(
    binding: &mut crate::UiScalarProjectionBinding,
    batch: &WorthQueryAsyncResultTransitionBatch,
    predecessor: Option<&UiScalarProjectionFactReceipt>,
) -> Option<UiScalarProjectionFactReceipt> {
    if batch.runtime_provenance() != binding.runtime_provenance() {
        return Some(super::fact_construction::batch_stop(
            binding,
            batch,
            predecessor,
            UiProjectionFactStopKind::WrongWorld,
            "the Query result batch belongs to a different runtime authority",
        ));
    }
    if batch.view_name() != binding.view_identity().as_str() {
        return Some(super::fact_construction::batch_stop(
            binding,
            batch,
            predecessor,
            UiProjectionFactStopKind::SchemaMismatch,
            "the Query result batch targets a different installed view",
        ));
    }
    match binding.async_binding_identity() {
        Some(expected) if expected != batch.binding_identity() => {
            Some(super::fact_construction::batch_stop(
                binding,
                batch,
                predecessor,
                UiProjectionFactStopKind::StaleBindingGeneration,
                "the Query async binding identity changed without compatible replacement",
            ))
        }
        Some(_) => None,
        None => {
            binding.retain_async_binding_identity(batch.binding_identity().clone());
            None
        }
    }
}

pub(super) fn validate_predecessor_lineage(
    binding: &crate::UiScalarProjectionBinding,
    batch: &WorthQueryAsyncResultTransitionBatch,
    state: &WorthQueryRuntimeAsyncResultState,
    predecessor: Option<&UiScalarProjectionFactReceipt>,
) -> Option<UiScalarProjectionFactReceipt> {
    let predecessor = predecessor?;
    let context = super::fact_construction::StateFactContext {
        binding,
        batch,
        state,
    };
    if predecessor.core().binding_authority() != batch.binding_identity() {
        return Some(super::fact_construction::state_stop(
            context,
            Some(predecessor),
            UiProjectionFactStopKind::StaleBindingGeneration,
            "the predecessor fact belongs to a different Query async binding",
        ));
    }
    if predecessor.core().result_generation_authority() == state.result_state_identity() {
        return Some(super::fact_construction::state_stop(
            context,
            Some(predecessor),
            UiProjectionFactStopKind::StaleResultGeneration,
            "the Query result generation was already consumed",
        ));
    }
    if predecessor.core().source_generation_authority() != state.basis_identity()
        && !permits_basis_drift(state.kind())
    {
        return Some(super::fact_construction::state_stop(
            context,
            Some(predecessor),
            UiProjectionFactStopKind::BasisMismatch,
            "the Query source basis drifted without a posture that admits basis replacement",
        ));
    }
    None
}

fn permits_basis_drift(
    kind: worth_query::facade::runtime::WorthQueryRuntimeAsyncResultStateKind,
) -> bool {
    use worth_query::facade::runtime::WorthQueryRuntimeAsyncResultStateKind as Kind;
    matches!(kind, Kind::Stale | Kind::Superseded | Kind::Denied)
}

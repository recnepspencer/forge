use super::{ReconciledOperationFate, ReconciledOperationFates, RecoveryOperationFate};
use crate::ImmutablePhysicalRedoPlan;

pub fn reconcile_materialized_operation_fates(
    fates: ReconciledOperationFates,
    redo: &ImmutablePhysicalRedoPlan,
) -> ReconciledOperationFates {
    let operations = fates
        .into_operations()
        .into_vec()
        .into_iter()
        .map(|fate| promote_if_fully_materialized(fate, redo))
        .collect::<Vec<_>>();
    ReconciledOperationFates::from_operations(operations)
}

fn promote_if_fully_materialized(
    fate: ReconciledOperationFate,
    redo: &ImmutablePhysicalRedoPlan,
) -> ReconciledOperationFate {
    if fate.fate() != RecoveryOperationFate::Indeterminate {
        return fate;
    }
    let identity = fate.identity().idempotency();
    if redo.operation_group_is_fully_materialized(identity) {
        fate.with_fate(RecoveryOperationFate::DurableUnacknowledged)
    } else {
        fate
    }
}

use super::{
    ForgeQueryEphemeralGraphIndex, ForgeQueryEphemeralGraphIndexCounters,
    ForgeQueryEphemeralGraphIndexLifecycleRegistry, ForgeQueryEphemeralGraphIndexReceipt,
    ForgeQueryEphemeralGraphIndexScope, ForgeQueryEphemeralGraphIndexScopeKind,
};
use crate::runtime::ForgeQueryAdmittedGraphReadAccessPlan;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryEphemeralGraphIndexProvisioningError {
    EstimatedBytesExceedScopeBudget {
        estimated_index_bytes: usize,
        admitted_byte_budget: usize,
        counters: ForgeQueryEphemeralGraphIndexCounters,
    },
}

impl ForgeQueryEphemeralGraphIndexProvisioningError {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EstimatedBytesExceedScopeBudget { .. } => "estimated_bytes_exceed_scope_budget",
        }
    }

    pub fn estimated_index_bytes(&self) -> usize {
        match self {
            Self::EstimatedBytesExceedScopeBudget {
                estimated_index_bytes,
                ..
            } => *estimated_index_bytes,
        }
    }

    pub fn admitted_byte_budget(&self) -> usize {
        match self {
            Self::EstimatedBytesExceedScopeBudget {
                admitted_byte_budget,
                ..
            } => *admitted_byte_budget,
        }
    }

    pub fn counters(&self) -> &ForgeQueryEphemeralGraphIndexCounters {
        match self {
            Self::EstimatedBytesExceedScopeBudget { counters, .. } => counters,
        }
    }
}

pub(crate) fn provision_ephemeral_graph_indexes_for_read_execution(
    admitted_plan: &ForgeQueryAdmittedGraphReadAccessPlan,
    snapshot_identity_digest: impl Into<String>,
) -> Result<
    Option<ForgeQueryEphemeralGraphIndexReceipt>,
    ForgeQueryEphemeralGraphIndexProvisioningError,
> {
    let Some(ephemeral_plan) = admitted_plan.ephemeral_index_plan() else {
        return Ok(None);
    };
    let scope = ForgeQueryEphemeralGraphIndexScope::read_execution(
        admitted_plan.digest(),
        snapshot_identity_digest,
        ephemeral_plan.admitted_byte_budget(),
    );
    debug_assert_eq!(
        ephemeral_plan.required_scope_kind(),
        &ForgeQueryEphemeralGraphIndexScopeKind::ReadExecution
    );
    let mut lifecycle_registry = ForgeQueryEphemeralGraphIndexLifecycleRegistry::open_scope();
    if ephemeral_plan.estimated_index_bytes() > scope.byte_budget() {
        lifecycle_registry.reject_before_allocation();
        return Err(
            ForgeQueryEphemeralGraphIndexProvisioningError::EstimatedBytesExceedScopeBudget {
                estimated_index_bytes: ephemeral_plan.estimated_index_bytes(),
                admitted_byte_budget: scope.byte_budget(),
                counters: lifecycle_registry.close_scope_counters(),
            },
        );
    }
    let active_index =
        ForgeQueryEphemeralGraphIndex::build_from_plan_and_scope(ephemeral_plan, &scope);
    lifecycle_registry.register_allocation(&active_index);
    let receipt_index = active_index.clone();
    lifecycle_registry.release_index(active_index);
    let counters = lifecycle_registry.close_scope_counters();
    Ok(Some(ForgeQueryEphemeralGraphIndexReceipt::finalized(
        &receipt_index,
        &scope,
        ephemeral_plan.admitted_byte_budget(),
        lifecycle_registry.active_resource_count(),
        counters,
    )))
}

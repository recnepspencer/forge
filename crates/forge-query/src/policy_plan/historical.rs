use crate::policy_execution_seam::{
    PolicyAwareExecutionMode, PolicyAwareExecutionSeamError, PolicyAwareExecutionSeamFailureClass,
    PolicyAwareSeamCounters,
};
use crate::policy_narrowing::NarrowedPolicyQueryArtifact;

use super::{PolicyAwarePlanCore, PolicyAwarePlanCostPosture};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareHistoricalBasis {
    basis_digest: String,
    runtime_backed: bool,
}

impl PolicyAwareHistoricalBasis {
    pub fn runtime_backed(basis_digest: impl Into<String>) -> Self {
        Self {
            basis_digest: basis_digest.into(),
            runtime_backed: true,
        }
    }

    pub fn store_backed_deferred(basis_digest: impl Into<String>) -> Self {
        Self {
            basis_digest: basis_digest.into(),
            runtime_backed: false,
        }
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn is_runtime_backed(&self) -> bool {
        self.runtime_backed
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareHistoricalPlan {
    core: PolicyAwarePlanCore,
    basis: PolicyAwareHistoricalBasis,
}

impl PolicyAwareHistoricalPlan {
    pub fn core(&self) -> &PolicyAwarePlanCore {
        &self.core
    }

    pub fn basis(&self) -> &PolicyAwareHistoricalBasis {
        &self.basis
    }
}

pub fn lower_policy_aware_historical_plan(
    artifact: &NarrowedPolicyQueryArtifact,
    basis: PolicyAwareHistoricalBasis,
) -> Result<PolicyAwareHistoricalPlan, PolicyAwareExecutionSeamError> {
    if !basis.is_runtime_backed() {
        return defer_store_backed_policy_historical_plan();
    }
    Ok(PolicyAwareHistoricalPlan {
        core: PolicyAwarePlanCore::from_narrowed(
            artifact,
            PolicyAwareExecutionMode::HistoricalRead,
            PolicyAwarePlanCostPosture::RuntimeHistoricalBounded,
            artifact.authorized_projection().visible_fields().len(),
            0,
        ),
        basis,
    })
}

pub fn defer_store_backed_policy_historical_plan<T>() -> Result<T, PolicyAwareExecutionSeamError> {
    Err(PolicyAwareExecutionSeamError::new(
        PolicyAwareExecutionSeamFailureClass::StoreBackedPolicyExecutionDeferred,
        "store-backed policy-aware historical execution is deferred until Forge Store-backed parity",
        PolicyAwareSeamCounters::deferred_store_backed(),
    ))
}

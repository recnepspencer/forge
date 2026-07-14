use crate::policy_execution_seam::{
    PolicyAwareExecutionMode, PolicyAwareExecutionSeamError, PolicyAwareExecutionSeamFailureClass,
    PolicyAwareSeamCounters,
};
use crate::policy_narrowing::NarrowedPolicyQueryArtifact;

use super::{PolicyAwarePlanCore, PolicyAwarePlanCostPosture};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareHistoricalBasis {
    basis_digest: String,
    basis_class: PolicyAwareHistoricalBasisClass,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyAwareHistoricalBasisClass {
    RuntimeBacked,
    StoreBackedRetainedSnapshot,
    StoreBackedDeferred,
}

impl PolicyAwareHistoricalBasis {
    pub fn runtime_backed(basis_digest: impl Into<String>) -> Self {
        Self {
            basis_digest: basis_digest.into(),
            basis_class: PolicyAwareHistoricalBasisClass::RuntimeBacked,
        }
    }

    pub fn store_backed_retained(basis_digest: impl Into<String>) -> Self {
        Self {
            basis_digest: basis_digest.into(),
            basis_class: PolicyAwareHistoricalBasisClass::StoreBackedRetainedSnapshot,
        }
    }

    pub fn store_backed_deferred(basis_digest: impl Into<String>) -> Self {
        Self {
            basis_digest: basis_digest.into(),
            basis_class: PolicyAwareHistoricalBasisClass::StoreBackedDeferred,
        }
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn basis_class(&self) -> PolicyAwareHistoricalBasisClass {
        self.basis_class
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

pub(crate) fn lower_policy_aware_historical_plan(
    artifact: &NarrowedPolicyQueryArtifact,
    basis: PolicyAwareHistoricalBasis,
) -> Result<PolicyAwareHistoricalPlan, PolicyAwareExecutionSeamError> {
    let posture = match basis.basis_class() {
        PolicyAwareHistoricalBasisClass::RuntimeBacked => {
            PolicyAwarePlanCostPosture::RuntimeHistoricalBounded
        }
        PolicyAwareHistoricalBasisClass::StoreBackedRetainedSnapshot => {
            PolicyAwarePlanCostPosture::StoreHistoricalRetainedBounded
        }
        PolicyAwareHistoricalBasisClass::StoreBackedDeferred => {
            return defer_store_backed_policy_historical_plan();
        }
    };
    Ok(PolicyAwareHistoricalPlan {
        core: PolicyAwarePlanCore::from_narrowed(
            artifact,
            PolicyAwareExecutionMode::HistoricalRead,
            posture,
            artifact.authorized_projection().visible_field_paths().len(),
            0,
        ),
        basis,
    })
}

pub fn defer_store_backed_policy_historical_plan<T>() -> Result<T, PolicyAwareExecutionSeamError> {
    Err(PolicyAwareExecutionSeamError::new(
        PolicyAwareExecutionSeamFailureClass::StoreBackedPolicyExecutionDeferred,
        "store-backed policy-aware historical execution is deferred until Worth Store-backed parity",
        PolicyAwareSeamCounters::deferred_store_backed(),
    ))
}

#[cfg(test)]
use crate::policy_execution_seam::PolicyAwareExecutionMode;
use crate::policy_execution_seam::{
    PolicyAwareExecutionSeamError, PolicyAwareExecutionSeamFailureClass, PolicyAwareSeamCounters,
};
#[cfg(test)]
use crate::policy_narrowing::NarrowedPolicyQueryArtifact;

use super::PolicyAwarePlanCore;
#[cfg(test)]
use super::PolicyAwarePlanCostPosture;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyAwareDiffScrubDisposition {
    AuthorizedDeltaOnly,
    DeniedRawDeltaWouldLeak,
    DeferredStoreBackedHistoricalParity,
}

impl PolicyAwareDiffScrubDisposition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthorizedDeltaOnly => "authorized_delta_only",
            Self::DeniedRawDeltaWouldLeak => "denied_raw_delta_would_leak",
            Self::DeferredStoreBackedHistoricalParity => "deferred_store_backed_historical_parity",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareDiffBasisPair {
    left_basis_digest: String,
    right_basis_digest: String,
    runtime_backed: bool,
}

impl PolicyAwareDiffBasisPair {
    pub fn runtime_backed(
        left_basis_digest: impl Into<String>,
        right_basis_digest: impl Into<String>,
    ) -> Self {
        Self {
            left_basis_digest: left_basis_digest.into(),
            right_basis_digest: right_basis_digest.into(),
            runtime_backed: true,
        }
    }

    pub fn store_backed_deferred(
        left_basis_digest: impl Into<String>,
        right_basis_digest: impl Into<String>,
    ) -> Self {
        Self {
            left_basis_digest: left_basis_digest.into(),
            right_basis_digest: right_basis_digest.into(),
            runtime_backed: false,
        }
    }

    pub fn left_basis_digest(&self) -> &str {
        &self.left_basis_digest
    }

    pub fn right_basis_digest(&self) -> &str {
        &self.right_basis_digest
    }

    pub fn is_runtime_backed(&self) -> bool {
        self.runtime_backed
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareDiffPlan {
    core: PolicyAwarePlanCore,
    basis_pair: PolicyAwareDiffBasisPair,
    scrub_disposition: PolicyAwareDiffScrubDisposition,
}

impl PolicyAwareDiffPlan {
    pub fn core(&self) -> &PolicyAwarePlanCore {
        &self.core
    }

    pub fn basis_pair(&self) -> &PolicyAwareDiffBasisPair {
        &self.basis_pair
    }

    pub fn scrub_disposition(&self) -> PolicyAwareDiffScrubDisposition {
        self.scrub_disposition
    }
}
#[cfg(test)]
pub(crate) fn lower_policy_aware_diff_plan(
    artifact: &NarrowedPolicyQueryArtifact,
    basis_pair: PolicyAwareDiffBasisPair,
) -> Result<PolicyAwareDiffPlan, PolicyAwareExecutionSeamError> {
    if !basis_pair.is_runtime_backed() {
        return Err(PolicyAwareExecutionSeamError::new(
            PolicyAwareExecutionSeamFailureClass::StoreBackedPolicyExecutionDeferred,
            "store-backed policy-aware historical diff is deferred until Worth Store-backed parity",
            PolicyAwareSeamCounters::deferred_store_backed(),
        ));
    }
    Ok(PolicyAwareDiffPlan {
        core: PolicyAwarePlanCore::from_narrowed(
            artifact,
            PolicyAwareExecutionMode::HistoricalDiff,
            PolicyAwarePlanCostPosture::RuntimeDiffBounded,
            artifact.authorized_projection().visible_field_paths().len(),
            0,
        ),
        basis_pair,
        scrub_disposition: PolicyAwareDiffScrubDisposition::AuthorizedDeltaOnly,
    })
}

pub fn deny_raw_diff_scrub() -> PolicyAwareExecutionSeamError {
    PolicyAwareExecutionSeamError::new(
        PolicyAwareExecutionSeamFailureClass::RawDiffScrubForbidden,
        "policy-aware diff plans must derive authorized delta shape before raw delta computation",
        PolicyAwareSeamCounters::denied_raw_diff_scrub(),
    )
}

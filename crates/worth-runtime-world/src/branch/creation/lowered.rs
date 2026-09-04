use crate::branch::name::ProductBranchName;
use crate::branch::observation::{ProductBranchObservation, RuntimeWorldBranchAdmissionDenial};

use super::plan::{RelationalBranchCreationPlan, SignalBranchCreationPlan};
use super::ProductBranchCreationIntent;

/// Owner-admitted creation plan: the source composite basis pinned against the
/// two owner postures. Distinct from `LoweredOwnerComponentPlan`; it never
/// carries a prepared candidate or a mutation closure.
#[derive(Debug)]
#[must_use = "a lowered creation plan is executed or dropped"]
pub(crate) struct LoweredBranchCreationPlan {
    expected: ProductBranchObservation,
    name: ProductBranchName,
    relational: RelationalBranchCreationPlan,
    signal: SignalBranchCreationPlan,
}

impl LoweredBranchCreationPlan {
    pub(crate) fn lower(
        source: ProductBranchObservation,
        intent: ProductBranchCreationIntent,
    ) -> Result<Self, RuntimeWorldBranchAdmissionDenial> {
        let (name, plans) = intent.into_parts();
        let plans = plans.ok_or(RuntimeWorldBranchAdmissionDenial::PlansOmitted)?;
        let relational = plans.relational().clone();
        let signal = plans.signal().clone();
        Ok(Self {
            expected: source,
            name,
            relational,
            signal,
        })
    }

    pub(crate) const fn expected(&self) -> &ProductBranchObservation {
        &self.expected
    }

    pub(crate) const fn name(&self) -> &ProductBranchName {
        &self.name
    }

    pub(crate) const fn relational(&self) -> &RelationalBranchCreationPlan {
        &self.relational
    }

    pub(crate) const fn signal(&self) -> &SignalBranchCreationPlan {
        &self.signal
    }

    pub(crate) fn is_exact_reuse(&self) -> bool {
        self.relational.is_reuse_exact() && self.signal.is_reuse_exact()
    }
}

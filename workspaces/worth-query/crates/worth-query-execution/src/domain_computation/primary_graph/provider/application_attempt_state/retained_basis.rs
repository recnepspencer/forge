//! Narrow attempt-owned products used outside the store lock.

use super::super::WorthQueryPrimaryGraphApplicationDecisionFact;
use crate::domain_computation::WorthQueryProposedFact;

pub(in crate::domain_computation::primary_graph::provider) struct WorthQueryObservedApplicationFactBasis
{
    fact: WorthQueryPrimaryGraphApplicationDecisionFact,
    branch: worth_relational::facade::history::BranchId,
}

impl WorthQueryObservedApplicationFactBasis {
    pub(super) const fn new(
        fact: WorthQueryPrimaryGraphApplicationDecisionFact,
        branch: worth_relational::facade::history::BranchId,
    ) -> Self {
        Self { fact, branch }
    }

    pub(in crate::domain_computation::primary_graph::provider) fn remains_equal_in(
        &self,
        runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
    ) -> bool {
        let Some(snapshot) = runtime.snapshots().snapshot_for_branch(&self.branch) else {
            return false;
        };
        let fresh = self.fact.remains_equal_in(runtime, &snapshot);
        runtime.snapshots().release_snapshot(&snapshot);
        fresh
    }
}

pub(in crate::domain_computation::primary_graph::provider) struct WorthQueryApplicationIdempotencyBasis {
    binding: crate::domain_computation::primary_graph::application_attempt::WorthQueryApplicationIdempotencyBinding,
    branch: worth_relational::facade::history::BranchId,
}

impl WorthQueryApplicationIdempotencyBasis {
    pub(super) const fn new(
        binding: crate::domain_computation::primary_graph::application_attempt::WorthQueryApplicationIdempotencyBinding,
        branch: worth_relational::facade::history::BranchId,
    ) -> Self {
        Self { binding, branch }
    }

    pub(in crate::domain_computation::primary_graph::provider) fn resolve(
        self,
        provider: &super::super::WorthQueryPrimaryGraphProvider,
    ) -> Result<super::super::WorthQueryProviderIdempotencyResolution, &'static str> {
        provider.resolve_idempotency_binding(self.binding, &self.branch)
    }
}

pub(in crate::domain_computation::primary_graph::provider) struct WorthQueryAdmittedApplicationOverlay
{
    identity: String,
    facts: Vec<WorthQueryProposedFact>,
}

impl WorthQueryAdmittedApplicationOverlay {
    pub(super) const fn new(identity: String, facts: Vec<WorthQueryProposedFact>) -> Self {
        Self { identity, facts }
    }

    pub(in crate::domain_computation::primary_graph::provider) fn admit(
        self,
        admission: crate::domain_computation::WorthQueryProvisionalOverlayAdmission,
    ) -> Result<
        crate::domain_computation::WorthQueryProvisionalOverlayEvidence,
        crate::domain_computation::WorthQueryProvisionalFailure,
    > {
        admission.admit(self.identity, self.facts)
    }
}

use crate::certification::support::commit_certification_input::TopologyCommitCertificationInput;
use crate::topology_operators::TopologyMutationDerivedFallbackPolicy;

use super::super::super::mutation_sequence_support::TopologyCloseoutMutationPlan;

#[derive(Clone)]
pub(super) struct AcceptedBranchSchemaAuthorityProjection {
    commit_input: TopologyCommitCertificationInput,
    plan: TopologyCloseoutMutationPlan,
    derived_fallback_policy: TopologyMutationDerivedFallbackPolicy,
}

impl AcceptedBranchSchemaAuthorityProjection {
    pub(super) const ROW_PROJECTION_MARKER: &str = "schema_branch_authority_projection";

    pub(super) fn from_plan(
        commit_input: TopologyCommitCertificationInput,
        plan: TopologyCloseoutMutationPlan,
    ) -> Self {
        let derived_fallback_policy = if plan
            .topology_mutation_digest
            .fallback_rejection_policy_count
            > 0
        {
            TopologyMutationDerivedFallbackPolicy::RejectAnyFallback
        } else {
            TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback
        };
        Self {
            commit_input,
            plan,
            derived_fallback_policy,
        }
    }

    pub(super) fn read_basis(
        &self,
    ) -> &schema::facade::topology_authoring::DerivedTopologyReadBasis {
        self.commit_input.read_basis()
    }

    pub(super) fn plan(&self) -> &TopologyCloseoutMutationPlan {
        &self.plan
    }

    pub(super) fn derived_fallback_policy(&self) -> TopologyMutationDerivedFallbackPolicy {
        self.derived_fallback_policy
    }
}

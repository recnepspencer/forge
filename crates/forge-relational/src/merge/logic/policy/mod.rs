mod contexts;
mod decisions;
mod resolution;
#[cfg(test)]
mod tests;
mod value_strategy;

use crate::logic::runtime::RelationalRuntime;
use crate::merge::data::{
    CausallyAnnotatedMergePlan, MergePlanningError, MergePlanningRequest, PolicyResolvedMergePlan,
};
use crate::merge::logic::MergeAccess;

pub(crate) use decisions::current_topology_rewire_admission_policy;

impl<'runtime> MergeAccess<'runtime> {
    pub(crate) fn plan_policy_scope(
        &self,
        request: MergePlanningRequest,
    ) -> Result<PolicyResolvedMergePlan, MergePlanningError> {
        let causal_plan = self.plan_causal_scope(request)?;
        self.resolve_policy_scope(causal_plan)
    }

    fn resolve_policy_scope(
        &self,
        causal_plan: CausallyAnnotatedMergePlan,
    ) -> Result<PolicyResolvedMergePlan, MergePlanningError> {
        resolution::resolve_policy_scope(self.runtime, causal_plan)
    }
}

fn binding_matches_aspect(
    runtime: &RelationalRuntime,
    binding: &crate::schema::data::LoweredAspectBinding,
    aspect_key: &forge_foundational::facade::AspectKey,
) -> bool {
    aspect_key_equivalent(runtime, &binding.aspect_key, aspect_key)
}

fn aspect_key_equivalent(
    _runtime: &RelationalRuntime,
    left: &forge_foundational::facade::AspectKey,
    right: &forge_foundational::facade::AspectKey,
) -> bool {
    left == right
}

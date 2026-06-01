use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::{InvariantCostClass, InvariantGroupSet, InvariantPlanContract};
use crate::validation::engine::{
    InvariantExecutionDisposition, InvariantExecutionMetadata, InvariantObservationKind,
    InvariantRequestProfile,
};
use crate::validation::logic::invariant_access::InvariantAccess;

impl<'runtime> InvariantAccess<'runtime> {
    pub(super) fn execution_metadata(
        &self,
        profile: InvariantRequestProfile,
        observation_kind: InvariantObservationKind,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'runtime MergedCommitPlan>,
        plan_contract: Option<InvariantPlanContract>,
        applicable_groups: InvariantGroupSet,
        max_cost: InvariantCostClass,
        disposition: InvariantExecutionDisposition,
    ) -> InvariantExecutionMetadata {
        InvariantExecutionMetadata::new(
            profile.execution_point(),
            observation_kind,
            version_id,
            self.runtime.current_version_id(),
            profile.consumed_groups(),
            applicable_groups,
            max_cost,
            disposition,
            plan_contract,
            merged_plan.is_some(),
            self.runtime.config.execution.execution_model,
            None,
            Vec::new(),
            None,
        )
    }
}

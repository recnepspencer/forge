use crate::replay_family_catalog::{
    SpatialReplayFamilyCoveredLookupIdentity, SpatialReplayFamilyIdentity,
    SpatialReplayFamilyScopeProductPosture, SpatialReplayFamilyWorkloadDependencyPosture,
};
use crate::replay_undo_semantic_graph::SpatialReplaySemanticGraphAdmittedInput;

#[derive(Debug)]
pub struct SpatialReplaySelectedPlan<'a> {
    family_identity: SpatialReplayFamilyIdentity,
    admitted_input: &'a SpatialReplaySemanticGraphAdmittedInput<'a>,
    admitted_input_semantic_graph_identity: String,
    lookup_consumed_workload_handoff_identity: String,
    retained_replay_receipt_identity: Option<String>,
    covered_lookup_identity: SpatialReplayFamilyCoveredLookupIdentity,
    workload_dependency_posture: SpatialReplayFamilyWorkloadDependencyPosture,
    scope_product_posture: SpatialReplayFamilyScopeProductPosture,
    selected_plan_identity: String,
}

impl<'a> SpatialReplaySelectedPlan<'a> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        family_identity: SpatialReplayFamilyIdentity,
        admitted_input: &'a SpatialReplaySemanticGraphAdmittedInput<'a>,
        admitted_input_semantic_graph_identity: String,
        lookup_consumed_workload_handoff_identity: String,
        retained_replay_receipt_identity: Option<String>,
        covered_lookup_identity: SpatialReplayFamilyCoveredLookupIdentity,
        workload_dependency_posture: SpatialReplayFamilyWorkloadDependencyPosture,
        scope_product_posture: SpatialReplayFamilyScopeProductPosture,
        selected_plan_identity: String,
    ) -> Self {
        Self {
            family_identity,
            admitted_input,
            admitted_input_semantic_graph_identity,
            lookup_consumed_workload_handoff_identity,
            retained_replay_receipt_identity,
            covered_lookup_identity,
            workload_dependency_posture,
            scope_product_posture,
            selected_plan_identity,
        }
    }

    pub const fn family_identity(&self) -> SpatialReplayFamilyIdentity {
        self.family_identity
    }

    pub const fn admitted_input(&self) -> &'a SpatialReplaySemanticGraphAdmittedInput<'a> {
        self.admitted_input
    }

    pub fn admitted_input_semantic_graph_identity(&self) -> &str {
        &self.admitted_input_semantic_graph_identity
    }

    pub fn lookup_consumed_workload_handoff_identity(&self) -> &str {
        &self.lookup_consumed_workload_handoff_identity
    }

    pub fn retained_replay_receipt_identity(&self) -> Option<&str> {
        self.retained_replay_receipt_identity.as_deref()
    }

    pub const fn covered_lookup_identity(&self) -> SpatialReplayFamilyCoveredLookupIdentity {
        self.covered_lookup_identity
    }

    pub const fn workload_dependency_posture(
        &self,
    ) -> SpatialReplayFamilyWorkloadDependencyPosture {
        self.workload_dependency_posture
    }

    pub const fn scope_product_posture(&self) -> SpatialReplayFamilyScopeProductPosture {
        self.scope_product_posture
    }

    pub fn selected_plan_identity(&self) -> &str {
        &self.selected_plan_identity
    }

    pub fn requires_retained_replay_binding(&self) -> bool {
        matches!(
            self.workload_dependency_posture,
            SpatialReplayFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkloadAndRetainedReplay
        )
    }
}

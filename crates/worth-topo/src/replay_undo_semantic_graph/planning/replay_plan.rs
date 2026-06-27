use crate::replay_family_catalog::{
    TopologyReplayFamilyIdentity, TopologyReplayFamilyScopeProductPosture,
};
use crate::replay_undo_semantic_graph::TopologyReplaySemanticGraphAdmittedInput;

#[derive(Debug)]
pub struct TopologyReplaySelectedPlan<'a> {
    family_identity: TopologyReplayFamilyIdentity,
    admitted_input: &'a TopologyReplaySemanticGraphAdmittedInput<'a>,
    scope_product_posture: TopologyReplayFamilyScopeProductPosture,
}

impl<'a> TopologyReplaySelectedPlan<'a> {
    pub(crate) fn new(
        family_identity: TopologyReplayFamilyIdentity,
        admitted_input: &'a TopologyReplaySemanticGraphAdmittedInput<'a>,
        scope_product_posture: TopologyReplayFamilyScopeProductPosture,
    ) -> Self {
        Self {
            family_identity,
            admitted_input,
            scope_product_posture,
        }
    }

    pub const fn family_identity(&self) -> TopologyReplayFamilyIdentity {
        self.family_identity
    }

    pub const fn admitted_input(&self) -> &'a TopologyReplaySemanticGraphAdmittedInput<'a> {
        self.admitted_input
    }

    pub const fn scope_product_posture(&self) -> TopologyReplayFamilyScopeProductPosture {
        self.scope_product_posture
    }

    pub const fn selected_plan_identity(
        &self,
    ) -> &crate::replay_undo_semantic_graph::TopologyReplaySemanticGraphSelectedPlanIdentity {
        self.admitted_input.selected_plan_identity()
    }

    pub const fn stage_identity(
        &self,
    ) -> &crate::replay_undo_semantic_graph::TopologyReplaySemanticGraphStageIdentity {
        self.admitted_input.stage_identity()
    }
}

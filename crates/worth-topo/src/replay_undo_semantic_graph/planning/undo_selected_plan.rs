use crate::replay_undo_semantic_graph::TopologyUndoSemanticGraphAdmittedInput;
use crate::undo_family_catalog::{
    TopologyUndoFamilyIdentity, TopologyUndoFamilyScopeProductPosture,
};

#[derive(Debug)]
pub struct TopologyUndoSelectedPlan<'a> {
    family_identity: TopologyUndoFamilyIdentity,
    admitted_input: &'a TopologyUndoSemanticGraphAdmittedInput<'a>,
    scope_product_posture: TopologyUndoFamilyScopeProductPosture,
}

impl<'a> TopologyUndoSelectedPlan<'a> {
    pub(crate) fn new(
        family_identity: TopologyUndoFamilyIdentity,
        admitted_input: &'a TopologyUndoSemanticGraphAdmittedInput<'a>,
        scope_product_posture: TopologyUndoFamilyScopeProductPosture,
    ) -> Self {
        Self {
            family_identity,
            admitted_input,
            scope_product_posture,
        }
    }

    pub const fn family_identity(&self) -> TopologyUndoFamilyIdentity {
        self.family_identity
    }

    pub const fn admitted_input(&self) -> &'a TopologyUndoSemanticGraphAdmittedInput<'a> {
        self.admitted_input
    }

    pub const fn scope_product_posture(&self) -> TopologyUndoFamilyScopeProductPosture {
        self.scope_product_posture
    }
}

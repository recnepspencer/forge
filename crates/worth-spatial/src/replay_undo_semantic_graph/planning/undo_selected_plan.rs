use crate::replay_undo_semantic_graph::SpatialUndoSemanticGraphAdmittedInput;
use crate::undo_family_catalog::{
    SpatialUndoFamilyIdentity, SpatialUndoFamilyScopeProductPosture,
    SpatialUndoFamilyWorkloadDependencyPosture,
};

#[derive(Debug)]
pub struct SpatialUndoSelectedPlan<'a> {
    family_identity: SpatialUndoFamilyIdentity,
    admitted_input: &'a SpatialUndoSemanticGraphAdmittedInput<'a>,
    workload_dependency_posture: SpatialUndoFamilyWorkloadDependencyPosture,
    scope_product_posture: SpatialUndoFamilyScopeProductPosture,
}

impl<'a> SpatialUndoSelectedPlan<'a> {
    pub(crate) fn new(
        family_identity: SpatialUndoFamilyIdentity,
        admitted_input: &'a SpatialUndoSemanticGraphAdmittedInput<'a>,
        workload_dependency_posture: SpatialUndoFamilyWorkloadDependencyPosture,
        scope_product_posture: SpatialUndoFamilyScopeProductPosture,
    ) -> Self {
        Self {
            family_identity,
            admitted_input,
            workload_dependency_posture,
            scope_product_posture,
        }
    }

    pub const fn family_identity(&self) -> SpatialUndoFamilyIdentity {
        self.family_identity
    }

    pub const fn admitted_input(&self) -> &'a SpatialUndoSemanticGraphAdmittedInput<'a> {
        self.admitted_input
    }

    pub const fn workload_dependency_posture(&self) -> SpatialUndoFamilyWorkloadDependencyPosture {
        self.workload_dependency_posture
    }

    pub const fn scope_product_posture(&self) -> SpatialUndoFamilyScopeProductPosture {
        self.scope_product_posture
    }
}

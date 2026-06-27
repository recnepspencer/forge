use super::family_identity::TopologyUndoFamilyIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyUndoFamilyLocalityPosture {
    RequiresTouchedClosure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyUndoFamilyPriorProofPosture {
    RequiresInvalidationExecutionReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyUndoFamilyStageIndexPosture {
    RequiresStageIndexIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyUndoFamilyWorkloadDependencyPosture {
    TopologyOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyUndoFamilyScopeProductPosture {
    RequiresTopologyUndoScopeProduct,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyUndoFamilyDeclaration {
    identity: TopologyUndoFamilyIdentity,
    locality_posture: TopologyUndoFamilyLocalityPosture,
    prior_proof_posture: TopologyUndoFamilyPriorProofPosture,
    stage_index_posture: TopologyUndoFamilyStageIndexPosture,
    workload_dependency_posture: TopologyUndoFamilyWorkloadDependencyPosture,
    scope_product_posture: TopologyUndoFamilyScopeProductPosture,
}

impl TopologyUndoFamilyDeclaration {
    pub(crate) fn new(
        identity: TopologyUndoFamilyIdentity,
        locality_posture: TopologyUndoFamilyLocalityPosture,
        prior_proof_posture: TopologyUndoFamilyPriorProofPosture,
        stage_index_posture: TopologyUndoFamilyStageIndexPosture,
        workload_dependency_posture: TopologyUndoFamilyWorkloadDependencyPosture,
        scope_product_posture: TopologyUndoFamilyScopeProductPosture,
    ) -> Self {
        Self {
            identity,
            locality_posture,
            prior_proof_posture,
            stage_index_posture,
            workload_dependency_posture,
            scope_product_posture,
        }
    }

    pub const fn identity(&self) -> TopologyUndoFamilyIdentity {
        self.identity
    }

    pub const fn locality_posture(&self) -> TopologyUndoFamilyLocalityPosture {
        self.locality_posture
    }

    pub const fn prior_proof_posture(&self) -> TopologyUndoFamilyPriorProofPosture {
        self.prior_proof_posture
    }

    pub const fn stage_index_posture(&self) -> TopologyUndoFamilyStageIndexPosture {
        self.stage_index_posture
    }

    pub const fn workload_dependency_posture(&self) -> TopologyUndoFamilyWorkloadDependencyPosture {
        self.workload_dependency_posture
    }

    pub const fn scope_product_posture(&self) -> TopologyUndoFamilyScopeProductPosture {
        self.scope_product_posture
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyUndoFamilyCatalog {
    declarations: Vec<TopologyUndoFamilyDeclaration>,
}

impl TopologyUndoFamilyCatalog {
    pub(crate) fn new(declarations: Vec<TopologyUndoFamilyDeclaration>) -> Self {
        Self { declarations }
    }

    pub fn declarations(&self) -> &[TopologyUndoFamilyDeclaration] {
        &self.declarations
    }

    pub fn require_family(
        &self,
        identity: TopologyUndoFamilyIdentity,
    ) -> Option<&TopologyUndoFamilyDeclaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.identity() == identity)
    }
}

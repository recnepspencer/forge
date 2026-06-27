use super::family_identity::TopologyReplayFamilyIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyReplayFamilyLocalityPosture {
    RequiresTouchedClosure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyReplayFamilyPriorProofPosture {
    RequiresInvalidationSelectedPlanAndExecutionReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyReplayFamilyStageIndexPosture {
    RequiresStageIndexIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyReplayFamilyWorkloadDependencyPosture {
    TopologyOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyReplayFamilyScopeProductPosture {
    RequiresTopologyReplayScopeProduct,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyReplayFamilyDeclaration {
    identity: TopologyReplayFamilyIdentity,
    locality_posture: TopologyReplayFamilyLocalityPosture,
    prior_proof_posture: TopologyReplayFamilyPriorProofPosture,
    stage_index_posture: TopologyReplayFamilyStageIndexPosture,
    workload_dependency_posture: TopologyReplayFamilyWorkloadDependencyPosture,
    scope_product_posture: TopologyReplayFamilyScopeProductPosture,
}

impl TopologyReplayFamilyDeclaration {
    pub(crate) fn new(
        identity: TopologyReplayFamilyIdentity,
        locality_posture: TopologyReplayFamilyLocalityPosture,
        prior_proof_posture: TopologyReplayFamilyPriorProofPosture,
        stage_index_posture: TopologyReplayFamilyStageIndexPosture,
        workload_dependency_posture: TopologyReplayFamilyWorkloadDependencyPosture,
        scope_product_posture: TopologyReplayFamilyScopeProductPosture,
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

    pub const fn identity(&self) -> TopologyReplayFamilyIdentity {
        self.identity
    }

    pub const fn locality_posture(&self) -> TopologyReplayFamilyLocalityPosture {
        self.locality_posture
    }

    pub const fn prior_proof_posture(&self) -> TopologyReplayFamilyPriorProofPosture {
        self.prior_proof_posture
    }

    pub const fn stage_index_posture(&self) -> TopologyReplayFamilyStageIndexPosture {
        self.stage_index_posture
    }

    pub const fn workload_dependency_posture(
        &self,
    ) -> TopologyReplayFamilyWorkloadDependencyPosture {
        self.workload_dependency_posture
    }

    pub const fn scope_product_posture(&self) -> TopologyReplayFamilyScopeProductPosture {
        self.scope_product_posture
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyReplayFamilyCatalog {
    declarations: Vec<TopologyReplayFamilyDeclaration>,
}

impl TopologyReplayFamilyCatalog {
    pub(crate) fn new(declarations: Vec<TopologyReplayFamilyDeclaration>) -> Self {
        Self { declarations }
    }

    pub fn declarations(&self) -> &[TopologyReplayFamilyDeclaration] {
        &self.declarations
    }

    pub fn require_family(
        &self,
        identity: TopologyReplayFamilyIdentity,
    ) -> Option<&TopologyReplayFamilyDeclaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.identity() == identity)
    }
}

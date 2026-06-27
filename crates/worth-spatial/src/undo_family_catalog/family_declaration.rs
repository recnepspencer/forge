use super::family_identity::SpatialUndoFamilyIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialUndoFamilyLocalityPosture {
    RequiresSpatialTouchAuthority,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialUndoFamilyPriorProofPosture {
    RequiresEvidenceLookupExecutionReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialUndoFamilyStageIndexPosture {
    RequiresStageIndexIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialUndoFamilyWorkloadDependencyPosture {
    LookupReceiptOnly,
    RequiresLookupConsumedWorkload,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialUndoFamilyScopeProductPosture {
    RequiresSpatialUndoScopeProduct,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialUndoFamilyDeclaration {
    identity: SpatialUndoFamilyIdentity,
    locality_posture: SpatialUndoFamilyLocalityPosture,
    prior_proof_posture: SpatialUndoFamilyPriorProofPosture,
    stage_index_posture: SpatialUndoFamilyStageIndexPosture,
    workload_dependency_posture: SpatialUndoFamilyWorkloadDependencyPosture,
    scope_product_posture: SpatialUndoFamilyScopeProductPosture,
}

impl SpatialUndoFamilyDeclaration {
    pub(crate) fn new(
        identity: SpatialUndoFamilyIdentity,
        locality_posture: SpatialUndoFamilyLocalityPosture,
        prior_proof_posture: SpatialUndoFamilyPriorProofPosture,
        stage_index_posture: SpatialUndoFamilyStageIndexPosture,
        workload_dependency_posture: SpatialUndoFamilyWorkloadDependencyPosture,
        scope_product_posture: SpatialUndoFamilyScopeProductPosture,
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

    pub const fn identity(&self) -> SpatialUndoFamilyIdentity {
        self.identity
    }

    pub const fn locality_posture(&self) -> SpatialUndoFamilyLocalityPosture {
        self.locality_posture
    }

    pub const fn prior_proof_posture(&self) -> SpatialUndoFamilyPriorProofPosture {
        self.prior_proof_posture
    }

    pub const fn stage_index_posture(&self) -> SpatialUndoFamilyStageIndexPosture {
        self.stage_index_posture
    }

    pub const fn workload_dependency_posture(&self) -> SpatialUndoFamilyWorkloadDependencyPosture {
        self.workload_dependency_posture
    }

    pub const fn scope_product_posture(&self) -> SpatialUndoFamilyScopeProductPosture {
        self.scope_product_posture
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialUndoFamilyCatalog {
    declarations: Vec<SpatialUndoFamilyDeclaration>,
}

impl SpatialUndoFamilyCatalog {
    pub(crate) fn new(declarations: Vec<SpatialUndoFamilyDeclaration>) -> Self {
        Self { declarations }
    }

    pub fn declarations(&self) -> &[SpatialUndoFamilyDeclaration] {
        &self.declarations
    }

    pub fn require_family(
        &self,
        identity: SpatialUndoFamilyIdentity,
    ) -> Option<&SpatialUndoFamilyDeclaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.identity() == identity)
    }
}

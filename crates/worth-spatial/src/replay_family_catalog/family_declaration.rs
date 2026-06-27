use super::family_identity::SpatialReplayFamilyIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialReplayFamilyLocalityPosture {
    RequiresSpatialTouchAuthority,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialReplayFamilyPriorProofPosture {
    RequiresEvidenceLookupExecutionReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialReplayFamilyStageIndexPosture {
    RequiresStageIndexIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialReplayFamilyWorkloadDependencyPosture {
    LookupReceiptOnly,
    RequiresLookupConsumedWorkloadAndRetainedReplay,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialReplayFamilyCoveredLookupIdentity {
    BooleanEventLedgerEvidence,
    ProjectionConsumptionEvidence,
}

impl SpatialReplayFamilyCoveredLookupIdentity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BooleanEventLedgerEvidence => "spatial-touch.boolean.event-ledger-evidence.v1",
            Self::ProjectionConsumptionEvidence => {
                "spatial-touch.boolean.projection-consumption-evidence.v1"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialReplayFamilyScopeProductPosture {
    RequiresSpatialReplayScopeProduct,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialReplayFamilyDeclaration {
    identity: SpatialReplayFamilyIdentity,
    locality_posture: SpatialReplayFamilyLocalityPosture,
    prior_proof_posture: SpatialReplayFamilyPriorProofPosture,
    stage_index_posture: SpatialReplayFamilyStageIndexPosture,
    covered_lookup_identity: SpatialReplayFamilyCoveredLookupIdentity,
    workload_dependency_posture: SpatialReplayFamilyWorkloadDependencyPosture,
    scope_product_posture: SpatialReplayFamilyScopeProductPosture,
}

impl SpatialReplayFamilyDeclaration {
    pub(crate) fn new(
        identity: SpatialReplayFamilyIdentity,
        locality_posture: SpatialReplayFamilyLocalityPosture,
        prior_proof_posture: SpatialReplayFamilyPriorProofPosture,
        stage_index_posture: SpatialReplayFamilyStageIndexPosture,
        covered_lookup_identity: SpatialReplayFamilyCoveredLookupIdentity,
        workload_dependency_posture: SpatialReplayFamilyWorkloadDependencyPosture,
        scope_product_posture: SpatialReplayFamilyScopeProductPosture,
    ) -> Self {
        Self {
            identity,
            locality_posture,
            prior_proof_posture,
            stage_index_posture,
            covered_lookup_identity,
            workload_dependency_posture,
            scope_product_posture,
        }
    }

    pub const fn identity(&self) -> SpatialReplayFamilyIdentity {
        self.identity
    }

    pub const fn locality_posture(&self) -> SpatialReplayFamilyLocalityPosture {
        self.locality_posture
    }

    pub const fn prior_proof_posture(&self) -> SpatialReplayFamilyPriorProofPosture {
        self.prior_proof_posture
    }

    pub const fn stage_index_posture(&self) -> SpatialReplayFamilyStageIndexPosture {
        self.stage_index_posture
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialReplayFamilyCatalog {
    declarations: Vec<SpatialReplayFamilyDeclaration>,
}

impl SpatialReplayFamilyCatalog {
    pub(crate) fn new(declarations: Vec<SpatialReplayFamilyDeclaration>) -> Self {
        Self { declarations }
    }

    pub fn declarations(&self) -> &[SpatialReplayFamilyDeclaration] {
        &self.declarations
    }

    pub fn require_family(
        &self,
        identity: SpatialReplayFamilyIdentity,
    ) -> Option<&SpatialReplayFamilyDeclaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.identity() == identity)
    }
}

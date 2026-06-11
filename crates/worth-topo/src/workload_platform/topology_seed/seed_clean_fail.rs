use super::{
    TopologySeedCounters, TopologySeedEntityIdentities, TopologySeedKind,
    TopologySeedQueryReceipts, TopologySeedTopologyPosture,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologySeedCleanFailStage {
    ParameterAdmission,
    TopologyValidation,
    SpatialBindingAdmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologySeedCleanFailClass {
    UnsupportedSeedParameter,
    DirtyTopology,
    InvalidTopology,
    WorkloadDeclaration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologySeedCleanFailReasonCode {
    SingleFaceLoopEdgeCountOutOfRange,
    MultiFaceShellFaceCountOutOfRange,
    SelfIntersectingLoopRequiresSpatialPolicy,
    NonManifoldWireCannotBindAsGeometry,
    TopologyValidationRejectedSeed,
    WorkloadDeclarationRejectedSeed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySeedCleanFailReceipt {
    kind: TopologySeedKind,
    stage: TopologySeedCleanFailStage,
    class: TopologySeedCleanFailClass,
    reason_code: TopologySeedCleanFailReasonCode,
    topology_posture: TopologySeedTopologyPosture,
    reason: String,
    query_receipts: Option<TopologySeedQueryReceipts>,
    entity_identities: Option<TopologySeedEntityIdentities>,
    counters: Option<TopologySeedCounters>,
}

impl TopologySeedCleanFailReceipt {
    pub(crate) fn new(
        kind: TopologySeedKind,
        stage: TopologySeedCleanFailStage,
        class: TopologySeedCleanFailClass,
        reason_code: TopologySeedCleanFailReasonCode,
        reason: impl Into<String>,
        query_receipts: Option<TopologySeedQueryReceipts>,
        entity_identities: Option<TopologySeedEntityIdentities>,
        counters: Option<TopologySeedCounters>,
    ) -> Self {
        Self {
            kind,
            stage,
            class,
            reason_code,
            topology_posture: kind.topology_posture(),
            reason: normalize_reason(reason),
            query_receipts,
            entity_identities,
            counters,
        }
    }

    pub fn kind(&self) -> TopologySeedKind {
        self.kind
    }

    pub fn stage(&self) -> TopologySeedCleanFailStage {
        self.stage
    }

    pub fn class(&self) -> TopologySeedCleanFailClass {
        self.class
    }

    pub fn topology_posture(&self) -> TopologySeedTopologyPosture {
        self.topology_posture
    }

    pub fn reason_code(&self) -> TopologySeedCleanFailReasonCode {
        self.reason_code
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn query_receipts(&self) -> Option<&TopologySeedQueryReceipts> {
        self.query_receipts.as_ref()
    }

    pub fn entity_identities(&self) -> Option<&TopologySeedEntityIdentities> {
        self.entity_identities.as_ref()
    }

    pub fn counters(&self) -> Option<TopologySeedCounters> {
        self.counters
    }

    pub fn can_enter_spatial_binding(&self) -> bool {
        false
    }
}

fn normalize_reason(reason: impl Into<String>) -> String {
    let reason = reason.into();
    if reason.trim().is_empty() {
        "topology seed clean-fail receipts require a human-readable reason".to_string()
    } else {
        reason
    }
}

use super::{TopologyBindingTarget, UnsupportedGeometryCarrierFamily};
use crate::workload_platform::vocabulary::{
    SpatialWorkloadStage, WorkloadStagePosture, WorkloadStageSupport,
};
use topology::facade::{
    TopologySeedCleanFailReceipt, TopologySeedCleanFailStage, TopologySeedTopologyPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsupportedGeometryBindingReasonCode {
    DirtyTopology,
    MissingBindingDeclaration,
    MissingGeometryCarrier,
    MismatchedCarrierTarget,
    UnsupportedCarrierFamily,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsupportedGeometryBinding {
    reason_code: UnsupportedGeometryBindingReasonCode,
    human_reason: String,
    topology_receipt_identity: Option<String>,
    topology_query_surface: Option<String>,
    binding_declaration: String,
    requested_unsupported_family: Option<UnsupportedGeometryCarrierFamily>,
    posture: WorkloadStagePosture,
}

impl UnsupportedGeometryBinding {
    pub(crate) fn new(
        reason_code: UnsupportedGeometryBindingReasonCode,
        human_reason: impl Into<String>,
        topology_receipt_identity: Option<String>,
        topology_query_surface: Option<String>,
        binding_declaration: impl Into<String>,
        requested_unsupported_family: Option<UnsupportedGeometryCarrierFamily>,
    ) -> Self {
        let human_reason = normalize_reason(human_reason);
        Self {
            reason_code,
            topology_receipt_identity,
            topology_query_surface,
            binding_declaration: binding_declaration.into(),
            requested_unsupported_family,
            posture: WorkloadStagePosture::new(
                SpatialWorkloadStage::GeometryBinding,
                WorkloadStageSupport::Unsupported,
                human_reason.clone(),
            ),
            human_reason,
        }
    }

    pub(crate) fn from_target(
        target: &TopologyBindingTarget,
        declaration: impl Into<String>,
        reason_code: UnsupportedGeometryBindingReasonCode,
        human_reason: impl Into<String>,
    ) -> Self {
        Self::from_target_with_requested_family(
            target,
            declaration,
            reason_code,
            human_reason,
            None,
        )
    }

    pub(crate) fn from_target_with_requested_family(
        target: &TopologyBindingTarget,
        declaration: impl Into<String>,
        reason_code: UnsupportedGeometryBindingReasonCode,
        human_reason: impl Into<String>,
        requested_unsupported_family: Option<UnsupportedGeometryCarrierFamily>,
    ) -> Self {
        Self::new(
            reason_code,
            human_reason,
            Some(target.topology_receipt_identity().to_string()),
            Some(target.topology_query_surface().to_string()),
            declaration,
            requested_unsupported_family,
        )
    }

    pub fn from_topology_clean_fail(clean_fail: &TopologySeedCleanFailReceipt) -> Self {
        let topology_receipt_identity = clean_fail
            .query_receipts()
            .map(|receipts| receipts.declaration_receipt().identity().name().to_string());
        let topology_query_surface = clean_fail
            .query_receipts()
            .map(|receipts| receipts.query_surface().to_string());
        Self::new(
            UnsupportedGeometryBindingReasonCode::DirtyTopology,
            dirty_reason(clean_fail),
            topology_receipt_identity,
            topology_query_surface,
            format!(
                "deny geometry binding for topology clean-fail {}",
                clean_fail.kind().as_str()
            ),
            None,
        )
    }

    pub fn reason_code(&self) -> UnsupportedGeometryBindingReasonCode {
        self.reason_code
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub fn topology_receipt_identity(&self) -> Option<&str> {
        self.topology_receipt_identity.as_deref()
    }

    pub fn topology_query_surface(&self) -> Option<&str> {
        self.topology_query_surface.as_deref()
    }

    pub fn binding_declaration(&self) -> &str {
        &self.binding_declaration
    }

    pub fn requested_unsupported_family(&self) -> Option<UnsupportedGeometryCarrierFamily> {
        self.requested_unsupported_family
    }

    pub fn posture(&self) -> &WorkloadStagePosture {
        &self.posture
    }

    pub fn can_enter_surface_support(&self) -> bool {
        false
    }
}

fn dirty_reason(clean_fail: &TopologySeedCleanFailReceipt) -> &'static str {
    match (
        clean_fail.stage(),
        clean_fail.topology_posture(),
        clean_fail.can_enter_spatial_binding(),
    ) {
        (TopologySeedCleanFailStage::SpatialBindingAdmission, _, false) => {
            "Topology clean-fail receipt explicitly denies spatial binding."
        }
        (_, TopologySeedTopologyPosture::Dirty, false) => {
            "Topology failed seed validation, so geometry binding was denied."
        }
        _ => "Topology did not produce an admitted seed receipt, so geometry binding was denied.",
    }
}

fn normalize_reason(reason: impl Into<String>) -> String {
    let reason = reason.into();
    if reason.trim().is_empty() {
        "Geometry binding was denied before an admitted spatial workload could be built."
            .to_string()
    } else {
        reason
    }
}

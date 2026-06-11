use super::{SurfaceFamily, SurfaceSupportMatrixRow, UnsupportedSurfaceSupportReceipt};
use crate::workload_platform::vocabulary::{
    SpatialWorkloadStage, WorkloadStagePosture, WorkloadStageSupport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsupportedSurfaceSupportReasonCode {
    MissingDeclaration,
    MissingSurfaceFamily,
    MissingGeometryBindingReceipt,
    FamilyNotAdmitted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsupportedSurfaceSupport {
    family: Option<SurfaceFamily>,
    reason_code: UnsupportedSurfaceSupportReasonCode,
    human_reason: String,
    upstream_geometry_binding_identity: Option<String>,
    topology_query_surface: Option<String>,
    matrix_rows: Vec<SurfaceSupportMatrixRow>,
    receipt: Option<UnsupportedSurfaceSupportReceipt>,
    posture: WorkloadStagePosture,
}

impl UnsupportedSurfaceSupport {
    pub(crate) fn new(
        family: Option<SurfaceFamily>,
        reason_code: UnsupportedSurfaceSupportReasonCode,
        human_reason: impl Into<String>,
        upstream_geometry_binding_identity: Option<String>,
        topology_query_surface: Option<String>,
        matrix_rows: Vec<SurfaceSupportMatrixRow>,
        receipt: Option<UnsupportedSurfaceSupportReceipt>,
    ) -> Self {
        let human_reason = normalize_reason(human_reason);
        Self {
            family,
            reason_code,
            upstream_geometry_binding_identity,
            topology_query_surface,
            matrix_rows,
            receipt,
            posture: WorkloadStagePosture::new(
                SpatialWorkloadStage::SurfaceSupport,
                WorkloadStageSupport::Unsupported,
                human_reason.clone(),
            ),
            human_reason,
        }
    }

    pub fn family(&self) -> Option<SurfaceFamily> {
        self.family
    }

    pub fn reason_code(&self) -> UnsupportedSurfaceSupportReasonCode {
        self.reason_code
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub fn upstream_geometry_binding_identity(&self) -> Option<&str> {
        self.upstream_geometry_binding_identity.as_deref()
    }

    pub fn topology_query_surface(&self) -> Option<&str> {
        self.topology_query_surface.as_deref()
    }

    pub fn matrix_rows(&self) -> &[SurfaceSupportMatrixRow] {
        &self.matrix_rows
    }

    pub fn receipt(&self) -> Option<&UnsupportedSurfaceSupportReceipt> {
        self.receipt.as_ref()
    }

    pub fn posture(&self) -> &WorkloadStagePosture {
        &self.posture
    }

    pub fn can_enter_local_frame_workload(&self) -> bool {
        false
    }

    pub fn can_enter_projection_workload(&self) -> bool {
        false
    }

    pub fn can_enter_operator_execution(&self) -> bool {
        false
    }
}

fn normalize_reason(reason: impl Into<String>) -> String {
    let reason = reason.into();
    if reason.trim().is_empty() {
        "Surface support was denied before a certified surface workload could be built.".to_string()
    } else {
        reason
    }
}

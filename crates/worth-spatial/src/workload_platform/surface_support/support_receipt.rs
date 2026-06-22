use super::{
    SurfaceFamily, SurfaceSupportMatrixRow, SurfaceSupportStatus,
    UnsupportedSurfaceSupportReasonCode,
};
use crate::workload_platform::vocabulary::{
    SpatialWorkloadStage, SurfaceSupportWorkloadReceipt, WorkloadStageEnvelope,
    WorkloadStageIdentity, WorkloadStagePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SurfaceSupportCounters {
    classified_families: usize,
    certified_planes: usize,
    unsupported_families: usize,
    upstream_geometry_carriers: usize,
}

impl SurfaceSupportCounters {
    pub(crate) fn new(
        classified_families: usize,
        certified_planes: usize,
        unsupported_families: usize,
        upstream_geometry_carriers: usize,
    ) -> Self {
        Self {
            classified_families,
            certified_planes,
            unsupported_families,
            upstream_geometry_carriers,
        }
    }

    pub fn classified_families(self) -> usize {
        self.classified_families
    }

    pub fn certified_planes(self) -> usize {
        self.certified_planes
    }

    pub fn unsupported_families(self) -> usize {
        self.unsupported_families
    }

    pub fn upstream_geometry_carriers(self) -> usize {
        self.upstream_geometry_carriers
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceSupportReceiptSet {
    stage_receipt: SurfaceSupportWorkloadReceipt,
    upstream_geometry_binding_identity: String,
    topology_query_surface: String,
    matrix_rows: Vec<SurfaceSupportMatrixRow>,
    counters: SurfaceSupportCounters,
}

impl SurfaceSupportReceiptSet {
    pub(crate) fn new(
        stage_receipt: SurfaceSupportWorkloadReceipt,
        upstream_geometry_binding_identity: impl Into<String>,
        topology_query_surface: impl Into<String>,
        matrix_rows: Vec<SurfaceSupportMatrixRow>,
        upstream_geometry_carriers: usize,
    ) -> Self {
        let counters =
            surface_support_counters_from_matrix(&matrix_rows, upstream_geometry_carriers);
        Self {
            stage_receipt,
            upstream_geometry_binding_identity: upstream_geometry_binding_identity.into(),
            topology_query_surface: topology_query_surface.into(),
            matrix_rows,
            counters,
        }
    }

    pub fn stage_identity(&self) -> &WorkloadStageIdentity {
        self.stage_receipt.identity()
    }

    pub fn stage_receipt(&self) -> &SurfaceSupportWorkloadReceipt {
        &self.stage_receipt
    }

    pub fn upstream_geometry_binding_identity(&self) -> &str {
        &self.upstream_geometry_binding_identity
    }

    pub fn topology_query_surface(&self) -> &str {
        &self.topology_query_surface
    }

    pub fn matrix_rows(&self) -> &[SurfaceSupportMatrixRow] {
        &self.matrix_rows
    }

    pub fn counters(&self) -> SurfaceSupportCounters {
        self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsupportedSurfaceSupportReceipt {
    envelope: WorkloadStageEnvelope,
    family: Option<SurfaceFamily>,
    reason_code: UnsupportedSurfaceSupportReasonCode,
    matrix_rows: Vec<SurfaceSupportMatrixRow>,
    counters: SurfaceSupportCounters,
}

impl UnsupportedSurfaceSupportReceipt {
    pub(crate) fn new(
        declaration: String,
        upstream_geometry_binding_identity: String,
        family: Option<SurfaceFamily>,
        reason_code: UnsupportedSurfaceSupportReasonCode,
        human_reason: String,
        matrix_rows: Vec<SurfaceSupportMatrixRow>,
        upstream_geometry_carriers: usize,
    ) -> Self {
        let identity = WorkloadStageIdentity::new(
            SpatialWorkloadStage::SurfaceSupport,
            declaration,
            upstream_geometry_binding_identity,
        );
        let posture =
            WorkloadStagePosture::unsupported(SpatialWorkloadStage::SurfaceSupport, human_reason);
        let counters =
            surface_support_counters_from_matrix(&matrix_rows, upstream_geometry_carriers);
        Self {
            envelope: WorkloadStageEnvelope::new(identity, posture),
            family,
            reason_code,
            matrix_rows,
            counters,
        }
    }

    pub fn stage_identity(&self) -> &WorkloadStageIdentity {
        self.envelope.identity()
    }

    pub fn envelope(&self) -> &WorkloadStageEnvelope {
        &self.envelope
    }

    pub fn family(&self) -> Option<SurfaceFamily> {
        self.family
    }

    pub fn reason_code(&self) -> UnsupportedSurfaceSupportReasonCode {
        self.reason_code
    }

    pub fn matrix_rows(&self) -> &[SurfaceSupportMatrixRow] {
        &self.matrix_rows
    }

    pub fn counters(&self) -> SurfaceSupportCounters {
        self.counters
    }
}

fn surface_support_counters_from_matrix(
    matrix_rows: &[SurfaceSupportMatrixRow],
    upstream_geometry_carriers: usize,
) -> SurfaceSupportCounters {
    let certified_planes = matrix_rows
        .iter()
        .filter(|row| row.status() == SurfaceSupportStatus::Certified)
        .count();
    let unsupported_families = matrix_rows
        .iter()
        .filter(|row| row.status() == SurfaceSupportStatus::Unsupported)
        .count();
    SurfaceSupportCounters::new(
        matrix_rows.len(),
        certified_planes,
        unsupported_families,
        upstream_geometry_carriers,
    )
}

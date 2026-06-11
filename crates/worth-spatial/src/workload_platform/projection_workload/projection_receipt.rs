use super::{CertifiedLocalFrameReceipt, ProjectionConsumedWorkloadReceipt};
use crate::workload_platform::vocabulary::{ProjectionWorkloadReceipt, WorkloadStageIdentity};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProjectionWorkloadCounters {
    projected_faces: usize,
    projected_edges: usize,
    projected_loops: usize,
    local_basis_parts: usize,
}

impl ProjectionWorkloadCounters {
    pub(crate) fn new(
        projected_faces: usize,
        projected_edges: usize,
        projected_loops: usize,
        local_basis_parts: usize,
    ) -> Self {
        Self {
            projected_faces,
            projected_edges,
            projected_loops,
            local_basis_parts,
        }
    }

    pub fn projected_faces(self) -> usize {
        self.projected_faces
    }

    pub fn projected_edges(self) -> usize {
        self.projected_edges
    }

    pub fn projected_loops(self) -> usize {
        self.projected_loops
    }

    pub fn local_basis_parts(self) -> usize {
        self.local_basis_parts
    }

    pub fn projected_topology_entities(self) -> usize {
        self.projected_faces + self.projected_edges + self.projected_loops
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionReceiptSet {
    stage_receipt: ProjectionWorkloadReceipt,
    upstream_surface_support_identity: String,
    certified_plane_support_identity: String,
    topology_query_surface: String,
    local_frame_receipt: CertifiedLocalFrameReceipt,
    projection_consumption_receipt: ProjectionConsumedWorkloadReceipt,
    counters: ProjectionWorkloadCounters,
}

impl ProjectionReceiptSet {
    pub(crate) fn new(
        stage_receipt: ProjectionWorkloadReceipt,
        upstream_surface_support_identity: impl Into<String>,
        certified_plane_support_identity: impl Into<String>,
        topology_query_surface: impl Into<String>,
        local_frame_receipt: CertifiedLocalFrameReceipt,
        projection_consumption_receipt: ProjectionConsumedWorkloadReceipt,
        counters: ProjectionWorkloadCounters,
    ) -> Self {
        Self {
            stage_receipt,
            upstream_surface_support_identity: upstream_surface_support_identity.into(),
            certified_plane_support_identity: certified_plane_support_identity.into(),
            topology_query_surface: topology_query_surface.into(),
            local_frame_receipt,
            projection_consumption_receipt,
            counters,
        }
    }

    pub fn stage_identity(&self) -> &WorkloadStageIdentity {
        self.stage_receipt.identity()
    }

    pub fn stage_receipt(&self) -> &ProjectionWorkloadReceipt {
        &self.stage_receipt
    }

    pub fn upstream_surface_support_identity(&self) -> &str {
        &self.upstream_surface_support_identity
    }

    pub fn certified_plane_support_identity(&self) -> &str {
        &self.certified_plane_support_identity
    }

    pub fn topology_query_surface(&self) -> &str {
        &self.topology_query_surface
    }

    pub fn local_frame_receipt(&self) -> &CertifiedLocalFrameReceipt {
        &self.local_frame_receipt
    }

    pub fn projection_consumption_receipt(&self) -> &ProjectionConsumedWorkloadReceipt {
        &self.projection_consumption_receipt
    }

    pub fn counters(&self) -> ProjectionWorkloadCounters {
        self.counters
    }
}

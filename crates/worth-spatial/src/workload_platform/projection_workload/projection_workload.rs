use super::{
    CertifiedLocalFrameReceipt, CertifiedLocalFrameWorkload, LocalFrameBasis, ProjectedEdgeSet,
    ProjectedFace, ProjectedLoop, ProjectedTopologyEntities, ProjectionConsumedWorkloadReceipt,
    ProjectionReceiptSet, ProjectionWorkloadCounters, UnsupportedProjectionReasonCode,
    UnsupportedProjectionWorkload,
};
use crate::workload_platform::{
    surface_support::CertifiedSurfaceSupport,
    vocabulary::{ProjectionWorkloadReceipt, WorkloadStageIdentity},
};

pub struct ProjectionWorkload {
    surface_support: CertifiedSurfaceSupport,
    declaration: String,
    local_frame_basis: Option<LocalFrameBasis>,
}

impl ProjectionWorkload {
    pub fn for_certified_surface_support(surface_support: CertifiedSurfaceSupport) -> Self {
        Self {
            surface_support,
            declaration: "projection workload".to_string(),
            local_frame_basis: None,
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn with_local_frame(mut self, local_frame_basis: LocalFrameBasis) -> Self {
        self.local_frame_basis = Some(local_frame_basis);
        self
    }

    pub fn project(mut self) -> Result<ProjectedPlanarWorkload, UnsupportedProjectionWorkload> {
        if self.declaration.trim().is_empty() {
            return Err(UnsupportedProjectionWorkload::new(
                UnsupportedProjectionReasonCode::MissingDeclaration,
                "Projection workload requires a human-readable declaration.",
            ));
        }
        if !self.surface_support.can_enter_projection_workload() {
            return Err(UnsupportedProjectionWorkload::new(
                UnsupportedProjectionReasonCode::MissingCertifiedSurfaceSupport,
                "Projection workload requires certified plane surface support.",
            ));
        }
        let Some(local_frame_basis) = self.local_frame_basis.take() else {
            return Err(UnsupportedProjectionWorkload::new(
                UnsupportedProjectionReasonCode::MissingLocalFrameBasis,
                "Projection workload requires an explicit local frame basis.",
            ));
        };

        self.project_with_basis(local_frame_basis)
    }

    fn project_with_basis(
        self,
        local_frame_basis: LocalFrameBasis,
    ) -> Result<ProjectedPlanarWorkload, UnsupportedProjectionWorkload> {
        let projection_origin =
            ProjectionOriginIdentities::from_certified_surface_support(&self.surface_support);
        let local_frame_receipt =
            CertifiedLocalFrameReceipt::new(&self.surface_support, &local_frame_basis);
        let projected_topology_entities = ProjectedTopologyEntities::from_certified_surface_support(
            &self.surface_support,
            projection_origin.surface_support_identity(),
            local_frame_basis.identity(),
        );
        let counters = projection_workload_counters(
            &projected_topology_entities,
            local_frame_basis.basis_parts().len(),
        );
        let stage_receipt =
            admit_projection_stage_receipt(&self.surface_support, self.declaration)?;
        let projection_consumption_receipt = projection_consumption_receipt(
            stage_receipt.identity().clone(),
            &local_frame_basis,
            counters,
        );
        let receipts = projection_receipt_set(
            stage_receipt,
            projection_origin,
            local_frame_receipt,
            projection_consumption_receipt,
            counters,
        );
        let (projected_faces, projected_edges, projected_loops) =
            projected_topology_entities.into_parts();
        Ok(ProjectedPlanarWorkload::new(
            CertifiedLocalFrameWorkload::new(receipts.local_frame_receipt().clone()),
            projected_faces,
            ProjectedEdgeSet::new(projected_edges),
            projected_loops,
            receipts,
        ))
    }
}

struct ProjectionOriginIdentities {
    surface_support_identity: String,
    certified_plane_support_identity: String,
    topology_query_surface: String,
}

impl ProjectionOriginIdentities {
    fn from_certified_surface_support(surface_support: &CertifiedSurfaceSupport) -> Self {
        Self {
            surface_support_identity: surface_support
                .receipts()
                .stage_identity()
                .receipt_identity(),
            certified_plane_support_identity: surface_support
                .certified_plane_support()
                .upstream_geometry_binding_identity()
                .to_string(),
            topology_query_surface: surface_support
                .certified_plane_support()
                .topology_query_surface()
                .to_string(),
        }
    }

    fn surface_support_identity(&self) -> &str {
        &self.surface_support_identity
    }
}

fn projection_workload_counters(
    projected_topology_entities: &ProjectedTopologyEntities,
    local_basis_parts: usize,
) -> ProjectionWorkloadCounters {
    ProjectionWorkloadCounters::new(
        projected_topology_entities.face_count(),
        projected_topology_entities.edge_count(),
        projected_topology_entities.loop_count(),
        local_basis_parts,
    )
}

fn admit_projection_stage_receipt(
    surface_support: &CertifiedSurfaceSupport,
    declaration: String,
) -> Result<ProjectionWorkloadReceipt, UnsupportedProjectionWorkload> {
    crate::workload_platform::vocabulary::ProjectionWorkload::for_surface_support(
        surface_support.receipts().stage_receipt(),
    )
    .declared(declaration)
    .admit()
    .map_err(|_| {
        UnsupportedProjectionWorkload::new(
            UnsupportedProjectionReasonCode::ProjectionStageReceiptDenied,
            "Projection workload could not produce a stage receipt from surface support.",
        )
    })
}

fn projection_consumption_receipt(
    projection_stage_identity: WorkloadStageIdentity,
    local_frame_basis: &LocalFrameBasis,
    counters: ProjectionWorkloadCounters,
) -> ProjectionConsumedWorkloadReceipt {
    ProjectionConsumedWorkloadReceipt::new(
        projection_stage_identity,
        local_frame_basis.identity(),
        counters.projected_topology_entities(),
    )
}

fn projection_receipt_set(
    stage_receipt: ProjectionWorkloadReceipt,
    projection_origin: ProjectionOriginIdentities,
    local_frame_receipt: CertifiedLocalFrameReceipt,
    projection_consumption_receipt: ProjectionConsumedWorkloadReceipt,
    counters: ProjectionWorkloadCounters,
) -> ProjectionReceiptSet {
    ProjectionReceiptSet::new(
        stage_receipt,
        projection_origin.surface_support_identity,
        projection_origin.certified_plane_support_identity,
        projection_origin.topology_query_surface,
        local_frame_receipt,
        projection_consumption_receipt,
        counters,
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectedPlanarWorkload {
    local_frame: CertifiedLocalFrameWorkload,
    projected_faces: Vec<ProjectedFace>,
    projected_edges: ProjectedEdgeSet,
    projected_loops: Vec<ProjectedLoop>,
    receipts: ProjectionReceiptSet,
}

impl ProjectedPlanarWorkload {
    pub(crate) fn new(
        local_frame: CertifiedLocalFrameWorkload,
        projected_faces: Vec<ProjectedFace>,
        projected_edges: ProjectedEdgeSet,
        projected_loops: Vec<ProjectedLoop>,
        receipts: ProjectionReceiptSet,
    ) -> Self {
        Self {
            local_frame,
            projected_faces,
            projected_edges,
            projected_loops,
            receipts,
        }
    }

    pub fn local_frame(&self) -> &CertifiedLocalFrameWorkload {
        &self.local_frame
    }

    pub fn projected_faces(&self) -> &[ProjectedFace] {
        &self.projected_faces
    }

    pub fn projected_edges(&self) -> &ProjectedEdgeSet {
        &self.projected_edges
    }

    pub fn projected_loops(&self) -> &[ProjectedLoop] {
        &self.projected_loops
    }

    pub fn receipts(&self) -> &ProjectionReceiptSet {
        &self.receipts
    }

    pub fn can_enter_projection_consumed_planar_facts(&self) -> bool {
        true
    }

    pub fn can_enter_operator_execution(&self) -> bool {
        false
    }
}

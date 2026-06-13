use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::projection_workload::ProjectedPlanarWorkload;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedAnalysisSurface {
    surface_identity: String,
    projection_stage_identity: String,
    surface_support_identity: String,
    certified_plane_support_identity: String,
    topology_query_surface_identity: String,
    workload_local_basis_identity: String,
}

impl CertifiedAnalysisSurface {
    pub(crate) fn from_projected_workload(projected: &ProjectedPlanarWorkload) -> Self {
        let receipts = projected.receipts();
        let projection_stage_identity = receipts.stage_identity().receipt_identity().to_string();
        let surface_support_identity = receipts.upstream_surface_support_identity().to_string();
        let certified_plane_support_identity =
            receipts.certified_plane_support_identity().to_string();
        let topology_query_surface_identity = receipts.topology_query_surface().to_string();
        let workload_local_basis_identity = projected
            .local_frame()
            .receipt()
            .local_basis_identity()
            .to_string();
        let surface_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                format!("projection-stage:{projection_stage_identity}"),
                format!("surface-support:{surface_support_identity}"),
                format!("plane-support:{certified_plane_support_identity}"),
                format!("topology-query-surface:{topology_query_surface_identity}"),
                format!("workload-local-basis:{workload_local_basis_identity}"),
            ],
        );
        Self {
            surface_identity,
            projection_stage_identity,
            surface_support_identity,
            certified_plane_support_identity,
            topology_query_surface_identity,
            workload_local_basis_identity,
        }
    }

    pub fn surface_identity(&self) -> &str {
        &self.surface_identity
    }

    pub fn projection_stage_identity(&self) -> &str {
        &self.projection_stage_identity
    }

    pub fn surface_support_identity(&self) -> &str {
        &self.surface_support_identity
    }

    pub fn certified_plane_support_identity(&self) -> &str {
        &self.certified_plane_support_identity
    }

    pub fn topology_query_surface_identity(&self) -> &str {
        &self.topology_query_surface_identity
    }

    pub fn workload_local_basis_identity(&self) -> &str {
        &self.workload_local_basis_identity
    }
}

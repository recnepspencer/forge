use crate::identity::hash_parts;

use super::super::super::materialization::QueryCausalInspectionArtifact;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspectionBoundaryAudit {
    ordinary_path_uses_query_artifact: bool,
    direct_lower_runtime_stitching_absent: bool,
    audited_artifact_digest: String,
    audit_digest: String,
}

impl CausalInspectionBoundaryAudit {
    pub fn from_query_artifact_public_surface(artifact: &QueryCausalInspectionArtifact) -> Self {
        let audited_artifact_digest = artifact.artifact_for_reporting().to_string();
        let audit_digest = hash_parts(&[
            "causal_inspection_boundary_audit_v1".to_string(),
            "ordinary-path:query-artifact".to_string(),
            "direct-lower-runtime-stitching:false".to_string(),
            format!("artifact:{audited_artifact_digest}"),
        ]);
        Self {
            ordinary_path_uses_query_artifact: true,
            direct_lower_runtime_stitching_absent: true,
            audited_artifact_digest,
            audit_digest,
        }
    }

    pub fn ordinary_path_uses_query_artifact(&self) -> bool {
        self.ordinary_path_uses_query_artifact
    }

    pub fn direct_lower_runtime_stitching_absent(&self) -> bool {
        self.direct_lower_runtime_stitching_absent
    }

    pub fn audited_artifact_digest(&self) -> &str {
        &self.audited_artifact_digest
    }

    pub fn audit_digest(&self) -> &str {
        &self.audit_digest
    }
}

use forge_query::facade::runtime::ForgeQueryRuntimeFacadeFamily;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_lookup_inventory::EvidenceLookupQuerySurface;
use crate::workload_platform::evidence_lookup_query_surface_matrix::EvidenceLookupQuerySurfaceTouchpoint;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupQuerySupportRequirementRow {
    runtime_family: ForgeQueryRuntimeFacadeFamily,
    touchpoint: EvidenceLookupQuerySurfaceTouchpoint,
    query_surface: EvidenceLookupQuerySurface,
    row_digest: String,
}

impl EvidenceLookupQuerySupportRequirementRow {
    pub(crate) fn new(
        runtime_family: ForgeQueryRuntimeFacadeFamily,
        touchpoint: EvidenceLookupQuerySurfaceTouchpoint,
        query_surface: EvidenceLookupQuerySurface,
    ) -> Self {
        let row_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:evidence-lookup-query-support-requirement-row:v1".to_string(),
                runtime_family.as_str().to_string(),
                touchpoint.as_str().to_string(),
                format!("{query_surface:?}"),
            ],
        );
        Self {
            runtime_family,
            touchpoint,
            query_surface,
            row_digest,
        }
    }

    pub const fn runtime_family(&self) -> ForgeQueryRuntimeFacadeFamily {
        self.runtime_family
    }

    pub const fn touchpoint(&self) -> EvidenceLookupQuerySurfaceTouchpoint {
        self.touchpoint
    }

    pub const fn query_surface(&self) -> EvidenceLookupQuerySurface {
        self.query_surface
    }

    #[cfg(test)]
    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

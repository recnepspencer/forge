use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSegmentCarrierEndpointFacts {
    point: [f64; 2],
    source_endpoint_identity: String,
    projected_endpoint_fact_identity: String,
    projected_loop_identity: String,
    projection_stage_identity: String,
    projection_local_basis_identity: String,
}

impl PlanarBooleanSegmentCarrierEndpointFacts {
    pub(crate) fn from_projected_loop_boundary(
        point: [f64; 2],
        source_endpoint_identity: impl Into<String>,
        projected_loop_identity: &str,
        projection_stage_identity: &str,
        projection_local_basis_identity: &str,
    ) -> Self {
        let source_endpoint_identity = source_endpoint_identity.into();
        let projected_endpoint_fact_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-segment-endpoint".to_string(),
                format!("source-endpoint:{source_endpoint_identity}"),
                format!("projected-loop:{projected_loop_identity}"),
                format!("projection-stage:{projection_stage_identity}"),
                format!("projection-local-basis:{projection_local_basis_identity}"),
                format!("point:{point:?}"),
            ],
        );
        Self {
            point,
            source_endpoint_identity,
            projected_endpoint_fact_identity,
            projected_loop_identity: projected_loop_identity.to_string(),
            projection_stage_identity: projection_stage_identity.to_string(),
            projection_local_basis_identity: projection_local_basis_identity.to_string(),
        }
    }

    pub fn point(&self) -> [f64; 2] {
        self.point
    }

    pub fn source_endpoint_identity(&self) -> &str {
        &self.source_endpoint_identity
    }

    pub fn projected_endpoint_fact_identity(&self) -> &str {
        &self.projected_endpoint_fact_identity
    }

    pub fn projected_loop_identity(&self) -> &str {
        &self.projected_loop_identity
    }

    pub fn projection_stage_identity(&self) -> &str {
        &self.projection_stage_identity
    }

    pub fn projection_local_basis_identity(&self) -> &str {
        &self.projection_local_basis_identity
    }

    #[cfg(test)]
    pub(crate) fn for_canonical_segment_test(
        point: [f64; 2],
        source_endpoint_identity: &str,
        projected_endpoint_fact_identity: &str,
    ) -> Self {
        Self {
            point,
            source_endpoint_identity: source_endpoint_identity.to_string(),
            projected_endpoint_fact_identity: projected_endpoint_fact_identity.to_string(),
            projected_loop_identity: "test projected loop".to_string(),
            projection_stage_identity: "test projection stage".to_string(),
            projection_local_basis_identity: "test projection local basis".to_string(),
        }
    }
}

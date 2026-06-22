use crate::workload_platform::projection_workload::{ProjectedEdge, ProjectedFace, ProjectedLoop};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformedEntityIdentity {
    projected_fact_identity: String,
    transform_evidence_identity: String,
    transformed_fact_identity: String,
}

impl TransformedEntityIdentity {
    fn from_projected_fact(
        projected_fact_identity: &str,
        transform_evidence_identity: &str,
    ) -> Self {
        Self {
            projected_fact_identity: projected_fact_identity.to_string(),
            transform_evidence_identity: transform_evidence_identity.to_string(),
            transformed_fact_identity: format!(
                "transformed-entity:{projected_fact_identity}:{transform_evidence_identity}"
            ),
        }
    }

    pub fn projected_fact_identity(&self) -> &str {
        &self.projected_fact_identity
    }

    pub fn transform_evidence_identity(&self) -> &str {
        &self.transform_evidence_identity
    }

    pub fn transformed_fact_identity(&self) -> &str {
        &self.transformed_fact_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformedFace {
    identity: TransformedEntityIdentity,
}

impl TransformedFace {
    pub(crate) fn from_projected_face(
        face: &ProjectedFace,
        transform_evidence_identity: &str,
    ) -> Self {
        Self {
            identity: TransformedEntityIdentity::from_projected_fact(
                face.identity().projected_fact_identity(),
                transform_evidence_identity,
            ),
        }
    }

    pub fn identity(&self) -> &TransformedEntityIdentity {
        &self.identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformedEdge {
    identity: TransformedEntityIdentity,
}

impl TransformedEdge {
    pub(crate) fn from_projected_edge(
        edge: &ProjectedEdge,
        transform_evidence_identity: &str,
    ) -> Self {
        Self {
            identity: TransformedEntityIdentity::from_projected_fact(
                edge.identity().projected_fact_identity(),
                transform_evidence_identity,
            ),
        }
    }

    pub fn identity(&self) -> &TransformedEntityIdentity {
        &self.identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformedLoop {
    identity: TransformedEntityIdentity,
}

impl TransformedLoop {
    pub(crate) fn from_projected_loop(
        loop_entity: &ProjectedLoop,
        transform_evidence_identity: &str,
    ) -> Self {
        Self {
            identity: TransformedEntityIdentity::from_projected_fact(
                loop_entity.identity().projected_fact_identity(),
                transform_evidence_identity,
            ),
        }
    }

    pub fn identity(&self) -> &TransformedEntityIdentity {
        &self.identity
    }
}

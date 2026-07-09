use worth_query::facade::{
    CanonicalResultShapeArtifact, ResultShapeFamily, ValidatedResultShapeArtifact,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryResultShapeReference {
    family: ResultShapeFamily,
    artifact_digest: Option<String>,
}

impl QueryResultShapeReference {
    pub fn from_result_shape_family(family: ResultShapeFamily) -> Self {
        Self {
            family,
            artifact_digest: None,
        }
    }

    pub fn from_canonical_result_shape(artifact: &CanonicalResultShapeArtifact) -> Self {
        Self {
            family: artifact.family().clone(),
            artifact_digest: Some(
                artifact
                    .result_shape_identity()
                    .terminal_projection_for_reporting()
                    .to_string(),
            ),
        }
    }

    pub fn from_validated_result_shape(
        family: ResultShapeFamily,
        artifact: &ValidatedResultShapeArtifact,
    ) -> Self {
        Self {
            family,
            artifact_digest: Some(
                artifact
                    .validated_result_shape_identity()
                    .terminal_projection_for_reporting()
                    .to_string(),
            ),
        }
    }

    pub fn family(&self) -> &ResultShapeFamily {
        &self.family
    }

    pub fn digest_basis(&self) -> String {
        format!(
            "{:?}|{}",
            self.family,
            self.artifact_digest.as_deref().unwrap_or("family_only")
        )
    }
}

use crate::authorized_projection::AuthorizedProjectionArtifact;
use crate::canonicalization::CanonicalResultShapeArtifact;
use crate::projection_consumption::{ProjectMaterializedFacts, ProjectionFactConsumptionPathError};
use crate::runtime::{
    WorthQueryPublishedDerivedArtifactHandle, WorthQueryPublishedProjectionConsumption,
};

pub struct WorthQueryPublicBridgePublishedProjectionReader<'a> {
    artifact: &'a WorthQueryPublishedDerivedArtifactHandle,
}

impl<'a> WorthQueryPublicBridgePublishedProjectionReader<'a> {
    pub fn from_published_artifact(artifact: &'a WorthQueryPublishedDerivedArtifactHandle) -> Self {
        Self { artifact }
    }

    pub fn consume_projection_facts(
        &self,
        result_shape: &CanonicalResultShapeArtifact,
        authorized_projection: &AuthorizedProjectionArtifact,
        requested: ProjectMaterializedFacts,
    ) -> Result<WorthQueryPublishedProjectionConsumption, ProjectionFactConsumptionPathError> {
        self.artifact
            .consume_projection_facts(result_shape, authorized_projection, requested)
    }
}

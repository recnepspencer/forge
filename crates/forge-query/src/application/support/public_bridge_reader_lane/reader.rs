use crate::authorized_projection::AuthorizedProjectionArtifact;
use crate::canonicalization::CanonicalResultShapeArtifact;
use crate::projection_consumption::{ProjectMaterializedFacts, ProjectionFactConsumptionPathError};
use crate::runtime::{
    ForgeQueryPublishedDerivedArtifactHandle, ForgeQueryPublishedProjectionConsumption,
};

pub struct ForgeQueryPublicBridgePublishedProjectionReader<'a> {
    artifact: &'a ForgeQueryPublishedDerivedArtifactHandle,
}

impl<'a> ForgeQueryPublicBridgePublishedProjectionReader<'a> {
    pub fn from_published_artifact(artifact: &'a ForgeQueryPublishedDerivedArtifactHandle) -> Self {
        Self { artifact }
    }

    pub fn consume_projection_facts(
        &self,
        result_shape: &CanonicalResultShapeArtifact,
        authorized_projection: &AuthorizedProjectionArtifact,
        requested: ProjectMaterializedFacts,
    ) -> Result<ForgeQueryPublishedProjectionConsumption, ProjectionFactConsumptionPathError> {
        self.artifact
            .consume_projection_facts(result_shape, authorized_projection, requested)
    }
}

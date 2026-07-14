use crate::authorized_projection::AuthorizedProjectionArtifact;
use crate::canonicalization::CanonicalResultShapeArtifact;
use crate::projection_consumption::{
    ProjectionAuthorityContract, ProjectionFactConsumptionPathError,
};
use crate::runtime::{
    WorthQueryPublishedDerivedArtifactHandle, WorthQueryPublishedProjectionAuthorityOutcome,
};

pub struct WorthQueryPublicBridgePublishedProjectionReader<'a> {
    artifact: &'a WorthQueryPublishedDerivedArtifactHandle,
}

impl<'a> WorthQueryPublicBridgePublishedProjectionReader<'a> {
    pub fn from_published_artifact(artifact: &'a WorthQueryPublishedDerivedArtifactHandle) -> Self {
        Self { artifact }
    }

    pub fn consume_projection_authority(
        &self,
        result_shape: &CanonicalResultShapeArtifact,
        authorized_projection: &AuthorizedProjectionArtifact,
        contract: ProjectionAuthorityContract,
    ) -> Result<WorthQueryPublishedProjectionAuthorityOutcome, ProjectionFactConsumptionPathError>
    {
        self.artifact
            .consume_projection_authority(result_shape, authorized_projection, contract)
    }
}

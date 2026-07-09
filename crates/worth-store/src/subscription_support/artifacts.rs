use super::witnesses::{
    SubscriptionSupportBasisWitness, SubscriptionSupportCheckpointWitness,
    SubscriptionSupportCompatibilityWitness, SubscriptionSupportCursorWitness,
    SubscriptionSupportSchemaWitness,
};
use super::{stable_digest, AdmittedSubscriptionSupportDeclaration, SubscriptionSupportArtifactId};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PublishableSubscriptionSupportArtifact {
    pub(crate) declaration: AdmittedSubscriptionSupportDeclaration,
    pub(crate) basis: SubscriptionSupportBasisWitness,
    pub(crate) cursor: SubscriptionSupportCursorWitness,
    pub(crate) checkpoint: SubscriptionSupportCheckpointWitness,
    pub(crate) schema: SubscriptionSupportSchemaWitness,
    pub(crate) compatibility: SubscriptionSupportCompatibilityWitness,
    pub(crate) artifact_id: SubscriptionSupportArtifactId,
}

impl PublishableSubscriptionSupportArtifact {
    pub(crate) fn new(
        declaration: AdmittedSubscriptionSupportDeclaration,
        basis: SubscriptionSupportBasisWitness,
        cursor: SubscriptionSupportCursorWitness,
        checkpoint: SubscriptionSupportCheckpointWitness,
        schema: SubscriptionSupportSchemaWitness,
        compatibility: SubscriptionSupportCompatibilityWitness,
    ) -> Result<Self, StoreError> {
        let artifact_id = deterministic_artifact_id(
            &declaration,
            &basis,
            &cursor,
            &checkpoint,
            &schema,
            &compatibility,
        )?;
        Ok(Self {
            declaration,
            basis,
            cursor,
            checkpoint,
            schema,
            compatibility,
            artifact_id,
        })
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PublishedSubscriptionSupportArtifact {
    pub(crate) declaration: AdmittedSubscriptionSupportDeclaration,
    pub(crate) artifact_id: SubscriptionSupportArtifactId,
    pub(crate) artifact_digest: String,
}

impl PublishedSubscriptionSupportArtifact {
    pub(crate) fn new(
        artifact: PublishableSubscriptionSupportArtifact,
    ) -> Result<Self, StoreError> {
        let artifact_digest = stable_digest(&artifact)?;
        Ok(Self {
            declaration: artifact.declaration,
            artifact_id: artifact.artifact_id,
            artifact_digest,
        })
    }

    pub fn declaration(&self) -> &AdmittedSubscriptionSupportDeclaration {
        &self.declaration
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }
}

fn deterministic_artifact_id(
    declaration: &AdmittedSubscriptionSupportDeclaration,
    basis: &SubscriptionSupportBasisWitness,
    cursor: &SubscriptionSupportCursorWitness,
    checkpoint: &SubscriptionSupportCheckpointWitness,
    schema: &SubscriptionSupportSchemaWitness,
    compatibility: &SubscriptionSupportCompatibilityWitness,
) -> Result<SubscriptionSupportArtifactId, StoreError> {
    Ok(SubscriptionSupportArtifactId(format!(
        "subscription-support:{}:{}",
        declaration.family_id().as_str(),
        stable_digest(&(
            declaration,
            basis,
            cursor,
            checkpoint,
            schema,
            compatibility
        ))?
    )))
}

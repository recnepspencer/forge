use super::{
    classification_error, PublishableSubscriptionSupportArtifact,
    PublishedSubscriptionSupportArtifact, SubscriptionResumeClassification,
    SubscriptionSupportArtifactId, SubscriptionSupportClassificationReport,
    SubscriptionSupportDeclarationDigest, SubscriptionSupportDriftCause,
    SubscriptionSupportFamilyId, SubscriptionSupportResultCostSurface,
};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportArtifactRecord {
    family_id: SubscriptionSupportFamilyId,
    artifact_id: SubscriptionSupportArtifactId,
    declaration_digest: SubscriptionSupportDeclarationDigest,
    artifact_digest: String,
}

impl SubscriptionSupportArtifactRecord {
    pub fn from_published(artifact: &PublishedSubscriptionSupportArtifact) -> Self {
        Self {
            family_id: artifact.declaration.family_id().clone(),
            artifact_id: artifact.artifact_id.clone(),
            declaration_digest: artifact.declaration.declaration_digest.clone(),
            artifact_digest: artifact.artifact_digest.clone(),
        }
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportLinkageRecord {
    artifact_id: SubscriptionSupportArtifactId,
    basis_digest: String,
    cursor_digest: String,
    checkpoint_digest: String,
    schema_digest: String,
    compatibility_binding: String,
    compatibility_digest: String,
}

impl SubscriptionSupportLinkageRecord {
    pub fn from_publishable(artifact: &PublishableSubscriptionSupportArtifact) -> Self {
        Self {
            artifact_id: artifact.artifact_id.clone(),
            basis_digest: artifact.basis.stable_basis_digest.clone(),
            cursor_digest: artifact.cursor.cursor_digest.clone(),
            checkpoint_digest: artifact.checkpoint.checkpoint_digest.clone(),
            schema_digest: artifact.schema.schema_digest.clone(),
            compatibility_binding: artifact
                .declaration
                .declaration
                .compatibility_binding
                .clone(),
            compatibility_digest: artifact.compatibility.compatibility_digest.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportClassificationRecord {
    artifact_id: SubscriptionSupportArtifactId,
    classification: SubscriptionResumeClassification,
    primary_cause: Option<SubscriptionSupportDriftCause>,
    suppressed_causes: Vec<SubscriptionSupportDriftCause>,
    cost_surface: SubscriptionSupportResultCostSurface,
}

impl SubscriptionSupportClassificationRecord {
    pub fn from_report(report: &SubscriptionSupportClassificationReport) -> Self {
        Self {
            artifact_id: report.artifact_id.clone(),
            classification: report.classification,
            primary_cause: report.primary_cause,
            suppressed_causes: report.suppressed_causes.clone(),
            cost_surface: report.cost_surface,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportRestartRecord {
    artifact_id: SubscriptionSupportArtifactId,
    restart_shard: String,
    classification: SubscriptionResumeClassification,
}

impl SubscriptionSupportRestartRecord {
    pub fn new(
        report: &SubscriptionSupportClassificationReport,
        restart_shard: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let restart_shard = restart_shard.into();
        if restart_shard.trim().is_empty() {
            return Err(classification_error(
                "subscription-support restart records require a non-empty restart shard",
            ));
        }
        Ok(Self {
            artifact_id: report.artifact_id.clone(),
            restart_shard,
            classification: report.classification,
        })
    }
}

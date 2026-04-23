use super::{
    classification_error, PublishedSubscriptionSupportArtifact, SubscriptionResumeClassification,
    SubscriptionSupportArtifactId, SubscriptionSupportClassificationReport,
    SubscriptionSupportDeclarationDigest, SubscriptionSupportDriftCause,
    SubscriptionSupportFamilyKind,
};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExactSubscriptionResumeHandle {
    artifact_id: SubscriptionSupportArtifactId,
    declaration_digest: SubscriptionSupportDeclarationDigest,
}

impl ExactSubscriptionResumeHandle {
    pub(crate) fn new(artifact: &PublishedSubscriptionSupportArtifact) -> Self {
        Self {
            artifact_id: artifact.artifact_id.clone(),
            declaration_digest: artifact.declaration.declaration_digest.clone(),
        }
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DegradedSubscriptionResumeHandle {
    artifact_id: SubscriptionSupportArtifactId,
    reason: SubscriptionSupportDriftCause,
}

impl DegradedSubscriptionResumeHandle {
    pub(crate) fn new(
        artifact: &PublishedSubscriptionSupportArtifact,
        reason: SubscriptionSupportDriftCause,
    ) -> Self {
        Self {
            artifact_id: artifact.artifact_id.clone(),
            reason,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportRebuildPlanHandle {
    artifact_id: SubscriptionSupportArtifactId,
    retained_rebuild_basis_digest: String,
    missing_or_stale_families: Vec<SubscriptionSupportFamilyKind>,
}

impl SubscriptionSupportRebuildPlanHandle {
    pub(crate) fn new(
        artifact: &PublishedSubscriptionSupportArtifact,
        retained_rebuild_basis_digest: impl Into<String>,
        mut missing_or_stale_families: Vec<SubscriptionSupportFamilyKind>,
    ) -> Result<Self, StoreError> {
        let retained_rebuild_basis_digest = retained_rebuild_basis_digest.into();
        if retained_rebuild_basis_digest.trim().is_empty() {
            return Err(classification_error(
                "rebuild-required subscription support requires retained rebuild basis evidence",
            ));
        }
        missing_or_stale_families.sort();
        missing_or_stale_families.dedup();
        if missing_or_stale_families.is_empty() {
            return Err(classification_error(
                "rebuild-required subscription support must name missing or stale families",
            ));
        }
        Ok(Self {
            artifact_id: artifact.artifact_id.clone(),
            retained_rebuild_basis_digest,
            missing_or_stale_families,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionResumeDeniedReport {
    artifact_id: SubscriptionSupportArtifactId,
    primary_cause: SubscriptionSupportDriftCause,
    suppressed_causes: Vec<SubscriptionSupportDriftCause>,
}

impl SubscriptionResumeDeniedReport {
    pub(crate) fn new(
        artifact: &PublishedSubscriptionSupportArtifact,
        primary_cause: SubscriptionSupportDriftCause,
        suppressed_causes: Vec<SubscriptionSupportDriftCause>,
    ) -> Self {
        Self {
            artifact_id: artifact.artifact_id.clone(),
            primary_cause,
            suppressed_causes,
        }
    }

    pub fn primary_cause(&self) -> SubscriptionSupportDriftCause {
        self.primary_cause
    }
}

pub(crate) fn ensure_report_matches_artifact(
    artifact: &PublishedSubscriptionSupportArtifact,
    report: &SubscriptionSupportClassificationReport,
) -> Result<(), StoreError> {
    if report.artifact_id != artifact.artifact_id
        || report.declaration_digest != artifact.declaration.declaration_digest
    {
        return Err(classification_error(
            "subscription-support resume handles require a report for the same artifact and declaration",
        ));
    }
    Ok(())
}

pub(crate) fn ensure_classification(
    report: &SubscriptionSupportClassificationReport,
    classification: SubscriptionResumeClassification,
    message: &'static str,
) -> Result<(), StoreError> {
    if report.classification != classification {
        return Err(classification_error(message));
    }
    Ok(())
}

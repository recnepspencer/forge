use super::super::{
    SubscriptionSupportArtifactId, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportRole,
};
use crate::failure::StoreError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionSupportActionOrigin {
    Retention,
    Compatibility,
    ReplicationExport,
    ReplicationImport,
    Maintenance,
    RestartRecovery,
    TierRecall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionSupportOperationalVerdict {
    ExactResumePreserved,
    DegradedResumePreserved,
    RebuildRequired,
    NotResumable,
    RejectedByPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriptionSupportOperationalBasis {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    artifact_id: SubscriptionSupportArtifactId,
    basis_digest: String,
    cursor_digest: String,
    checkpoint_digest: String,
    compatibility_digest: String,
    portability_digest: String,
    action_origin: SubscriptionSupportActionOrigin,
}

impl SubscriptionSupportOperationalBasis {
    pub fn new(
        family_id: SubscriptionSupportFamilyId,
        family_kind: SubscriptionSupportFamilyKind,
        support_role: SubscriptionSupportRole,
        artifact_id: SubscriptionSupportArtifactId,
        basis_digest: impl Into<String>,
        cursor_digest: impl Into<String>,
        checkpoint_digest: impl Into<String>,
        compatibility_digest: impl Into<String>,
        portability_digest: impl Into<String>,
        action_origin: SubscriptionSupportActionOrigin,
    ) -> Result<Self, StoreError> {
        let basis_digest = require_non_empty("basis", basis_digest)?;
        let cursor_digest = require_non_empty("cursor", cursor_digest)?;
        let checkpoint_digest = require_non_empty("checkpoint", checkpoint_digest)?;
        let compatibility_digest = require_non_empty("compatibility", compatibility_digest)?;
        let portability_digest = require_non_empty("portability", portability_digest)?;
        Ok(Self {
            family_id,
            family_kind,
            support_role,
            artifact_id,
            basis_digest,
            cursor_digest,
            checkpoint_digest,
            compatibility_digest,
            portability_digest,
            action_origin,
        })
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn cursor_digest(&self) -> &str {
        &self.cursor_digest
    }

    pub fn checkpoint_digest(&self) -> &str {
        &self.checkpoint_digest
    }

    pub fn compatibility_digest(&self) -> &str {
        &self.compatibility_digest
    }

    pub fn portability_digest(&self) -> &str {
        &self.portability_digest
    }

    pub fn action_origin(&self) -> SubscriptionSupportActionOrigin {
        self.action_origin
    }
}

pub(super) fn require_non_empty(
    label: &'static str,
    value: impl Into<String>,
) -> Result<String, StoreError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(super::super::classification_error(format!(
            "subscription-support operational {label} evidence must be non-empty"
        )));
    }
    Ok(value)
}

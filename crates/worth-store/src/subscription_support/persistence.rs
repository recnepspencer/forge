use super::{
    publication_error, PublishableSubscriptionSupportArtifact,
    PublishedSubscriptionSupportArtifact, SubscriptionResumeClassification,
    SubscriptionSupportArtifactId, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportRole,
};
use crate::failure::StoreError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SubscriptionSupportStoredRecordKey {
    family_id: String,
    artifact_id: String,
}

impl SubscriptionSupportStoredRecordKey {
    pub fn new(
        family_id: &SubscriptionSupportFamilyId,
        artifact_id: &SubscriptionSupportArtifactId,
    ) -> Self {
        Self {
            family_id: family_id.as_str().to_string(),
            artifact_id: artifact_id.as_str().to_string(),
        }
    }

    pub fn family_id(&self) -> &str {
        &self.family_id
    }

    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }

    pub(crate) fn storage_key(&self) -> String {
        format!("{}\u{1f}{}", self.family_id, self.artifact_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriptionSupportStoredRecordSet {
    key: SubscriptionSupportStoredRecordKey,
    family_kind: SubscriptionSupportFamilyKind,
    role: SubscriptionSupportRole,
    declaration_digest: String,
    artifact_digest: String,
    payload_digest: String,
    basis_digest: String,
    cursor_digest: String,
    checkpoint_digest: String,
    schema_digest: String,
    compatibility_binding: String,
    compatibility_digest: String,
    initial_classification: Option<SubscriptionResumeClassification>,
    restart_shard: Option<String>,
}

impl SubscriptionSupportStoredRecordSet {
    pub(crate) fn from_publishable_and_published(
        publishable: &PublishableSubscriptionSupportArtifact,
        published: &PublishedSubscriptionSupportArtifact,
    ) -> Result<Self, StoreError> {
        let record = Self {
            key: SubscriptionSupportStoredRecordKey::new(
                published.declaration.family_id(),
                published.artifact_id(),
            ),
            family_kind: published.declaration.family_kind(),
            role: published.declaration.role(),
            declaration_digest: published
                .declaration
                .declaration_digest()
                .as_str()
                .to_string(),
            artifact_digest: published.artifact_digest().to_string(),
            payload_digest: published.declaration.payload_digest().as_str().to_string(),
            basis_digest: publishable.basis.stable_basis_digest.clone(),
            cursor_digest: publishable.cursor.cursor_digest.clone(),
            checkpoint_digest: publishable.checkpoint.checkpoint_digest.clone(),
            schema_digest: publishable.schema.schema_digest.clone(),
            compatibility_binding: publishable
                .declaration
                .declaration
                .compatibility_binding
                .clone(),
            compatibility_digest: publishable.compatibility.compatibility_digest.clone(),
            initial_classification: None,
            restart_shard: Some(restart_shard_for_family(published.declaration.family_id())),
        };
        record.validate()?;
        Ok(record)
    }

    pub fn key(&self) -> &SubscriptionSupportStoredRecordKey {
        &self.key
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub fn role(&self) -> SubscriptionSupportRole {
        self.role
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn payload_digest(&self) -> &str {
        &self.payload_digest
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
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

    pub fn schema_digest(&self) -> &str {
        &self.schema_digest
    }

    pub fn compatibility_digest(&self) -> &str {
        &self.compatibility_digest
    }

    pub fn compatibility_binding(&self) -> &str {
        &self.compatibility_binding
    }

    pub fn initial_classification_index(&self) -> Option<String> {
        self.initial_classification
            .map(|classification| format!("{classification:?}"))
    }

    pub fn restart_shard(&self) -> Option<&str> {
        self.restart_shard.as_deref()
    }

    pub fn validate(&self) -> Result<(), StoreError> {
        let required = [
            ("family id", self.key.family_id()),
            ("artifact id", self.key.artifact_id()),
            ("declaration digest", &self.declaration_digest),
            ("artifact digest", &self.artifact_digest),
            ("payload digest", &self.payload_digest),
            ("basis digest", &self.basis_digest),
            ("cursor digest", &self.cursor_digest),
            ("checkpoint digest", &self.checkpoint_digest),
            ("schema digest", &self.schema_digest),
            ("compatibility binding", &self.compatibility_binding),
            ("compatibility digest", &self.compatibility_digest),
        ];
        for (label, value) in required {
            if value.trim().is_empty() {
                return Err(publication_error(format!(
                    "subscription-support durable record set has empty {label}"
                )));
            }
        }
        let expected_prefix = format!("subscription-support:{}:", self.key.family_id());
        if !self.key.artifact_id().starts_with(&expected_prefix) {
            return Err(publication_error(
                "subscription-support durable record artifact id is not family-bound",
            ));
        }
        let expected_restart_shard =
            format!("subscription-support-restart:{}", self.key.family_id());
        if self.restart_shard.as_deref() != Some(expected_restart_shard.as_str()) {
            return Err(publication_error(
                "subscription-support durable record restart shard is not family-bound",
            ));
        }
        Ok(())
    }
}

pub(crate) fn restart_shard_for_family(family_id: &SubscriptionSupportFamilyId) -> String {
    format!("subscription-support-restart:{}", family_id.as_str())
}

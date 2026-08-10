use super::super::{
    classification_error, SubscriptionSupportArtifactId, SubscriptionSupportClassificationReport,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportRuntimeHandoffRequest {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    artifact_id: SubscriptionSupportArtifactId,
    source_runtime_owner: String,
    target_runtime_owner: String,
}

impl SubscriptionSupportRuntimeHandoffRequest {
    pub fn new(
        family_id: SubscriptionSupportFamilyId,
        family_kind: SubscriptionSupportFamilyKind,
        artifact_id: SubscriptionSupportArtifactId,
        source_runtime_owner: impl Into<String>,
        target_runtime_owner: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let request = Self {
            family_id,
            family_kind,
            artifact_id,
            source_runtime_owner: source_runtime_owner.into(),
            target_runtime_owner: target_runtime_owner.into(),
        };
        request.validate()?;
        Ok(request)
    }

    pub(crate) fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub(crate) fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub(crate) fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }

    fn validate(&self) -> Result<(), StoreError> {
        if self.source_runtime_owner.trim().is_empty() {
            return Err(classification_error(
                "subscription-support runtime handoff requires a source runtime owner",
            ));
        }
        if self.target_runtime_owner.trim().is_empty() {
            return Err(classification_error(
                "subscription-support runtime handoff requires a target runtime owner",
            ));
        }
        if self.source_runtime_owner == self.target_runtime_owner {
            return Err(classification_error(
                "subscription-support runtime handoff requires distinct runtime owners",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportRuntimeHandoffReport {
    source_runtime_owner: String,
    target_runtime_owner: String,
    durable_report: SubscriptionSupportClassificationReport,
    delivery_session_memory_persisted: bool,
}

impl SubscriptionSupportRuntimeHandoffReport {
    pub(crate) fn new(
        request: &SubscriptionSupportRuntimeHandoffRequest,
        durable_report: SubscriptionSupportClassificationReport,
    ) -> Self {
        Self {
            source_runtime_owner: request.source_runtime_owner.clone(),
            target_runtime_owner: request.target_runtime_owner.clone(),
            durable_report,
            delivery_session_memory_persisted: false,
        }
    }

    pub fn durable_report(&self) -> &SubscriptionSupportClassificationReport {
        &self.durable_report
    }

    pub fn delivery_session_memory_persisted(&self) -> bool {
        self.delivery_session_memory_persisted
    }
}

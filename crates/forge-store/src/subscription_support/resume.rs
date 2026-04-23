use super::{
    classification_error, FetchedSubscriptionSupportArtifact,
    SubscriptionSupportClassificationPlan, SubscriptionSupportFamilyKind,
};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportResumeEvidence {
    expected_family_kind: SubscriptionSupportFamilyKind,
    basis_digest: String,
    cursor_digest: String,
    checkpoint_digest: String,
    schema_digest: String,
    compatibility_digest: String,
    support_artifact_digest: String,
    retained_rebuild_basis_digest: Option<String>,
    observed_payload_bytes: u64,
    session_memory_present: bool,
    placement_unavailable: bool,
}

impl SubscriptionSupportResumeEvidence {
    pub fn new(
        expected_family_kind: SubscriptionSupportFamilyKind,
        basis_digest: impl Into<String>,
        cursor_digest: impl Into<String>,
        checkpoint_digest: impl Into<String>,
        schema_digest: impl Into<String>,
        compatibility_digest: impl Into<String>,
        support_artifact_digest: impl Into<String>,
        observed_payload_bytes: u64,
        session_memory_present: bool,
    ) -> Result<Self, StoreError> {
        let evidence = Self {
            expected_family_kind,
            basis_digest: basis_digest.into(),
            cursor_digest: cursor_digest.into(),
            checkpoint_digest: checkpoint_digest.into(),
            schema_digest: schema_digest.into(),
            compatibility_digest: compatibility_digest.into(),
            support_artifact_digest: support_artifact_digest.into(),
            retained_rebuild_basis_digest: None,
            observed_payload_bytes,
            session_memory_present,
            placement_unavailable: false,
        };
        evidence.validate()?;
        Ok(evidence)
    }

    pub fn matching(
        fetched: &FetchedSubscriptionSupportArtifact,
        observed_payload_bytes: u64,
        session_memory_present: bool,
    ) -> Result<Self, StoreError> {
        let record_set = fetched.record_set();
        Self::new(
            record_set.family_kind(),
            record_set.basis_digest(),
            record_set.cursor_digest(),
            record_set.checkpoint_digest(),
            record_set.schema_digest(),
            record_set.compatibility_digest(),
            record_set.artifact_digest(),
            observed_payload_bytes,
            session_memory_present,
        )
    }

    pub fn with_basis_digest(
        mut self,
        basis_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        self.basis_digest = basis_digest.into();
        self.validate()?;
        Ok(self)
    }

    pub fn with_cursor_digest(
        mut self,
        cursor_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        self.cursor_digest = cursor_digest.into();
        self.validate()?;
        Ok(self)
    }

    pub fn with_checkpoint_digest(
        mut self,
        checkpoint_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        self.checkpoint_digest = checkpoint_digest.into();
        self.validate()?;
        Ok(self)
    }

    pub fn with_schema_digest(
        mut self,
        schema_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        self.schema_digest = schema_digest.into();
        self.validate()?;
        Ok(self)
    }

    pub fn with_compatibility_digest(
        mut self,
        compatibility_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        self.compatibility_digest = compatibility_digest.into();
        self.validate()?;
        Ok(self)
    }

    pub fn with_support_artifact_digest(
        mut self,
        support_artifact_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        self.support_artifact_digest = support_artifact_digest.into();
        self.validate()?;
        Ok(self)
    }

    pub fn with_retained_rebuild_basis_digest(
        mut self,
        retained_rebuild_basis_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        self.retained_rebuild_basis_digest = Some(retained_rebuild_basis_digest.into());
        self.validate()?;
        Ok(self)
    }

    pub fn with_expected_family_kind(mut self, family_kind: SubscriptionSupportFamilyKind) -> Self {
        self.expected_family_kind = family_kind;
        self
    }

    pub fn without_session_memory(mut self) -> Self {
        self.session_memory_present = false;
        self
    }

    pub fn with_placement_unavailable(mut self) -> Self {
        self.placement_unavailable = true;
        self
    }

    pub fn expected_family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.expected_family_kind
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

    pub fn support_artifact_digest(&self) -> &str {
        &self.support_artifact_digest
    }

    pub fn observed_payload_bytes(&self) -> u64 {
        self.observed_payload_bytes
    }

    pub fn retained_rebuild_basis_digest(&self) -> Option<&str> {
        self.retained_rebuild_basis_digest.as_deref()
    }

    pub fn session_memory_present(&self) -> bool {
        self.session_memory_present
    }

    pub fn placement_unavailable(&self) -> bool {
        self.placement_unavailable
    }

    fn validate(&self) -> Result<(), StoreError> {
        for (label, value) in [
            ("basis digest", &self.basis_digest),
            ("cursor digest", &self.cursor_digest),
            ("checkpoint digest", &self.checkpoint_digest),
            ("schema digest", &self.schema_digest),
            ("compatibility digest", &self.compatibility_digest),
            ("support artifact digest", &self.support_artifact_digest),
        ] {
            if value.trim().is_empty() {
                return Err(classification_error(format!(
                    "subscription-support resume evidence requires non-empty {label}"
                )));
            }
        }
        if self
            .retained_rebuild_basis_digest
            .as_ref()
            .is_some_and(|digest| digest.trim().is_empty())
        {
            return Err(classification_error(
                "subscription-support rebuild classification requires non-empty retained rebuild basis evidence",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportResumeRequest {
    fetched: FetchedSubscriptionSupportArtifact,
    evidence: SubscriptionSupportResumeEvidence,
    plan: SubscriptionSupportClassificationPlan,
}

impl SubscriptionSupportResumeRequest {
    pub fn new(
        fetched: FetchedSubscriptionSupportArtifact,
        evidence: SubscriptionSupportResumeEvidence,
        plan: SubscriptionSupportClassificationPlan,
    ) -> Self {
        Self {
            fetched,
            evidence,
            plan,
        }
    }

    pub(crate) fn fetched(&self) -> &FetchedSubscriptionSupportArtifact {
        &self.fetched
    }

    pub(crate) fn evidence(&self) -> &SubscriptionSupportResumeEvidence {
        &self.evidence
    }

    pub(crate) fn plan(&self) -> &SubscriptionSupportClassificationPlan {
        &self.plan
    }
}

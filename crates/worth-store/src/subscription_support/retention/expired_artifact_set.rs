use super::affected_set::SupportAffectedSet;
use super::evidence_validation::require_non_empty;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExpiredSupportArtifactSet {
    affected_set: SupportAffectedSet,
    policy_reason: String,
}

impl ExpiredSupportArtifactSet {
    pub(crate) fn new(
        affected_set: SupportAffectedSet,
        policy_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            affected_set,
            policy_reason: require_non_empty("policy expiration", policy_reason)?,
        })
    }

    pub fn affected_set(&self) -> &SupportAffectedSet {
        &self.affected_set
    }

    pub fn policy_reason(&self) -> &str {
        &self.policy_reason
    }
}

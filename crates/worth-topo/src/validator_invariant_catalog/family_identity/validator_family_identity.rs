use crate::validation::TopologyValidationRuleIdentity;

use super::legality_family_identity_digest;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorthTopologyValidatorFamilyIdentity {
    namespace: String,
    name: String,
    semantic_version: String,
    stable_key: String,
    identity_digest: String,
}

impl WorthTopologyValidatorFamilyIdentity {
    pub(in crate::validator_invariant_catalog) fn from_registered_rule(
        rule_identity: TopologyValidationRuleIdentity,
    ) -> Self {
        let semantic_version = format!("v{}", rule_identity.version());
        let stable_key = format!(
            "validator:{}:{}:{}",
            rule_identity.namespace(),
            rule_identity.name(),
            semantic_version
        );
        let identity_digest = legality_family_identity_digest(&[
            "validator",
            rule_identity.namespace(),
            rule_identity.name(),
            semantic_version.as_str(),
        ]);
        Self {
            namespace: rule_identity.namespace().to_string(),
            name: rule_identity.name().to_string(),
            semantic_version,
            stable_key,
            identity_digest,
        }
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn semantic_version(&self) -> &str {
        &self.semantic_version
    }

    pub fn stable_key(&self) -> &str {
        &self.stable_key
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }
}

use crate::identity::hash_parts;

use super::{BranchAccessGrantClass, PolicyRuleSnapshot};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BranchAccessGrant {
    branch_identity: String,
    grant_class: BranchAccessGrantClass,
    denial_class: Option<String>,
    policy_digest: String,
    digest: String,
}

impl BranchAccessGrant {
    pub fn synthetic_granted(
        branch_identity: impl Into<String>,
        policy: &PolicyRuleSnapshot,
    ) -> Self {
        Self::synthetic(
            branch_identity,
            BranchAccessGrantClass::Granted,
            None,
            policy,
        )
    }

    pub fn synthetic_denied(
        branch_identity: impl Into<String>,
        denial_class: impl Into<String>,
        policy: &PolicyRuleSnapshot,
    ) -> Self {
        Self::synthetic(
            branch_identity,
            BranchAccessGrantClass::Denied,
            Some(denial_class.into()),
            policy,
        )
    }

    fn synthetic(
        branch_identity: impl Into<String>,
        grant_class: BranchAccessGrantClass,
        denial_class: Option<String>,
        policy: &PolicyRuleSnapshot,
    ) -> Self {
        let branch_identity = branch_identity.into();
        let policy_digest = policy.digest().to_string();
        let digest = hash_parts(&[
            format!("branch:{branch_identity}"),
            format!("grant:{}", grant_class.as_str()),
            format!("denial:{}", denial_class.as_deref().unwrap_or("none")),
            format!("policy:{policy_digest}"),
        ]);
        Self {
            branch_identity,
            grant_class,
            denial_class,
            policy_digest,
            digest,
        }
    }

    pub fn branch_identity(&self) -> &str {
        &self.branch_identity
    }

    pub fn grant_class(&self) -> BranchAccessGrantClass {
        self.grant_class
    }

    pub fn denial_class(&self) -> Option<&str> {
        self.denial_class.as_deref()
    }

    pub fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

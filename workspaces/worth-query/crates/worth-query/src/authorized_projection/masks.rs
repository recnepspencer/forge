use crate::identity::hash_parts;
use crate::policy_basis::AdmittedPolicyTenantContext;
pub use crate::policy_basis::{PolicyAspectMask, ProjectionVisibility};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyMaskSnapshot {
    policy_digest: String,
    mask: PolicyAspectMask,
    digest: String,
}

impl PolicyMaskSnapshot {
    pub(crate) fn from_admitted_policy(admitted: &AdmittedPolicyTenantContext) -> Option<Self> {
        admitted
            .policy_basis()
            .projection_mask()
            .cloned()
            .map(|mask| {
                Self::from_policy_digest(admitted.bundle().policy_digest().to_string(), mask)
            })
    }

    pub fn synthetic_authority(policy_digest: impl Into<String>, mask: PolicyAspectMask) -> Self {
        Self::from_policy_digest(policy_digest.into(), mask)
    }

    fn from_policy_digest(policy_digest: String, mask: PolicyAspectMask) -> Self {
        let mut parts = vec![
            "policy_mask_snapshot".to_string(),
            format!("policy:{policy_digest}"),
        ];
        parts.extend(mask.digest_parts());
        Self {
            policy_digest,
            mask,
            digest: hash_parts(&parts),
        }
    }

    pub fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub(crate) fn mask(&self) -> &PolicyAspectMask {
        &self.mask
    }
}

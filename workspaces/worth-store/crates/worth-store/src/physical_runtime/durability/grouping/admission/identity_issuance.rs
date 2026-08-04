use std::num::NonZeroU64;

use sha2::{Digest, Sha256};

use super::{
    PhysicalDurabilityGroupAdmissionDenial, PhysicalDurabilityGroupIdentity,
    PhysicalDurabilityGroupingRuntimeOwner,
};
use crate::physical_runtime::durability::grouping::unique_membership::PhysicalGroupMembershipDigest;

const GROUP_IDENTITY_DOMAIN: &[u8] = b"store.physical.durability-group.v1";

impl PhysicalDurabilityGroupingRuntimeOwner {
    pub(super) fn issue_identity(
        &self,
        membership: PhysicalGroupMembershipDigest,
    ) -> Result<PhysicalDurabilityGroupIdentity, PhysicalDurabilityGroupAdmissionDenial> {
        let mut sequence = self
            .next_sequence
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let current = *sequence;
        *sequence = NonZeroU64::new(
            current
                .get()
                .checked_add(1)
                .ok_or(PhysicalDurabilityGroupAdmissionDenial::GroupIdentityExhausted)?,
        )
        .expect("a checked positive increment remains nonzero");
        let mut digest = Sha256::new();
        digest.update(GROUP_IDENTITY_DOMAIN);
        digest.update(self.store.bytes());
        digest.update(self.runtime.get().to_le_bytes());
        digest.update(self.policy.bytes());
        digest.update(current.get().to_le_bytes());
        digest.update(membership.bytes());
        Ok(PhysicalDurabilityGroupIdentity(digest.finalize().into()))
    }
}

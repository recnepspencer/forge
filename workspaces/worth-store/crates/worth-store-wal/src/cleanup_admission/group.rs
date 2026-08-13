use std::collections::{BTreeMap, BTreeSet};

use sha2::{Digest, Sha256};

use super::{CheckpointCoveredWalCleanupDenial, WalMemberBinding, MEMBERSHIP_DOMAIN};

pub(super) fn validate(
    members: &[WalMemberBinding],
) -> Result<(), CheckpointCoveredWalCleanupDenial> {
    let mut groups = BTreeMap::<[u8; 32], Vec<WalMemberBinding>>::new();
    for member in members {
        groups
            .entry(member.group_identity)
            .or_default()
            .push(*member);
    }
    for group in groups.values_mut() {
        group.sort_unstable_by_key(|member| member.ordinal);
        let first = group
            .first()
            .ok_or(CheckpointCoveredWalCleanupDenial::InvalidWalMember)?;
        let mut member_ids = BTreeSet::new();
        let mut operation_ids = BTreeSet::new();
        if group.len() != first.count as usize
            || group.iter().enumerate().any(|(index, member)| {
                member.ordinal as usize != index + 1
                    || member.count != first.count
                    || member.membership != first.membership
                    || !member_ids.insert(member.member_identity)
                    || !operation_ids.insert(member.basis.idempotency)
            })
            || membership_digest(group) != first.membership
        {
            return Err(CheckpointCoveredWalCleanupDenial::InvalidWalMember);
        }
    }
    Ok(())
}

fn membership_digest(group: &[WalMemberBinding]) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(MEMBERSHIP_DOMAIN);
    digest.update((group.len() as u64).to_le_bytes());
    for member in group {
        digest.update(member.basis.store);
        digest.update(member.basis.runtime.to_le_bytes());
        digest.update(member.basis.operation.to_le_bytes());
        digest.update(member.member_identity);
        digest.update(member.basis.idempotency);
    }
    digest.finalize().into()
}

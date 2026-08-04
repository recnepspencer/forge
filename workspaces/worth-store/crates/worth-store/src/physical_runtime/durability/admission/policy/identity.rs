use sha2::{Digest, Sha256};
use worth_store_physical_backend::PhysicalDurabilityAdmissionIdentity;

use super::{
    GroupPolicy, PhysicalCheckpointPolicy, PhysicalDurabilityPolicyIdentity,
    PhysicalIdempotencyPolicy, PhysicalWalPolicy,
};

const POLICY_IDENTITY_DOMAIN: &[u8] = b"worth.store.physical.durability.policy.v1";

pub(super) fn policy_identity(
    basis: PhysicalDurabilityAdmissionIdentity,
    group: GroupPolicy,
    wal: PhysicalWalPolicy,
    idempotency: PhysicalIdempotencyPolicy,
    checkpoint: PhysicalCheckpointPolicy,
) -> PhysicalDurabilityPolicyIdentity {
    let mut digest = Sha256::new();
    digest.update((POLICY_IDENTITY_DOMAIN.len() as u64).to_le_bytes());
    digest.update(POLICY_IDENTITY_DOMAIN);
    digest.update(basis.bytes());
    digest.update(group.limit.get().get().to_le_bytes());
    digest.update(group.delay.signal_duration().get().to_le_bytes());
    digest.update(wal.segment_byte_limit().get().get().to_le_bytes());
    digest.update(wal.segment_inventory_limit().get().get().to_le_bytes());
    digest.update(idempotency.retention.get().get().to_le_bytes());
    digest.update(idempotency.pending_unresolved.get().get().to_le_bytes());
    digest.update(idempotency.live_bindings.get().get().to_le_bytes());
    digest.update(checkpoint.memory.get().get().to_le_bytes());
    digest.update(checkpoint.retained_wal_tail.get().get().to_le_bytes());
    PhysicalDurabilityPolicyIdentity(digest.finalize().into())
}

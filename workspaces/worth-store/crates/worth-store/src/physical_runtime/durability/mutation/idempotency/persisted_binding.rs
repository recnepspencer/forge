use super::attempt_binding::AllocatedPhysicalMutationAttemptBinding;
use super::registry::PhysicalMutationUnresolvedBindingObservation;

mod decoding;
pub(in crate::physical_runtime) use decoding::{
    decode_binding_basis, CanonicalBindingCursor, PhysicalBindingDecodingContext,
    PhysicalPersistedBindingDecodeDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::physical_runtime) struct PersistedPhysicalMutationAttemptBinding {
    key: super::PhysicalMutationIdempotencyKey,
    fingerprint: crate::physical_runtime::PhysicalMutationRequestFingerprint,
    mutation: crate::physical_runtime::PhysicalMutationIdentity,
    group: crate::physical_runtime::PhysicalDurabilityGroupMemberBinding,
    member: crate::physical_runtime::PhysicalWalMemberBasis,
    redo_digest: [u8; 32],
    bytes: Box<[u8]>,
}

impl PersistedPhysicalMutationAttemptBinding {
    pub(in crate::physical_runtime) fn from_allocated(
        binding: &AllocatedPhysicalMutationAttemptBinding,
    ) -> Self {
        let mut persisted = Self {
            key: binding.key().clone(),
            fingerprint: binding.fingerprint(),
            mutation: binding.mutation_identity(),
            group: binding.group_binding(),
            member: binding.member(),
            redo_digest: binding.redo_digest(),
            bytes: Box::default(),
        };
        persisted.bytes = persisted.encode().into_boxed_slice();
        debug_assert_eq!(persisted.bytes(), binding.encode_persisted());
        persisted
    }

    pub(in crate::physical_runtime) const fn observation(
        &self,
    ) -> PhysicalMutationUnresolvedBindingObservation {
        PhysicalMutationUnresolvedBindingObservation::new(
            self.key.identity(),
            self.fingerprint,
            self.mutation,
        )
    }

    pub(in crate::physical_runtime) const fn group(
        &self,
    ) -> crate::physical_runtime::PhysicalDurabilityGroupMemberBinding {
        self.group
    }

    pub(in crate::physical_runtime) const fn member(
        &self,
    ) -> crate::physical_runtime::PhysicalWalMemberBasis {
        self.member
    }

    pub(in crate::physical_runtime) const fn idempotency_identity(
        &self,
    ) -> super::PhysicalMutationIdempotencyKeyIdentity {
        self.key.identity()
    }

    pub(in crate::physical_runtime) const fn policy_identity(
        &self,
    ) -> crate::physical_runtime::PhysicalDurabilityPolicyIdentity {
        self.key.lease().policy_identity()
    }

    pub(in crate::physical_runtime) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub(super) fn key(&self) -> &super::PhysicalMutationIdempotencyKey {
        &self.key
    }

    pub(in crate::physical_runtime) const fn fingerprint(
        &self,
    ) -> crate::physical_runtime::PhysicalMutationRequestFingerprint {
        self.fingerprint
    }

    pub(in crate::physical_runtime) const fn mutation(
        &self,
    ) -> crate::physical_runtime::PhysicalMutationIdentity {
        self.mutation
    }

    fn encode(&self) -> Vec<u8> {
        let lease = self.key.lease();
        let range = self.member.lsn_range();
        let mut bytes = Vec::with_capacity(320);
        write_field(&mut bytes, b"store.physical.mutation-attempt-binding.v1");
        write_field(&mut bytes, &self.key.identity().bytes());
        write_field(&mut bytes, &lease.store_identity().bytes());
        write_field(&mut bytes, &lease.policy_identity().bytes());
        bytes.extend_from_slice(&lease.issuance_generation().get().to_le_bytes());
        bytes.extend_from_slice(&lease.expiry_generation().get().to_le_bytes());
        write_field(&mut bytes, &self.key.caller_material().bytes());
        write_field(&mut bytes, &self.fingerprint.bytes());
        write_field(&mut bytes, &self.mutation.store_identity().bytes());
        bytes.extend_from_slice(&self.mutation.runtime_identity().get().to_le_bytes());
        bytes.extend_from_slice(&self.mutation.lifecycle_generation().to_le_bytes());
        bytes.extend_from_slice(&self.mutation.operation_identity().get().to_le_bytes());
        write_field(&mut bytes, &self.group.group_identity().bytes());
        bytes.extend_from_slice(&self.group.ordinal().get().to_le_bytes());
        bytes.extend_from_slice(&self.group.member_count().get().to_le_bytes());
        write_field(&mut bytes, &self.group.membership_digest());
        write_field(&mut bytes, &self.member.member_identity().bytes());
        bytes.extend_from_slice(&range.start().get().to_le_bytes());
        bytes.extend_from_slice(&range.end_exclusive().get().to_le_bytes());
        write_field(&mut bytes, &self.redo_digest);
        bytes
    }
}

fn write_field(target: &mut Vec<u8>, field: &[u8]) {
    target.extend_from_slice(&(field.len() as u64).to_le_bytes());
    target.extend_from_slice(field);
}

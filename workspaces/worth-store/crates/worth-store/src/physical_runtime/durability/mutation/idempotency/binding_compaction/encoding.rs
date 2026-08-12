use super::super::registry::PhysicalMutationBindingBasis;

pub(super) const COMPACTION_RECORD_DOMAIN: &[u8] =
    worth_store_physical_format::PHYSICAL_MUTATION_BINDING_COMPACTION_RECORD_DOMAIN;
pub(super) const STATE_UNSEALED: u8 = 1;
pub(super) const STATE_GROUP_SEALED: u8 = 2;
pub(super) const STATE_TERMINAL: u8 = 3;
pub(super) const STATE_WAL_BOUND: u8 = 4;

pub(super) fn encode_unsealed(basis: &PhysicalMutationBindingBasis) -> Vec<u8> {
    encode_basis(STATE_UNSEALED, basis)
}

pub(super) fn encode_group_sealed(
    basis: &PhysicalMutationBindingBasis,
    group: crate::physical_runtime::PhysicalDurabilityGroupMemberBinding,
) -> Vec<u8> {
    let mut encoded = encode_basis(STATE_GROUP_SEALED, basis);
    write_field(&mut encoded, &group.group_identity().bytes());
    write_field(&mut encoded, &group.member_identity().bytes());
    encoded.extend_from_slice(&group.ordinal().get().to_le_bytes());
    encoded.extend_from_slice(&group.member_count().get().to_le_bytes());
    write_field(&mut encoded, &group.membership_digest());
    encoded
}

pub(super) fn encode_terminal(
    basis: &PhysicalMutationBindingBasis,
    fate: &super::super::fate::PersistedPhysicalMutationFate,
) -> Vec<u8> {
    let mut encoded = encode_basis(STATE_TERMINAL, basis);
    fate.encode(&mut encoded);
    encoded
}

pub(super) fn encode_wal_bound(
    persisted: &super::super::PersistedPhysicalMutationAttemptBinding,
) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(persisted.bytes().len() + 80);
    write_field(&mut encoded, COMPACTION_RECORD_DOMAIN);
    encoded.push(STATE_WAL_BOUND);
    write_field(&mut encoded, persisted.bytes());
    encoded
}

fn encode_basis(state: u8, basis: &PhysicalMutationBindingBasis) -> Vec<u8> {
    let key = basis.key();
    let lease = key.lease();
    let mutation = basis.mutation();
    let mut encoded = Vec::with_capacity(256);
    write_field(&mut encoded, COMPACTION_RECORD_DOMAIN);
    encoded.push(state);
    write_field(&mut encoded, &key.identity().bytes());
    write_field(&mut encoded, &lease.store_identity().bytes());
    write_field(&mut encoded, &lease.policy_identity().bytes());
    encoded.extend_from_slice(&lease.issuance_generation().get().to_le_bytes());
    encoded.extend_from_slice(&lease.expiry_generation().get().to_le_bytes());
    write_field(&mut encoded, &key.caller_material().bytes());
    write_field(&mut encoded, &basis.fingerprint().bytes());
    write_field(&mut encoded, &mutation.store_identity().bytes());
    encoded.extend_from_slice(&mutation.runtime_identity().get().to_le_bytes());
    encoded.extend_from_slice(&mutation.lifecycle_generation().to_le_bytes());
    encoded.extend_from_slice(&mutation.operation_identity().get().to_le_bytes());
    encoded
}

fn write_field(target: &mut Vec<u8>, field: &[u8]) {
    target.extend_from_slice(&(field.len() as u64).to_le_bytes());
    target.extend_from_slice(field);
}

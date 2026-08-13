use sha2::{Digest, Sha256};

use super::{
    binding_basis, group, BindingBasis, CheckpointCoveredWalCleanupDenial, Cursor,
    WalMemberBinding, IDEMPOTENCY_KEY_DOMAIN, MEMBERSHIP_DOMAIN,
};

const STORE: [u8; 16] = [7; 16];
const POLICY: [u8; 32] = [11; 32];
const MATERIAL: [u8; 32] = [13; 32];

#[test]
fn binding_basis_requires_the_checkpoint_retention_exactly() {
    let bytes = encoded_basis(31, 39);
    let admitted = binding_basis(&mut Cursor::new(&bytes), STORE, POLICY, 8).unwrap();
    assert_eq!(admitted.store, STORE);
    assert_eq!(admitted.runtime, 17);
    assert_eq!(admitted.operation, 23);

    assert!(matches!(
        binding_basis(&mut Cursor::new(&bytes), STORE, POLICY, 7),
        Err(CheckpointCoveredWalCleanupDenial::InvalidWalMember)
    ));
}

#[test]
fn group_admission_requires_complete_exact_membership() {
    let mut members = members();
    assert!(group::validate(&members).is_ok());

    members.pop();
    assert_eq!(
        group::validate(&members),
        Err(CheckpointCoveredWalCleanupDenial::InvalidWalMember)
    );
}

fn members() -> Vec<WalMemberBinding> {
    let bases = [basis(23, [3; 32]), basis(29, [5; 32])];
    let group_identity = [17; 32];
    let mut digest = Sha256::new();
    digest.update(MEMBERSHIP_DOMAIN);
    digest.update(2_u64.to_le_bytes());
    for basis in bases {
        digest.update(basis.store);
        digest.update(basis.runtime.to_le_bytes());
        digest.update(basis.operation.to_le_bytes());
        digest.update(super::member_identity_for(basis));
        digest.update(basis.idempotency);
    }
    let membership = digest.finalize().into();
    bases
        .into_iter()
        .enumerate()
        .map(|(index, basis)| WalMemberBinding {
            basis,
            group_identity,
            member_identity: super::member_identity_for(basis),
            ordinal: index as u32 + 1,
            count: 2,
            membership,
        })
        .collect()
}

const fn basis(operation: u64, idempotency: [u8; 32]) -> BindingBasis {
    BindingBasis {
        idempotency,
        store: STORE,
        runtime: 17,
        operation,
    }
}

fn encoded_basis(issuance: u64, expiry: u64) -> Vec<u8> {
    let mut digest = Sha256::new();
    digest.update((IDEMPOTENCY_KEY_DOMAIN.len() as u64).to_le_bytes());
    digest.update(IDEMPOTENCY_KEY_DOMAIN);
    digest.update(STORE);
    digest.update(POLICY);
    digest.update(issuance.to_le_bytes());
    digest.update(expiry.to_le_bytes());
    digest.update(MATERIAL);
    let identity: [u8; 32] = digest.finalize().into();

    let mut bytes = Vec::new();
    field(&mut bytes, &identity);
    field(&mut bytes, &STORE);
    field(&mut bytes, &POLICY);
    bytes.extend_from_slice(&issuance.to_le_bytes());
    bytes.extend_from_slice(&expiry.to_le_bytes());
    field(&mut bytes, &MATERIAL);
    field(&mut bytes, &[19; 32]);
    field(&mut bytes, &STORE);
    bytes.extend_from_slice(&17_u64.to_le_bytes());
    bytes.extend_from_slice(&18_u64.to_le_bytes());
    bytes.extend_from_slice(&23_u64.to_le_bytes());
    bytes
}

fn field(bytes: &mut Vec<u8>, value: &[u8]) {
    bytes.extend_from_slice(&(value.len() as u64).to_le_bytes());
    bytes.extend_from_slice(value);
}

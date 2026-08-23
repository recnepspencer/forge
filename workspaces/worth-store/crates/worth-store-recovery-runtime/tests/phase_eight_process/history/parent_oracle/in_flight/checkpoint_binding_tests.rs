use sha2::{Digest, Sha256};

use super::super::canonical_membership_placement::RecordIdentity;
use super::scan_checkpoint_binding;

const ATTEMPT_DOMAIN: &[u8] = b"store.physical.mutation-attempt-binding.v1";
const COMPACTION_DOMAIN: &[u8] =
    worth_store_physical_format::PHYSICAL_MUTATION_BINDING_COMPACTION_RECORD_DOMAIN;
const KEY_DOMAIN: &[u8] = b"store.physical.mutation.idempotency-key.v1";

#[test]
fn completed_checkpoint_terminal_binds_the_exact_record() {
    let expected = record(1, 7);
    let (idempotency, checkpoint, redo_digest) = terminal(1, expected);
    assert_eq!(
        scan_checkpoint_binding(&checkpoint, &idempotency, expected, Some(&redo_digest)).unwrap(),
        1
    );
}

#[test]
fn completed_checkpoint_terminal_rejects_a_rehashed_redo_digest() {
    let expected = record(1, 7);
    let (idempotency, checkpoint, redo_digest) = terminal(1, expected);
    let mut mutated = checkpoint.clone();
    let offset = mutated
        .windows(redo_digest.len())
        .position(|window| window == redo_digest)
        .expect("synthetic checkpoint contains its redo digest");
    mutated[offset] ^= 1;
    let crc = crc32c(&mutated[..mutated.len() - 4]);
    let crc_offset = mutated.len() - 4;
    mutated[crc_offset..].copy_from_slice(&crc.to_le_bytes());
    assert_eq!(
        scan_checkpoint_binding(&mutated, &idempotency, expected, Some(&redo_digest)).unwrap(),
        0
    );
}

#[test]
fn completed_checkpoint_terminal_rejects_a_rehashed_wrong_idempotency() {
    let expected = record(1, 7);
    let (idempotency, checkpoint, _) = terminal(2, expected);
    let (wrong_idempotency, _, redo_digest) = terminal(1, expected);
    assert_eq!(
        scan_checkpoint_binding(
            &checkpoint,
            &wrong_idempotency,
            expected,
            Some(&redo_digest),
        )
        .unwrap(),
        0
    );
    assert_ne!(idempotency, wrong_idempotency);
}

#[test]
fn completed_checkpoint_terminal_rejects_a_rehashed_wrong_record() {
    let expected = record(1, 7);
    let observed = record(2, 8);
    let (idempotency, checkpoint, redo_digest) = terminal(1, observed);
    assert_eq!(
        scan_checkpoint_binding(&checkpoint, &idempotency, expected, Some(&redo_digest)).unwrap(),
        0
    );
}

#[test]
fn duplicate_completed_checkpoint_terminals_are_not_exactly_one() {
    let expected = record(1, 7);
    let (idempotency, one, redo_digest) = terminal(1, expected);
    let mut duplicate = one.clone();
    duplicate.extend_from_slice(&one);
    assert_eq!(
        scan_checkpoint_binding(&duplicate, &idempotency, expected, Some(&redo_digest)).unwrap(),
        2
    );
}

fn record(epoch: u8, ordinal: u64) -> RecordIdentity {
    RecordIdentity {
        allocation_epoch: [epoch; 16],
        ordinal,
    }
}

fn terminal(seed: u8, record: RecordIdentity) -> ([u8; 32], Vec<u8>, [u8; 32]) {
    let store = [seed; 16];
    let policy = [seed.wrapping_add(10); 32];
    let material = [seed.wrapping_add(20); 32];
    let issuance = u64::from(seed);
    let expiry = issuance + 10;
    let idempotency = key(store, policy, issuance, expiry, material);
    let redo_digest = [seed.wrapping_add(60); 32];
    let binding = attempt_binding(
        idempotency,
        store,
        policy,
        issuance,
        expiry,
        material,
        seed,
        redo_digest,
    );
    let mut payload = Vec::new();
    field(&mut payload, COMPACTION_DOMAIN);
    payload.push(3);
    field(&mut payload, &idempotency);
    field(&mut payload, &store);
    field(&mut payload, &policy);
    payload.extend_from_slice(&issuance.to_le_bytes());
    payload.extend_from_slice(&expiry.to_le_bytes());
    field(&mut payload, &material);
    field(&mut payload, &[seed.wrapping_add(30); 32]);
    field(&mut payload, &store);
    payload.extend_from_slice(&1_u64.to_le_bytes());
    payload.extend_from_slice(&1_u64.to_le_bytes());
    payload.extend_from_slice(&1_u64.to_le_bytes());
    payload.push(2);
    field(&mut payload, &binding);
    payload.extend_from_slice(&1_u32.to_le_bytes());
    payload.extend_from_slice(&1_u64.to_le_bytes());
    payload.extend_from_slice(&1_u32.to_le_bytes());
    field(&mut payload, &record.allocation_epoch);
    payload.extend_from_slice(&record.ordinal.to_le_bytes());
    for _ in 0..13 {
        payload.extend_from_slice(&1_u64.to_le_bytes());
    }
    (idempotency, checkpoint_record(&payload), redo_digest)
}

fn key(
    store: [u8; 16],
    policy: [u8; 32],
    issuance: u64,
    expiry: u64,
    material: [u8; 32],
) -> [u8; 32] {
    let mut digest = Sha256::new();
    field_digest(&mut digest, KEY_DOMAIN);
    digest.update(store);
    digest.update(policy);
    digest.update(issuance.to_le_bytes());
    digest.update(expiry.to_le_bytes());
    digest.update(material);
    digest.finalize().into()
}

fn attempt_binding(
    idempotency: [u8; 32],
    store: [u8; 16],
    policy: [u8; 32],
    issuance: u64,
    expiry: u64,
    material: [u8; 32],
    seed: u8,
    redo_digest: [u8; 32],
) -> Vec<u8> {
    let mut binding = Vec::new();
    field(&mut binding, ATTEMPT_DOMAIN);
    field(&mut binding, &idempotency);
    field(&mut binding, &store);
    field(&mut binding, &policy);
    binding.extend_from_slice(&issuance.to_le_bytes());
    binding.extend_from_slice(&expiry.to_le_bytes());
    field(&mut binding, &material);
    field(&mut binding, &[seed.wrapping_add(30); 32]);
    field(&mut binding, &store);
    binding.extend_from_slice(&1_u64.to_le_bytes());
    binding.extend_from_slice(&1_u64.to_le_bytes());
    binding.extend_from_slice(&1_u64.to_le_bytes());
    field(&mut binding, &[seed; 32]);
    binding.extend_from_slice(&1_u32.to_le_bytes());
    binding.extend_from_slice(&1_u32.to_le_bytes());
    field(&mut binding, &[seed.wrapping_add(40); 32]);
    field(&mut binding, &[seed.wrapping_add(50); 32]);
    binding.extend_from_slice(&1_u64.to_le_bytes());
    binding.extend_from_slice(&2_u64.to_le_bytes());
    field(&mut binding, &redo_digest);
    binding
}

fn checkpoint_record(payload: &[u8]) -> Vec<u8> {
    let mut record = vec![0; 16];
    record[..8].copy_from_slice(b"WCP7REC\0");
    record[8] = 1;
    record[9] = 4;
    record[12..16].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    record.extend_from_slice(payload);
    record.extend_from_slice(&crc32c(&record).to_le_bytes());
    record
}

fn field(target: &mut Vec<u8>, bytes: &[u8]) {
    target.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    target.extend_from_slice(bytes);
}

fn field_digest(digest: &mut Sha256, bytes: &[u8]) {
    digest.update((bytes.len() as u64).to_le_bytes());
    digest.update(bytes);
}

fn crc32c(bytes: &[u8]) -> u32 {
    let mut value = !0_u32;
    for byte in bytes {
        value ^= u32::from(*byte);
        for _ in 0..8 {
            let mask = 0_u32.wrapping_sub(value & 1);
            value = (value >> 1) ^ (0x82f6_3b78 & mask);
        }
    }
    !value
}

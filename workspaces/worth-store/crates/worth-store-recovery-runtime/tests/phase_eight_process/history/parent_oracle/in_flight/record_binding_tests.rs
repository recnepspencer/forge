use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

use super::super::canonical_membership::ExpectedCanonicalRecord;
use super::super::canonical_membership_placement::RecordIdentity;
use super::{require_bound_record, require_bound_records};

const ATTEMPT_DOMAIN: &[u8] = b"store.physical.mutation-attempt-binding.v1";
const KEY_DOMAIN: &[u8] = b"store.physical.mutation.idempotency-key.v1";
const REDO_DOMAIN: &[u8] = b"store.physical.wal.canonical-redo.v3";
const PROJECTION_DOMAIN: &[u8] = b"store.physical.recovery-projection.v3";

#[test]
fn rehashed_exact_binding_is_admitted_by_the_binding_oracle() {
    let record = record(1, 7);
    let payload = b"payload-a";
    let (idempotency, bytes) = wal(1, record, payload);
    require_bound_record(&wal_file(bytes), &idempotency, record, payload, None).unwrap();
}

#[test]
fn rehashed_wrong_idempotency_does_not_bind_the_operation() {
    let expected_record = record(1, 7);
    let expected_payload = b"payload-a";
    let (expected_idempotency, _) = wal(1, expected_record, expected_payload);
    let (_, observed) = wal(2, expected_record, expected_payload);
    require_bound_record(
        &wal_file(observed),
        &expected_idempotency,
        expected_record,
        expected_payload,
        None,
    )
    .expect_err("a validly rehashed foreign idempotency key must not bind");
}

#[test]
fn rehashed_wrong_physical_record_does_not_bind_the_operation() {
    let expected_record = record(1, 7);
    let expected_payload = b"payload-a";
    let (idempotency, _) = wal(1, expected_record, expected_payload);
    let (_, observed) = wal(1, record(2, 8), expected_payload);
    require_bound_record(
        &wal_file(observed),
        &idempotency,
        expected_record,
        expected_payload,
        None,
    )
    .expect_err("a validly rehashed foreign record must not bind");
}

#[test]
fn rehashed_wrong_payload_does_not_bind_the_operation() {
    let expected_record = record(1, 7);
    let expected_payload = b"payload-a";
    let (idempotency, _) = wal(1, expected_record, expected_payload);
    let (_, observed) = wal(1, expected_record, b"payload-b");
    require_bound_record(
        &wal_file(observed),
        &idempotency,
        expected_record,
        expected_payload,
        None,
    )
    .expect_err("a validly rehashed foreign payload must not bind");
}

#[test]
fn rehashed_cross_operation_swap_is_rejected_by_the_tuple_oracle() {
    let first_record = record(1, 7);
    let second_record = record(2, 8);
    let first_payload = b"payload-a";
    let second_payload = b"payload-b";
    let (first_idempotency, _) = wal(1, first_record, first_payload);
    let (second_idempotency, _) = wal(2, second_record, second_payload);
    let (_, swapped_first) = wal(1, second_record, second_payload);
    let (_, swapped_second) = wal(2, first_record, first_payload);
    let expected = BTreeMap::from([
        (
            first_idempotency,
            ExpectedCanonicalRecord {
                allocation_epoch: first_record.allocation_epoch,
                ordinal: first_record.ordinal,
                payload: first_payload.to_vec(),
                redo_digest: [0; 32],
            },
        ),
        (
            second_idempotency,
            ExpectedCanonicalRecord {
                allocation_epoch: second_record.allocation_epoch,
                ordinal: second_record.ordinal,
                payload: second_payload.to_vec(),
                redo_digest: [0; 32],
            },
        ),
    ]);
    let mut files = wal_file(swapped_first);
    files.extend(wal_file(swapped_second));
    require_bound_records(&files, &expected)
        .expect_err("cross-operation record/payload swaps must not bind");
}

fn record(epoch: u8, ordinal: u64) -> RecordIdentity {
    RecordIdentity {
        allocation_epoch: [epoch; 16],
        ordinal,
    }
}

fn wal(seed: u8, record: RecordIdentity, payload: &[u8]) -> ([u8; 32], Vec<u8>) {
    let store = [seed; 16];
    let policy = [seed.wrapping_add(10); 32];
    let material = [seed.wrapping_add(20); 32];
    let issuance = u64::from(seed);
    let expiry = issuance + 10;
    let mut key = Sha256::new();
    field_digest(&mut key, KEY_DOMAIN);
    key.update(store);
    key.update(policy);
    key.update(issuance.to_le_bytes());
    key.update(expiry.to_le_bytes());
    key.update(material);
    let idempotency: [u8; 32] = key.finalize().into();
    let redo = redo(record, payload);
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
    field(&mut binding, &Sha256::digest(&redo));
    let mut member = Vec::new();
    field(&mut member, &binding);
    field(&mut member, &redo);
    (idempotency, frame(&member))
}

fn redo(record: RecordIdentity, payload: &[u8]) -> Vec<u8> {
    let mut projection = Vec::new();
    field(&mut projection, PROJECTION_DOMAIN);
    projection.extend_from_slice(&1_u64.to_le_bytes());
    field(&mut projection, &[3]);
    projection.extend_from_slice(&1_u64.to_le_bytes());
    record_field(&mut projection, record);
    projection.extend_from_slice(&1_u64.to_le_bytes());
    field(&mut projection, &[4]);
    projection.extend_from_slice(&1_u64.to_le_bytes());
    let mut placement = vec![2];
    raw_record(&mut placement, record);
    placement.extend_from_slice(&1_u64.to_le_bytes());
    placement.extend_from_slice(&1_u64.to_le_bytes());
    placement.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    field(&mut projection, &placement);
    projection.extend_from_slice(&0_u64.to_le_bytes());
    projection.extend_from_slice(&0_u64.to_le_bytes());
    let mut redo = Vec::new();
    field(&mut redo, REDO_DOMAIN);
    redo.extend_from_slice(&1_u64.to_le_bytes());
    redo.extend_from_slice(&0_u32.to_le_bytes());
    redo.extend_from_slice(&1_u64.to_le_bytes());
    redo.extend_from_slice(&1_u64.to_le_bytes());
    field(&mut redo, &[1]);
    redo.extend_from_slice(&[2; 32]);
    field(&mut redo, payload);
    field(&mut redo, &projection);
    redo
}

fn frame(member: &[u8]) -> Vec<u8> {
    let mut header = vec![0; 116];
    header[..8].copy_from_slice(b"WORTHWAL");
    header[8..10].copy_from_slice(&1_u16.to_le_bytes());
    header[10..12].copy_from_slice(&116_u16.to_le_bytes());
    header[12..20].copy_from_slice(&1_u64.to_le_bytes());
    header[20..28].copy_from_slice(&1_u64.to_le_bytes());
    header[28..36].copy_from_slice(&10_u64.to_le_bytes());
    header[36..44].copy_from_slice(&11_u64.to_le_bytes());
    header[44..52].copy_from_slice(&(member.len() as u64).to_le_bytes());
    header[84..116].copy_from_slice(&Sha256::digest(member));
    let mut frame = header;
    frame.extend_from_slice(member);
    frame.extend_from_slice(&Sha256::digest(&frame));
    frame
}

fn wal_file(bytes: Vec<u8>) -> Vec<(String, Vec<u8>)> {
    vec![
        ("families/checkpoint.current".to_owned(), checkpoint()),
        ("families/wal/segment-1-generation-1.wal".to_owned(), bytes),
    ]
}

fn checkpoint() -> Vec<u8> {
    let mut header = vec![0; 144];
    header[..16].copy_from_slice(&[1; 16]);
    header[16..24].copy_from_slice(&1_u64.to_le_bytes());
    header[24..32].copy_from_slice(&1_u64.to_le_bytes());
    header[32..40].copy_from_slice(&10_u64.to_le_bytes());
    header[64] = 1;
    let mut compaction = [0; 16];
    compaction[8] = 1;
    let mut footer = vec![0; 136];
    footer[..24].copy_from_slice(&header[..24]);
    footer[32..64].copy_from_slice(&Sha256::digest([]));
    footer[64..72].copy_from_slice(&(record_bytes(144) as u64).to_le_bytes());
    footer[72..80].copy_from_slice(&0_u64.to_le_bytes());
    footer[80..88].copy_from_slice(&1_u64.to_le_bytes());
    footer[104..136].copy_from_slice(&Sha256::digest([]));
    let mut bytes = checkpoint_record(1, &header);
    bytes.extend_from_slice(&checkpoint_record(3, &compaction));
    bytes.extend_from_slice(&checkpoint_record(5, &footer));
    bytes
}

fn checkpoint_record(kind: u8, payload: &[u8]) -> Vec<u8> {
    let mut record = vec![0; 16];
    record[..8].copy_from_slice(b"WCP7REC\0");
    record[8] = 1;
    record[9] = kind;
    record[12..16].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    record.extend_from_slice(payload);
    record.extend_from_slice(&crc32c(&record).to_le_bytes());
    record
}

const fn record_bytes(payload_bytes: usize) -> usize {
    16 + payload_bytes + 4
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

fn field(target: &mut Vec<u8>, bytes: &[u8]) {
    target.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    target.extend_from_slice(bytes);
}

fn field_digest(digest: &mut Sha256, bytes: &[u8]) {
    digest.update((bytes.len() as u64).to_le_bytes());
    digest.update(bytes);
}

fn record_field(target: &mut Vec<u8>, record: RecordIdentity) {
    let mut bytes = Vec::new();
    raw_record(&mut bytes, record);
    field(target, &bytes);
}

fn raw_record(target: &mut Vec<u8>, record: RecordIdentity) {
    target.extend_from_slice(&record.allocation_epoch);
    target.extend_from_slice(&record.ordinal.to_le_bytes());
}

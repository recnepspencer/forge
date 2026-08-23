use sha2::{Digest, Sha256};

pub fn mutate_artifact_identity_digest(encoded: &[u8]) -> Vec<u8> {
    let mut mutated = encoded.to_vec();
    let payload_start = payload_start(&mutated);
    let artifact_identity_digest_offset = payload_start + 8 + 8 + 32 + 8;
    mutated[artifact_identity_digest_offset] ^= 1;
    refresh_digest(mutated)
}

pub fn mutate_runtime_root_generation(encoded: &[u8]) -> Vec<u8> {
    let mut mutated = encoded.to_vec();
    let root_generation_flag = runtime_root_generation_offset(&mutated);
    if mutated[root_generation_flag] == 1 {
        mutated[root_generation_flag + 1] ^= 1;
    } else {
        mutated[root_generation_flag] = 1;
        mutated.splice(
            root_generation_flag + 1..root_generation_flag + 1,
            u64::MAX.to_le_bytes(),
        );
    }
    refresh_digest(mutated)
}

pub fn mutate_runtime_publication_as_blocked(encoded: &[u8]) -> Vec<u8> {
    let mut mutated = encoded.to_vec();
    let outcome_offset = payload_start(&mutated);
    mutated[outcome_offset] = 3;
    let cause_offset = mutated.len() - 33;
    mutated[cause_offset] = 16;
    refresh_digest(mutated)
}

pub fn mutate_runtime_blocked_as_indeterminate(encoded: &[u8]) -> Vec<u8> {
    let mut mutated = encoded.to_vec();
    let root_generation_flag = runtime_root_generation_offset(&mutated);
    if mutated[root_generation_flag] == 1 {
        mutated.drain(root_generation_flag + 1..root_generation_flag + 9);
    }
    mutated[root_generation_flag] = 0;
    let effects_offset = root_generation_flag + 1;
    mutated[effects_offset..effects_offset + 8].copy_from_slice(&1_u64.to_le_bytes());
    mutated[effects_offset + 8..effects_offset + 16].copy_from_slice(&0_u64.to_le_bytes());
    mutated[effects_offset + 16..effects_offset + 24].copy_from_slice(&0_u64.to_le_bytes());
    mutated[effects_offset + 24..effects_offset + 32].copy_from_slice(&0_u64.to_le_bytes());
    let outcome_offset = payload_start(&mutated);
    mutated[outcome_offset] = 4;
    let cause_offset = mutated.len() - 33;
    mutated[cause_offset] = 32;
    refresh_digest(mutated)
}

pub fn mutate_runtime_peak_recovery_bytes(encoded: &[u8], value: u64) -> Vec<u8> {
    let mut mutated = encoded.to_vec();
    let payload_start = payload_start(&mutated);
    let peak_offset = payload_start + 1 + 1 + 16 + 1 + 8 + 8 + 8 + 8;
    mutated[peak_offset..peak_offset + 8].copy_from_slice(&value.to_le_bytes());
    refresh_digest(mutated)
}

pub fn mutate_runtime_recovery_effects(encoded: &[u8], value: u64) -> Vec<u8> {
    let mut mutated = encoded.to_vec();
    let root_generation_flag = runtime_root_generation_offset(&mutated);
    let effects_offset = root_generation_flag
        + 1
        + if mutated[root_generation_flag] == 1 {
            8
        } else {
            0
        };
    mutated[effects_offset..effects_offset + 8].copy_from_slice(&value.to_le_bytes());
    refresh_digest(mutated)
}

pub fn mutate_runtime_denial_cause(encoded: &[u8]) -> Vec<u8> {
    let mut mutated = encoded.to_vec();
    let cause_offset = mutated.len() - 33;
    let cause = mutated[cause_offset];
    mutated[cause_offset] = if (16..=27).contains(&cause) {
        16 + (cause - 16 + 1) % 12
    } else {
        16
    };
    refresh_digest(mutated)
}

pub fn mutate_observer_evidence_fields(encoded: &[u8]) -> Vec<(&'static str, Vec<u8>)> {
    observer_u64_offsets(encoded)
        .into_iter()
        .map(|(name, offset)| {
            let mut mutated = encoded.to_vec();
            let value = u64::from_le_bytes(
                mutated[offset..offset + 8]
                    .try_into()
                    .expect("observer mutation field is a u64"),
            );
            mutated[offset..offset + 8].copy_from_slice(&value.wrapping_add(1).to_le_bytes());
            (name, refresh_digest(mutated))
        })
        .collect()
}

fn runtime_root_generation_offset(encoded: &[u8]) -> usize {
    let mut cursor = payload_start(encoded) + 1;
    let store_present = *encoded
        .get(cursor)
        .expect("runtime store identity option flag");
    cursor += 1 + if store_present == 1 { 16 } else { 0 };
    cursor
}

fn observer_u64_offsets(encoded: &[u8]) -> Vec<(&'static str, usize)> {
    let mut cursor = payload_start(encoded);
    let mut offsets = Vec::new();
    take_u64(&mut cursor, &mut offsets, "artifact_count");
    take_u64(&mut cursor, &mut offsets, "bytes_read");
    skip_bytes(&mut cursor, 32);
    take_u64(&mut cursor, &mut offsets, "artifact_identity_count");
    skip_bytes(&mut cursor, 32);
    take_u64(&mut cursor, &mut offsets, "generation_link_count");
    skip_bytes(&mut cursor, 32);

    take_u64(&mut cursor, &mut offsets, "selector_count");
    take_u64(&mut cursor, &mut offsets, "linked_selector_count");
    take_u64(&mut cursor, &mut offsets, "unpaired_selector_link_count");
    skip_optional_array(encoded, &mut cursor, 16);
    skip_optional_u64(encoded, &mut cursor);
    skip_bytes(&mut cursor, 32);

    take_u64(&mut cursor, &mut offsets, "checkpoint_count");
    take_u64(&mut cursor, &mut offsets, "checkpoint_page_count");
    for _ in 0..4 {
        skip_optional_u64(encoded, &mut cursor);
    }
    skip_bytes(&mut cursor, 32);

    take_u64(&mut cursor, &mut offsets, "wal_segment_count");
    take_u64(&mut cursor, &mut offsets, "valid_wal_prefix_bytes");
    take_u64(&mut cursor, &mut offsets, "observed_wal_bytes");
    take_u64(&mut cursor, &mut offsets, "wal_frame_count");
    for _ in 0..2 {
        skip_optional_u64(encoded, &mut cursor);
    }
    skip_bytes(&mut cursor, 32);

    take_u64(&mut cursor, &mut offsets, "page_lsn_count");
    for _ in 0..2 {
        skip_optional_u64(encoded, &mut cursor);
    }
    skip_bytes(&mut cursor, 32);

    take_u64(&mut cursor, &mut offsets, "manifest_count");
    take_u64(&mut cursor, &mut offsets, "manifest_member_count");
    skip_bytes(&mut cursor, 32);

    take_u64(&mut cursor, &mut offsets, "residue_artifact_count");
    take_u64(&mut cursor, &mut offsets, "residue_bytes");
    skip_bytes(&mut cursor, 32);
    assert_eq!(
        cursor,
        encoded.len() - 32,
        "observer mutation cursor drifted"
    );
    offsets
}

fn take_u64(cursor: &mut usize, offsets: &mut Vec<(&'static str, usize)>, name: &'static str) {
    offsets.push((name, *cursor));
    *cursor += 8;
}

fn skip_optional_array(encoded: &[u8], cursor: &mut usize, length: usize) {
    let present = cursor_value(encoded, cursor);
    *cursor += 1 + if present == 1 { length } else { 0 };
}

fn skip_optional_u64(encoded: &[u8], cursor: &mut usize) {
    let present = cursor_value(encoded, cursor);
    *cursor += 1 + if present == 1 { 8 } else { 0 };
}

fn cursor_value(encoded: &[u8], cursor: &usize) -> u8 {
    *encoded.get(*cursor).expect("observer optional field flag")
}

fn skip_bytes(cursor: &mut usize, length: usize) {
    *cursor += length;
}

fn payload_start(encoded: &[u8]) -> usize {
    let family_bytes = u64::from_le_bytes(
        encoded[..8]
            .try_into()
            .expect("report family length is encoded as u64"),
    ) as usize;
    8 + family_bytes + 4
}

fn refresh_digest(mut encoded: Vec<u8>) -> Vec<u8> {
    let payload_end = encoded.len() - 32;
    let digest: [u8; 32] = Sha256::digest(&encoded[..payload_end]).into();
    encoded[payload_end..].copy_from_slice(&digest);
    encoded
}

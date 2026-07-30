use std::sync::OnceLock;

use sha2::{Digest, Sha256};

use super::{
    EXTENT_RECORDS, EXTENT_RECORD_BYTES, INLINE_RECORDS, INLINE_RECORD_BYTES, WORKLOAD_SEED,
};

pub(super) fn digest() -> [u8; 32] {
    static EXPECTATION: OnceLock<[u8; 32]> = OnceLock::new();
    *EXPECTATION.get_or_init(compute_digest)
}

fn compute_digest() -> [u8; 32] {
    let mut digest = Sha256::new();
    for ordinal in 0..INLINE_RECORDS + EXTENT_RECORDS {
        let bytes = if ordinal < INLINE_RECORDS {
            INLINE_RECORD_BYTES
        } else {
            EXTENT_RECORD_BYTES
        };
        digest.update((bytes as u64).to_le_bytes());
        update_record(&mut digest, ordinal, bytes);
    }
    digest.finalize().into()
}

fn update_record(digest: &mut Sha256, ordinal: usize, bytes: usize) {
    let mut offset = 0;
    let mut chunk = [0_u8; 8 * 1024];
    while offset < bytes {
        let width = (bytes - offset).min(chunk.len());
        for (index, byte) in chunk[..width].iter_mut().enumerate() {
            *byte = expected_byte(WORKLOAD_SEED, ordinal as u64, offset + index);
        }
        digest.update(&chunk[..width]);
        offset += width;
    }
}

fn payload_byte(seed: u64, ordinal: u64, offset: u64) -> u8 {
    let mixed = seed
        ^ ordinal.wrapping_add(1).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ offset.wrapping_add(1).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    avalanche(mixed) as u8
}

fn expected_byte(seed: u64, ordinal: u64, offset: usize) -> u8 {
    if offset < 8 {
        ordinal.to_le_bytes()[offset]
    } else {
        payload_byte(seed, ordinal, offset as u64)
    }
}

const fn avalanche(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

#[cfg(test)]
mod tests {
    use super::digest;

    #[test]
    fn expectation_digest_is_nonzero_and_repeatable() {
        assert_ne!(digest(), [0; 32]);
        assert_eq!(digest(), digest());
    }
}

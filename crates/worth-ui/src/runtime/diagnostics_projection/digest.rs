use std::fmt::Debug;

pub(crate) fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}

pub(crate) fn digest_debug(value: &impl Debug) -> u64 {
    stable_text_digest(&format!("{value:?}"))
}

pub(crate) fn combine_digest(seed: u64, value: u64) -> u64 {
    seed.wrapping_mul(0x0000_0100_0000_01B3) ^ value
}

pub(super) fn fold_text(mut digest: u64, text: &str) -> u64 {
    for byte in text.bytes() {
        digest = fold(digest, u64::from(byte));
    }
    digest
}

pub(super) fn fold(digest: u64, value: u64) -> u64 {
    (digest ^ value).wrapping_mul(0x100000001b3)
}

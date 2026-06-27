pub(crate) fn fold_texts(parts: impl IntoIterator<Item = impl AsRef<str>>) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325u64;
    for part in parts {
        fold_text(&mut digest, part.as_ref());
    }
    digest
}

pub(crate) fn fold_text(digest: &mut u64, text: &str) {
    for byte in text.as_bytes() {
        *digest ^= u64::from(*byte);
        *digest = digest.wrapping_mul(0x100_0000_01b3);
    }
}

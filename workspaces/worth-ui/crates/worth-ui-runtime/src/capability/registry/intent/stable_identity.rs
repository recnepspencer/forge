pub(super) const fn assert_valid_stable_identity(stable_text: &str) {
    let bytes = stable_text.as_bytes();
    assert!(!bytes.is_empty(), "stable identity must not be empty");
    let mut index = 0;
    let mut at_segment_start = true;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'.' {
            assert!(
                !at_segment_start,
                "stable identity contains an empty segment"
            );
            at_segment_start = true;
        } else if at_segment_start {
            assert!(
                byte >= b'a' && byte <= b'z',
                "stable identity segments must begin with a lowercase ASCII letter"
            );
            at_segment_start = false;
        } else {
            assert!(
                (byte >= b'a' && byte <= b'z') || (byte >= b'0' && byte <= b'9') || byte == b'_',
                "stable identity contains an invalid ASCII byte"
            );
        }
        index += 1;
    }
    assert!(
        !at_segment_start,
        "stable identity must not end with a separator"
    );
}

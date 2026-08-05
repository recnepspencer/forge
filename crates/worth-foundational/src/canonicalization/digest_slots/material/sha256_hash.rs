use sha2::{Digest, Sha256};

pub(crate) fn sha256_digest(material: &[u8]) -> [u8; 32] {
    Sha256::digest(material).into()
}

#[cfg(test)]
mod tests {
    use super::sha256_digest;

    #[test]
    fn sha256_matches_the_standard_abc_vector() {
        assert_eq!(
            sha256_digest(b"abc"),
            [
                186, 120, 22, 191, 143, 1, 207, 234, 65, 65, 64, 222, 93, 174, 34, 35, 176, 3, 97,
                163, 150, 23, 122, 156, 180, 16, 255, 97, 242, 0, 21, 173,
            ]
        );
    }
}

const CANONICAL_SOURCE: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/app/main.wui"));

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct CanonicalPlatformPulse {
    _private: (),
}

impl CanonicalPlatformPulse {
    pub(crate) fn checked_in() -> Self {
        Self { _private: () }
    }

    pub(crate) fn source_bytes(self) -> &'static [u8] {
        CANONICAL_SOURCE
    }
}

#[cfg(test)]
mod tests {
    use super::CanonicalPlatformPulse;

    #[test]
    fn canonical_world_is_the_exact_checked_in_source() {
        assert_eq!(
            CanonicalPlatformPulse::checked_in().source_bytes(),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/app/main.wui"))
        );
    }
}

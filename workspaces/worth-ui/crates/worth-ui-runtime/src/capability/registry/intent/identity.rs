use core::fmt;

use super::stable_identity::assert_valid_stable_identity;

/// Stable compiled identity for one product intent definition.
///
/// Definitions are Rust-authored capability meaning, so the identity is
/// intentionally static. File-authored declarations may reference its text
/// but cannot mint a definition.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiIntentId {
    stable_text: &'static str,
}

impl UiIntentId {
    pub const fn stable(stable_text: &'static str) -> Self {
        assert_valid_stable_identity(stable_text);
        Self { stable_text }
    }

    pub const fn as_str(self) -> &'static str {
        self.stable_text
    }
}

impl fmt::Debug for UiIntentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("UiIntentId")
            .field(&self.stable_text)
            .finish()
    }
}

impl fmt::Display for UiIntentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.stable_text)
    }
}

#[cfg(test)]
mod tests {
    use super::UiIntentId;

    const INTENT_ID: UiIntentId = UiIntentId::stable("platform.pulse.advance");

    #[test]
    fn stable_identity_is_available_in_const_context() {
        assert_eq!(INTENT_ID.as_str(), "platform.pulse.advance");
    }
}

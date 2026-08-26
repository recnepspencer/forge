/// Stable semantic identity for a Rust type axis retained by a portable package.
///
/// The value is declaration metadata, not runtime authority. Declarations must
/// still pass schema validation before the identity can enter package meaning.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryPortableTypeIdentity(&'static str);

impl WorthQueryPortableTypeIdentity {
    #[doc(hidden)]
    pub const fn declared(value: &'static str) -> Self {
        Self(value)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }

    pub fn is_valid(self) -> bool {
        !self.0.is_empty()
            && self.0.trim() == self.0
            && !self
                .0
                .chars()
                .any(|character| character.is_whitespace() || character.is_control())
    }
}

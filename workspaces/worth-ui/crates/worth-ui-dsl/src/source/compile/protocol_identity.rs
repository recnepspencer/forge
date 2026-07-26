/// Exact schema/protocol identity of a sealed authored-to-runtime package.
///
/// Construction remains DSL-owned. Runtime consumers compare the complete
/// value before attempting capability or generation admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiDslProtocolIdentity {
    schema: &'static str,
    major: u16,
    minor: u16,
}

impl WorthUiDslProtocolIdentity {
    const CURRENT: Self = Self {
        schema: "worth-ui.dsl.semantic-package",
        major: 1,
        minor: 0,
    };

    pub const fn current() -> Self {
        Self::CURRENT
    }

    pub const fn schema(self) -> &'static str {
        self.schema
    }

    pub const fn major(self) -> u16 {
        self.major
    }

    pub const fn minor(self) -> u16 {
        self.minor
    }

    pub fn is_current(self) -> bool {
        self.schema == Self::CURRENT.schema
            && self.major == Self::CURRENT.major
            && self.minor == Self::CURRENT.minor
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) const fn unsupported_for_test() -> Self {
        Self {
            schema: "worth-ui.dsl.semantic-package",
            major: 2,
            minor: 0,
        }
    }
}

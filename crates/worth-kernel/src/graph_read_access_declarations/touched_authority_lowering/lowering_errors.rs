#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadTouchedAuthorityLoweringErrorKind {
    UnsupportedTouchedAuthorityScope,
    MissingTouchedAuthorityDigest,
    TouchedAuthorityDigestMismatch,
    UnsupportedOperatingWorldScope,
    QueryTouchDescriptorDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadTouchedAuthorityLoweringError {
    kind: WorthGraphReadTouchedAuthorityLoweringErrorKind,
}

impl WorthGraphReadTouchedAuthorityLoweringError {
    pub(crate) const fn new(kind: WorthGraphReadTouchedAuthorityLoweringErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthGraphReadTouchedAuthorityLoweringErrorKind {
        self.kind
    }
}

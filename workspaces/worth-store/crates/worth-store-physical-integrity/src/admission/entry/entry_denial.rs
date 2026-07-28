#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityEntryDenial {
    kind: IntegrityEntryDenialKind,
}

impl IntegrityEntryDenial {
    pub const fn new(kind: IntegrityEntryDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(self) -> IntegrityEntryDenialKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityEntryDenialKind {
    MissingProtectedPhysicalByteView,
    VerificationStoreMismatch,
    VerificationGenerationMismatch,
}

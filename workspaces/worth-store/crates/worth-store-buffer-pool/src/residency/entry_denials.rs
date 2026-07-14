#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferPoolEntryDenial {
    kind: BufferPoolEntryDenialKind,
}

impl BufferPoolEntryDenial {
    pub(crate) const fn new(kind: BufferPoolEntryDenialKind) -> Self {
        Self { kind }
    }

    pub const fn forbidden_shortcut(kind: BufferPoolEntryDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> BufferPoolEntryDenialKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferPoolEntryDenialKind {
    MissingReadiness,
    MissingBudget,
    UnsealedReadiness,
    RawPageIdRejected,
    RawPayloadViewRejected,
    CompatibilityBackendHandleRejected,
    FoundationalEvidenceAsAuthorityRejected,
}

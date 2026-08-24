#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthQueryApplicationSchemaContractCatalogDenialKind {
    DuplicateAspectIdentity,
    DuplicateAspectLocus,
    DuplicateFieldLocus,
    RevisionZero,
    MissingAspectFieldClosure,
    FieldWithoutAspect,
    InvalidAspectKey,
    InvalidFieldKey,
    InvalidAspectShape,
    ProjectionMaskRejected,
    CanonicalContractRejected,
    CanonicalEntryBudgetExceeded,
    CanonicalEncodedByteBudgetExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryApplicationSchemaContractCatalogDenial {
    kind: WorthQueryApplicationSchemaContractCatalogDenialKind,
    subject: String,
}

impl WorthQueryApplicationSchemaContractCatalogDenial {
    pub(crate) fn new(
        kind: WorthQueryApplicationSchemaContractCatalogDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub(crate) const fn kind(&self) -> WorthQueryApplicationSchemaContractCatalogDenialKind {
        self.kind
    }

    pub(crate) fn subject(&self) -> &str {
        &self.subject
    }
}

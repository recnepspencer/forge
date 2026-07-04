#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SpatialCompiledProductAdmissionErrorKind {
    BroadEvidenceScanDenied,
    FamilyCatalogDenied,
    WrongAuthorityBasis,
    WrongReceiptFamily,
    WrongSupportPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SpatialCompiledProductAdmissionError {
    kind: SpatialCompiledProductAdmissionErrorKind,
    detail: String,
}

impl SpatialCompiledProductAdmissionError {
    pub(crate) fn new(
        kind: SpatialCompiledProductAdmissionErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub(crate) const fn kind(&self) -> SpatialCompiledProductAdmissionErrorKind {
        self.kind
    }

    pub(crate) fn detail(&self) -> &str {
        &self.detail
    }
}

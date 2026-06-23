#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerProductAdapterCertificationCode {
    BlankAdapterLabel,
    MissingDeclarations,
    BlankOperationName,
    BlankPayloadSchemaIdentity,
    BlankSupportSnapshotRow,
    MissingErrorMap,
    BlankDraftScope,
    BlankCoordinationLane,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductAdapterCertificationError {
    code: ForgeServerProductAdapterCertificationCode,
    detail: String,
}

impl ForgeServerProductAdapterCertificationError {
    pub(crate) fn new(
        code: ForgeServerProductAdapterCertificationCode,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> ForgeServerProductAdapterCertificationCode {
        self.code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerProductResultContractErrorCode {
    BlankSchemaIdentity,
    ZeroSchemaVersion,
    ZeroInlineBudget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductResultContractError {
    code: WorthServerProductResultContractErrorCode,
    detail: String,
}

impl WorthServerProductResultContractError {
    pub(crate) fn blank_schema_identity() -> Self {
        Self::new(
            WorthServerProductResultContractErrorCode::BlankSchemaIdentity,
            "product result schema identity must be non-blank",
        )
    }

    pub(crate) fn zero_schema_version() -> Self {
        Self::new(
            WorthServerProductResultContractErrorCode::ZeroSchemaVersion,
            "product result schema version must be nonzero",
        )
    }

    pub(crate) fn zero_inline_budget() -> Self {
        Self::new(
            WorthServerProductResultContractErrorCode::ZeroInlineBudget,
            "product result inline byte budget must be nonzero",
        )
    }

    fn new(code: WorthServerProductResultContractErrorCode, detail: impl Into<String>) -> Self {
        Self {
            code,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> WorthServerProductResultContractErrorCode {
        self.code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

pub(crate) fn artifact_matches_contract(
    artifact: &super::WorthServerProductResultArtifact,
    contract: &super::WorthServerProductResultContract,
) -> bool {
    artifact.contract() == contract
}

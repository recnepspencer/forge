use super::super::selection_substrate::QueryObligationSelectionError;
use super::kinds::QueryGraphObligationSelectionFacadeErrorKind;

#[derive(Debug)]
pub enum QueryGraphObligationSelectionFacadeError {
    Selection(QueryObligationSelectionError),
    WorkloadAuthorityMismatch(String),
}

impl QueryGraphObligationSelectionFacadeError {
    pub(crate) fn workload_authority_mismatch(detail: impl Into<String>) -> Self {
        Self::WorkloadAuthorityMismatch(detail.into())
    }

    pub fn kind(&self) -> QueryGraphObligationSelectionFacadeErrorKind {
        match self {
            Self::Selection(error) => error.kind().into(),
            Self::WorkloadAuthorityMismatch(_) => {
                QueryGraphObligationSelectionFacadeErrorKind::WorkloadAuthorityMismatch
            }
        }
    }

    pub fn detail(&self) -> &str {
        match self {
            Self::Selection(error) => error.detail(),
            Self::WorkloadAuthorityMismatch(detail) => detail,
        }
    }
}

impl From<QueryObligationSelectionError> for QueryGraphObligationSelectionFacadeError {
    fn from(error: QueryObligationSelectionError) -> Self {
        Self::Selection(error)
    }
}

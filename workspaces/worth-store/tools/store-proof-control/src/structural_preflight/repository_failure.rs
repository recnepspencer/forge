use worth_store_test_support::structural_preflight::StructuralPredicate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RepositoryPredicateFailure {
    pub predicate: StructuralPredicate,
    pub failure_code: &'static str,
    pub message: String,
    pub invalidated_inputs: Vec<String>,
}

impl RepositoryPredicateFailure {
    pub(crate) fn new(
        predicate: StructuralPredicate,
        failure_code: &'static str,
        message: impl Into<String>,
        invalidated_inputs: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            predicate,
            failure_code,
            message: message.into(),
            invalidated_inputs: invalidated_inputs.into_iter().map(Into::into).collect(),
        }
    }
}

impl std::fmt::Display for RepositoryPredicateFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{:?}/{}: {}",
            self.predicate, self.failure_code, self.message
        )
    }
}

impl std::error::Error for RepositoryPredicateFailure {}

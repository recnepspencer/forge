use super::{QueryValidationCounters, QueryValidationError, ValidationRejectionMatrix};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationFailureArtifact {
    pub error: QueryValidationError,
    pub counters: QueryValidationCounters,
    pub rejection_matrix: ValidationRejectionMatrix,
}

impl ValidationFailureArtifact {
    pub fn new(
        error: QueryValidationError,
        counters: QueryValidationCounters,
        rejection_matrix: ValidationRejectionMatrix,
    ) -> Self {
        Self {
            error,
            counters,
            rejection_matrix,
        }
    }
}

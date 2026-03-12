use crate::publication::data::{PublicationError, PublicationStage};
use crate::transactions::data::{CommitConflict, ConflictClass};
use crate::validation::data::{
    InvariantCheckResult, InvariantFailureEffect, InvariantVerdict, InvariantViolation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantFailure {
    effect: InvariantFailureEffect,
    violation: InvariantViolation,
}

impl InvariantFailure {
    pub fn effect(&self) -> InvariantFailureEffect {
        self.effect
    }

    pub fn violation(&self) -> &InvariantViolation {
        &self.violation
    }

    pub fn into_commit_conflict(self) -> CommitConflict {
        CommitConflict::new(ConflictClass::InvariantViolation {
            code: self.violation.code,
            detail: self.violation.detail,
        })
    }

    pub fn into_publication_error(self, stage: PublicationStage) -> PublicationError {
        PublicationError::new(stage, self.violation.detail)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantExecutionResult {
    results: Vec<InvariantCheckResult>,
}

impl InvariantExecutionResult {
    pub fn new(results: Vec<InvariantCheckResult>) -> Self {
        Self { results }
    }

    pub fn results(&self) -> &[InvariantCheckResult] {
        &self.results
    }

    pub fn into_results(self) -> Vec<InvariantCheckResult> {
        self.results
    }

    pub fn first_blocking_failure(&self) -> Option<InvariantFailure> {
        self.first_failure_with_effect(InvariantFailureEffect::BlockCommit)
    }

    pub fn first_publication_failure(&self) -> Option<InvariantFailure> {
        self.first_failure_with_effect(InvariantFailureEffect::BlockPublication)
    }

    pub fn blocking_commit_conflict(&self) -> Option<CommitConflict> {
        self.first_blocking_failure()
            .map(|failure| failure.into_commit_conflict())
    }

    pub fn publication_error(&self, stage: PublicationStage) -> Option<PublicationError> {
        self.first_publication_failure()
            .map(|failure| failure.into_publication_error(stage))
    }

    fn first_failure_with_effect(
        &self,
        effect: InvariantFailureEffect,
    ) -> Option<InvariantFailure> {
        self.results
            .iter()
            .find(|result| {
                result.failure_effect == effect
                    && result.verdict == InvariantVerdict::Fail
                    && !result.violations.is_empty()
            })
            .and_then(|result| result.violations.first())
            .cloned()
            .map(|violation| InvariantFailure { effect, violation })
    }
}

#[cfg(test)]
mod tests {
    use super::InvariantFailure;
    use crate::diagnostics::data::DiagnosticCode;
    use crate::publication::data::PublicationStage;
    use crate::validation::data::{
        InvariantClass, InvariantFailureEffect, InvariantViolation,
    };

    #[test]
    fn invariant_failure_converts_to_commit_and_publication_errors() {
        let failure = InvariantFailure {
            effect: InvariantFailureEffect::BlockPublication,
            violation: InvariantViolation {
                class: InvariantClass::SnapshotAudit,
                code: DiagnosticCode::InvariantViolation,
                detail: "detail".to_string(),
            },
        };

        let conflict = failure.clone().into_commit_conflict();
        assert_eq!(conflict.code(), DiagnosticCode::InvariantViolation);
        assert_eq!(conflict.detail(), "detail".to_string());

        let publication = failure.into_publication_error(PublicationStage::InvariantCheck);
        assert_eq!(publication.stage, PublicationStage::InvariantCheck);
        assert_eq!(publication.detail, "detail".to_string());
    }
}

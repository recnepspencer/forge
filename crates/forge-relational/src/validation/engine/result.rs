use crate::publication::data::{PublicationError, PublicationStage};
use crate::transactions::data::{CommitConflict, ConflictClass};
use crate::validation::data::{
    InvariantCheckResult, InvariantExecutionPoint, InvariantFailureEffect, InvariantVerdict,
    InvariantViolation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantFailure {
    execution_point: InvariantExecutionPoint,
    effect: InvariantFailureEffect,
    violation: InvariantViolation,
}

impl InvariantFailure {
    pub fn execution_point(&self) -> InvariantExecutionPoint {
        self.execution_point
    }

    pub fn effect(&self) -> InvariantFailureEffect {
        self.effect
    }

    pub fn violation(&self) -> &InvariantViolation {
        &self.violation
    }

    pub fn detail(&self) -> &str {
        &self.violation.detail
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

    fn first_failure_with_effect(
        &self,
        effect: InvariantFailureEffect,
    ) -> Option<InvariantFailure> {
        self.results
            .iter()
            .find_map(|result| {
                if result.failure_effect != effect {
                    return None;
                }
                match &result.verdict {
                    InvariantVerdict::Violation(violation) => {
                        Some((result.execution_point, violation.clone()))
                    }
                    InvariantVerdict::Pass | InvariantVerdict::Advisory { .. } => None,
                }
            })
            .map(|(execution_point, violation)| InvariantFailure {
                execution_point,
                effect,
                violation,
            })
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
            execution_point: crate::validation::data::InvariantExecutionPoint::SnapshotPublication,
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

use crate::identity::CanonicalEquivalence;

use super::super::{
    QueryValidationCounters, QueryValidationError, QueryValidationReport, ValidationEvent,
};
use super::{ValidatedQueryArtifact, ValidatedResultShapeArtifact};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedQueryBundle {
    query: ValidatedQueryArtifact,
    result_shape: ValidatedResultShapeArtifact,
    report: QueryValidationReport,
    counters: QueryValidationCounters,
}

impl ValidatedQueryBundle {
    pub fn query(&self) -> &ValidatedQueryArtifact {
        &self.query
    }

    pub fn result_shape(&self) -> &ValidatedResultShapeArtifact {
        &self.result_shape
    }

    pub fn report(&self) -> &QueryValidationReport {
        &self.report
    }

    pub fn counters(&self) -> &QueryValidationCounters {
        &self.counters
    }

    pub fn equivalence_to(&self, other: &Self) -> CanonicalEquivalence {
        if self.query.digest() == other.query.digest()
            && self.result_shape.digest() == other.result_shape.digest()
        {
            CanonicalEquivalence::Equivalent
        } else {
            CanonicalEquivalence::Distinct
        }
    }

    pub fn check_invariants(&self) -> Result<(), QueryValidationError> {
        if self.query.schema_basis() != self.result_shape.schema_basis() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated query and result-shape schema bases diverged",
            });
        }

        if self.report.schema_basis_digest() != self.query.schema_basis().as_str() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message:
                    "validation report schema basis does not match validated artifact schema basis",
            });
        }

        if self.report.validated_projection_entries() != self.query.projection().len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated projection count does not match validated projection length",
            });
        }

        if self.report.validated_traversal_entries() != self.query.traversal().len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated traversal count does not match validated traversal length",
            });
        }

        if self.report.validated_result_shape_bindings() != self.result_shape.bindings().len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated result-shape binding count does not match validated result-shape binding length",
            });
        }

        if self.report.validated_predicates() != self.query.predicates().entries().len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated predicate count does not match validated predicate length",
            });
        }

        if self.report.validated_ordering_fields() != self.query.ordering().entries().len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated ordering count does not match validated ordering length",
            });
        }

        if self.counters.validated_projection_entry_count() != self.query.projection().len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated projection counter does not match validated projection length",
            });
        }

        if self.counters.validated_traversal_clause_count() != self.query.traversal().len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated traversal counter does not match validated traversal length",
            });
        }

        if self.counters.validated_result_shape_binding_count()
            != self.result_shape.bindings().len()
        {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message:
                    "validated result-shape binding counter does not match validated binding length",
            });
        }

        if self.counters.validated_predicate_count() != self.query.predicates().entries().len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated predicate counter does not match validated predicate length",
            });
        }

        if self.counters.validated_ordering_field_count() != self.query.ordering().entries().len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated ordering counter does not match validated ordering length",
            });
        }

        if self.counters.validation_warning_count() != self.report.warnings().len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validation warning counter does not match warning list length",
            });
        }

        let compatibility_events = self
            .report
            .events()
            .iter()
            .filter(|event| matches!(event, ValidationEvent::CompatibilityEstablished))
            .count();
        if compatibility_events != 1 {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validation compatibility must be established exactly once",
            });
        }

        let identity_frozen_events = self
            .report
            .events()
            .iter()
            .filter(|event| matches!(event, ValidationEvent::IdentityFrozen { .. }))
            .count();
        if identity_frozen_events != 1 {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated identity must be frozen exactly once",
            });
        }

        Ok(())
    }

    pub(crate) fn new(
        query: ValidatedQueryArtifact,
        result_shape: ValidatedResultShapeArtifact,
        report: QueryValidationReport,
        counters: QueryValidationCounters,
    ) -> Self {
        Self {
            query,
            result_shape,
            report,
            counters,
        }
    }

    #[cfg(test)]
    pub(crate) fn report_mut_for_test(&mut self) -> &mut QueryValidationReport {
        &mut self.report
    }
}

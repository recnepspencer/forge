use crate::diagnostics::{
    CanonicalizationCounters, CanonicalizationReport, CanonicalizationWarning, NormalizationEvent,
};
use crate::identity::CanonicalEquivalence;

use super::artifacts::{CanonicalQueryArtifact, CanonicalResultShapeArtifact};
use super::errors::QueryCanonicalizationError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalQueryBundle {
    pub(crate) query: CanonicalQueryArtifact,
    pub(crate) result_shape: CanonicalResultShapeArtifact,
    pub(crate) report: CanonicalizationReport,
    pub(crate) counters: CanonicalizationCounters,
}

impl CanonicalQueryBundle {
    pub fn query(&self) -> &CanonicalQueryArtifact {
        &self.query
    }

    pub fn result_shape(&self) -> &CanonicalResultShapeArtifact {
        &self.result_shape
    }

    pub fn report(&self) -> &CanonicalizationReport {
        &self.report
    }

    pub fn counters(&self) -> &CanonicalizationCounters {
        &self.counters
    }

    pub fn equivalence_to(&self, other: &Self) -> CanonicalEquivalence {
        if self.query.equivalence_to(&other.query) == CanonicalEquivalence::Equivalent
            && self.result_shape.equivalence_to(&other.result_shape)
                == CanonicalEquivalence::Equivalent
            && self.report.compatibility() == other.report.compatibility()
            && self.report.identity_freeze() == other.report.identity_freeze()
        {
            CanonicalEquivalence::Equivalent
        } else {
            CanonicalEquivalence::Distinct
        }
    }

    pub fn check_invariants(&self) -> Result<(), QueryCanonicalizationError> {
        if self.report.identity_freeze().query_digest != self.query.digest().as_str() {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message: "query digest mismatch between bundle and identity freeze evidence",
            });
        }

        if self.report.identity_freeze().result_shape_digest != self.result_shape.digest().as_str()
        {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message: "result-shape digest mismatch between bundle and identity freeze evidence",
            });
        }

        if self.report.normalized_projection_entries() != self.query.projection().len() {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message:
                    "normalized projection count does not match canonical query projection length",
            });
        }

        if self.report.normalized_traversal_entries() != self.query.traversal().len() {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message:
                    "normalized traversal count does not match canonical query traversal length",
            });
        }

        if self.report.normalized_result_fields() != self.result_shape.fields().len() {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message: "normalized result field count does not match canonical result-shape field length",
            });
        }

        if self.counters.canonicalization_warning_count != self.report.warnings().len() {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message: "warning count does not match warning list length",
            });
        }

        let compatibility_events = self
            .report
            .events()
            .iter()
            .filter(|event| matches!(event, NormalizationEvent::CompatibilityEstablished))
            .count();
        if compatibility_events != 1 {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message: "compatibility must be established exactly once",
            });
        }

        let identity_freeze_events = self
            .report
            .events()
            .iter()
            .filter(|event| matches!(event, NormalizationEvent::IdentityFrozen { .. }))
            .count();
        if identity_freeze_events != 1 {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message: "identity must be frozen exactly once",
            });
        }

        let retained_projection_events = self
            .report
            .events()
            .iter()
            .filter(|event| matches!(event, NormalizationEvent::ProjectionRetained { .. }))
            .count();
        if retained_projection_events != self.query.projection().len() {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message:
                    "projection retained event count does not match canonical projection length",
            });
        }

        let retained_traversal_events = self
            .report
            .events()
            .iter()
            .filter(|event| matches!(event, NormalizationEvent::TraversalRetained { .. }))
            .count();
        if retained_traversal_events != self.query.traversal().len() {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message: "traversal retained event count does not match canonical traversal length",
            });
        }

        let retained_result_field_events = self
            .report
            .events()
            .iter()
            .filter(|event| matches!(event, NormalizationEvent::ResultFieldRetained { .. }))
            .count();
        if retained_result_field_events != self.result_shape.fields().len() {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message:
                    "result-field retained event count does not match canonical result-field length",
            });
        }

        let retained_identity_binding_events = self
            .report
            .events()
            .iter()
            .filter(|event| matches!(event, NormalizationEvent::IdentityBindingRetained { .. }))
            .count();
        if retained_identity_binding_events != self.query.identity_bindings().len() {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message: "identity-binding retained event count does not match canonical identity-binding length",
            });
        }

        let query_duplicate_events = self
            .report
            .events()
            .iter()
            .filter(|event| {
                matches!(
                    event,
                    NormalizationEvent::ProjectionCollapsedDuplicate { .. }
                        | NormalizationEvent::TraversalCollapsedDuplicate { .. }
                )
            })
            .count();
        if query_duplicate_events != self.counters.query_deduplication_count {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message: "query duplicate event count does not match query deduplication counter",
            });
        }

        let result_field_duplicate_events = self
            .report
            .events()
            .iter()
            .filter(|event| {
                matches!(
                    event,
                    NormalizationEvent::ResultFieldCollapsedDuplicate { .. }
                )
            })
            .count();
        if result_field_duplicate_events != self.counters.result_shape_deduplication_count {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message: "result-field duplicate event count does not match result-shape deduplication counter",
            });
        }

        let ignored_binding_events = self
            .report
            .events()
            .iter()
            .filter(|event| matches!(event, NormalizationEvent::NonIdentityBindingIgnored { .. }))
            .count();
        let ignored_binding_warnings = self
            .report
            .warnings()
            .iter()
            .filter(|warning| {
                matches!(
                    warning,
                    CanonicalizationWarning::NonIdentityBindingMetadataIgnored { .. }
                )
            })
            .count();
        if ignored_binding_events != ignored_binding_warnings {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message: "ignored binding event count does not match ignored binding warning count",
            });
        }

        let projection_duplicate_warnings = self
            .report
            .warnings()
            .iter()
            .filter(|warning| {
                matches!(
                    warning,
                    CanonicalizationWarning::DuplicateProjectionCollapsed { .. }
                )
            })
            .count();
        let traversal_duplicate_warnings = self
            .report
            .warnings()
            .iter()
            .filter(|warning| {
                matches!(
                    warning,
                    CanonicalizationWarning::DuplicateTraversalCollapsed { .. }
                )
            })
            .count();
        if projection_duplicate_warnings + traversal_duplicate_warnings
            != self.counters.query_deduplication_count
        {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message: "query duplicate warning count does not match query deduplication counter",
            });
        }

        let result_field_duplicate_warnings = self
            .report
            .warnings()
            .iter()
            .filter(|warning| {
                matches!(
                    warning,
                    CanonicalizationWarning::DuplicateResultFieldCollapsed { .. }
                )
            })
            .count();
        if result_field_duplicate_warnings != self.counters.result_shape_deduplication_count {
            return Err(QueryCanonicalizationError::BundleInvariantViolation {
                message: "result-field duplicate warning count does not match result-shape deduplication counter",
            });
        }

        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn query_mut_for_test(&mut self) -> &mut CanonicalQueryArtifact {
        &mut self.query
    }

    #[cfg(test)]
    pub(crate) fn result_shape_mut_for_test(&mut self) -> &mut CanonicalResultShapeArtifact {
        &mut self.result_shape
    }

    #[cfg(test)]
    pub(crate) fn report_mut_for_test(&mut self) -> &mut CanonicalizationReport {
        &mut self.report
    }

    #[cfg(test)]
    pub(crate) fn counters_mut_for_test(&mut self) -> &mut CanonicalizationCounters {
        &mut self.counters
    }
}

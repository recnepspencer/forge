#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationFailureClass {
    ProjectionRejection,
    PredicateRejection,
    OrderingRejection,
    TraversalRejection,
    ProjectionWideningDenied,
    ResultShapeRejection,
    SchemaBasisRejection,
    BundleCompatibilityRejection,
    InternalInvariantBreak,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryValidationError {
    UnknownAspect {
        aspect: String,
    },
    UnknownField {
        aspect: String,
        field: String,
    },
    NonQueryableField {
        aspect: String,
        field: String,
    },
    UnsupportedStructuredContentProjection {
        aspect: String,
        field: String,
    },
    UnknownOrderingField {
        aspect: String,
        field: String,
    },
    NonOrderableField {
        aspect: String,
        field: String,
    },
    UnsupportedStructuredContentOrdering {
        aspect: String,
        field: String,
        direction: &'static str,
    },
    IncompatiblePredicateFamily {
        aspect: String,
        field: String,
        predicate_family: &'static str,
        field_kind: &'static str,
    },
    UnsupportedStructuredContentPredicate {
        aspect: String,
        field: String,
        predicate_family: &'static str,
    },
    ContradictoryPredicateSet {
        aspect: String,
        field: String,
        reason: &'static str,
    },
    IllegalWorkflowPredicateCapabilityOrContextShape {
        aspect: String,
        field: String,
        predicate_family: &'static str,
    },
    IllegalTraversalRelation {
        relation: String,
    },
    IllegalTraversalDepth {
        relation: String,
        requested_depth: u8,
        max_depth: u8,
    },
    IllegalResultShapeBinding {
        aspect: String,
        field: String,
        delivered_name: String,
    },
    ProjectionWideningDenied {
        aspect: String,
        field: String,
    },
    SchemaBasisIncompatibility {
        expected_basis: String,
        actual_basis: String,
    },
    ValidatedBundleCompatibilityFailure {
        message: &'static str,
    },
    ValidationInvariantViolation {
        message: &'static str,
    },
}

impl QueryValidationError {
    pub fn failure_class(&self) -> ValidationFailureClass {
        match self {
            Self::UnknownAspect { .. }
            | Self::UnknownField { .. }
            | Self::NonQueryableField { .. }
            | Self::UnsupportedStructuredContentProjection { .. } => {
                ValidationFailureClass::ProjectionRejection
            }
            Self::UnknownOrderingField { .. }
            | Self::NonOrderableField { .. }
            | Self::UnsupportedStructuredContentOrdering { .. } => {
                ValidationFailureClass::OrderingRejection
            }
            Self::IncompatiblePredicateFamily { .. }
            | Self::UnsupportedStructuredContentPredicate { .. }
            | Self::ContradictoryPredicateSet { .. }
            | Self::IllegalWorkflowPredicateCapabilityOrContextShape { .. } => {
                ValidationFailureClass::PredicateRejection
            }
            Self::IllegalTraversalRelation { .. } | Self::IllegalTraversalDepth { .. } => {
                ValidationFailureClass::TraversalRejection
            }
            Self::IllegalResultShapeBinding { .. } => ValidationFailureClass::ResultShapeRejection,
            Self::ProjectionWideningDenied { .. } => {
                ValidationFailureClass::ProjectionWideningDenied
            }
            Self::SchemaBasisIncompatibility { .. } => ValidationFailureClass::SchemaBasisRejection,
            Self::ValidatedBundleCompatibilityFailure { .. } => {
                ValidationFailureClass::BundleCompatibilityRejection
            }
            Self::ValidationInvariantViolation { .. } => {
                ValidationFailureClass::InternalInvariantBreak
            }
        }
    }

    pub fn failure_digest(&self) -> String {
        match self {
            Self::UnknownAspect { aspect } => format!("unknown-aspect:{aspect}"),
            Self::UnknownField { aspect, field } => format!("unknown-field:{aspect}:{field}"),
            Self::NonQueryableField { aspect, field } => {
                format!("non-queryable-field:{aspect}:{field}")
            }
            Self::UnsupportedStructuredContentProjection { aspect, field } => {
                format!("unsupported-structured-content-projection:{aspect}:{field}")
            }
            Self::UnknownOrderingField { aspect, field } => {
                format!("unknown-ordering-field:{aspect}:{field}")
            }
            Self::NonOrderableField { aspect, field } => {
                format!("non-orderable-field:{aspect}:{field}")
            }
            Self::UnsupportedStructuredContentOrdering {
                aspect,
                field,
                direction,
            } => format!(
                "unsupported-structured-content-ordering:{aspect}:{field}:{direction}"
            ),
            Self::IncompatiblePredicateFamily {
                aspect,
                field,
                predicate_family,
                field_kind,
            } => format!(
                "incompatible-predicate-family:{aspect}:{field}:{predicate_family}:{field_kind}"
            ),
            Self::UnsupportedStructuredContentPredicate {
                aspect,
                field,
                predicate_family,
            } => format!(
                "unsupported-structured-content-predicate:{aspect}:{field}:{predicate_family}"
            ),
            Self::ContradictoryPredicateSet {
                aspect,
                field,
                reason,
            } => format!("contradictory-predicate-set:{aspect}:{field}:{reason}"),
            Self::IllegalWorkflowPredicateCapabilityOrContextShape {
                aspect,
                field,
                predicate_family,
            } => format!(
                "illegal-workflow-predicate-capability-or-context-shape:{aspect}:{field}:{predicate_family}"
            ),
            Self::IllegalTraversalRelation { relation } => {
                format!("illegal-traversal-relation:{relation}")
            }
            Self::IllegalTraversalDepth {
                relation,
                requested_depth,
                max_depth,
            } => format!("illegal-traversal-depth:{relation}:{requested_depth}:{max_depth}"),
            Self::IllegalResultShapeBinding {
                aspect,
                field,
                delivered_name,
            } => format!("illegal-result-shape-binding:{aspect}:{field}:{delivered_name}"),
            Self::ProjectionWideningDenied { aspect, field } => {
                format!("projection-widening-denied:{aspect}:{field}")
            }
            Self::SchemaBasisIncompatibility {
                expected_basis,
                actual_basis,
            } => format!("schema-basis-incompatibility:{expected_basis}:{actual_basis}"),
            Self::ValidatedBundleCompatibilityFailure { message } => {
                format!("validated-bundle-compatibility-failure:{message}")
            }
            Self::ValidationInvariantViolation { message } => {
                format!("validation-invariant-violation:{message}")
            }
        }
    }
}

use crate::canonicalization::{
    CanonicalOrderingEntry, CanonicalPredicateEntry, CanonicalPredicateOperand,
    CanonicalProjectionEntry, CanonicalQueryArtifact, CanonicalResultField,
    CanonicalResultShapeArtifact, CanonicalTraversalEntry,
};
use crate::identity::{
    CanonicalEquivalence, CanonicalQueryDigest, CanonicalResultShapeDigest, SchemaBasisDigest,
    ValidatedQueryDigest, ValidatedResultShapeDigest,
};
use crate::schema_view::SchemaFieldKind;

use super::{
    QueryValidationCounters, QueryValidationError, QueryValidationReport, ValidationEvent,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ValidatedProjectionEntry {
    aspect: String,
    field: String,
    field_kind: SchemaFieldKind,
}

impl ValidatedProjectionEntry {
    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn field_kind(&self) -> &SchemaFieldKind {
        &self.field_kind
    }

    pub(crate) fn from_canonical(
        entry: &CanonicalProjectionEntry,
        field_kind: SchemaFieldKind,
    ) -> Self {
        Self {
            aspect: entry.aspect.clone(),
            field: entry.field.clone(),
            field_kind,
        }
    }

    fn digest_part(&self) -> String {
        format!(
            "validated-projection:{}:{}:{:?}",
            self.aspect, self.field, self.field_kind
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ValidatedTraversalEntry {
    relation: String,
    depth: u8,
    max_depth: u8,
}

impl ValidatedTraversalEntry {
    pub fn relation(&self) -> &str {
        &self.relation
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }

    pub fn max_depth(&self) -> u8 {
        self.max_depth
    }

    pub(crate) fn from_canonical(entry: &CanonicalTraversalEntry, max_depth: u8) -> Self {
        Self {
            relation: entry.relation.clone(),
            depth: entry.depth,
            max_depth,
        }
    }

    fn digest_part(&self) -> String {
        format!(
            "validated-traversal:{}:{}:{}",
            self.relation, self.depth, self.max_depth
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ValidatedResultShapeBinding {
    source_aspect: String,
    source_field: String,
    delivered_name: String,
    field_kind: SchemaFieldKind,
}

impl ValidatedResultShapeBinding {
    pub fn source_aspect(&self) -> &str {
        &self.source_aspect
    }

    pub fn source_field(&self) -> &str {
        &self.source_field
    }

    pub fn delivered_name(&self) -> &str {
        &self.delivered_name
    }

    pub fn field_kind(&self) -> &SchemaFieldKind {
        &self.field_kind
    }

    pub(crate) fn from_canonical(
        field: &CanonicalResultField,
        field_kind: SchemaFieldKind,
    ) -> Self {
        Self {
            source_aspect: field.source_aspect.clone(),
            source_field: field.source_field.clone(),
            delivered_name: field.delivered_name.clone(),
            field_kind,
        }
    }

    fn digest_part(&self) -> String {
        format!(
            "validated-result-binding:{}:{}:{}:{:?}",
            self.source_aspect, self.source_field, self.delivered_name, self.field_kind
        )
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValidatedPredicateSet {
    entries: Vec<ValidatedPredicateEntry>,
}

impl ValidatedPredicateSet {
    pub fn entries(&self) -> &[ValidatedPredicateEntry] {
        &self.entries
    }

    fn digest_parts(&self) -> impl Iterator<Item = String> + '_ {
        self.entries
            .iter()
            .map(ValidatedPredicateEntry::digest_part)
    }

    pub(crate) fn from_entries(entries: Vec<ValidatedPredicateEntry>) -> Self {
        Self { entries }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValidatedOrderingSet {
    entries: Vec<ValidatedOrderingEntry>,
}

impl ValidatedOrderingSet {
    pub fn entries(&self) -> &[ValidatedOrderingEntry] {
        &self.entries
    }

    fn digest_parts(&self) -> impl Iterator<Item = String> + '_ {
        self.entries.iter().map(ValidatedOrderingEntry::digest_part)
    }

    pub(crate) fn from_entries(entries: Vec<ValidatedOrderingEntry>) -> Self {
        Self { entries }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedQueryArtifact {
    digest: ValidatedQueryDigest,
    canonical_query_digest: CanonicalQueryDigest,
    schema_basis: SchemaBasisDigest,
    projection: Vec<ValidatedProjectionEntry>,
    traversal: Vec<ValidatedTraversalEntry>,
    predicates: ValidatedPredicateSet,
    ordering: ValidatedOrderingSet,
}

impl ValidatedQueryArtifact {
    pub fn digest(&self) -> &ValidatedQueryDigest {
        &self.digest
    }

    pub fn canonical_query_digest(&self) -> &CanonicalQueryDigest {
        &self.canonical_query_digest
    }

    pub fn schema_basis(&self) -> &SchemaBasisDigest {
        &self.schema_basis
    }

    pub fn projection(&self) -> &[ValidatedProjectionEntry] {
        &self.projection
    }

    pub fn traversal(&self) -> &[ValidatedTraversalEntry] {
        &self.traversal
    }

    pub fn predicates(&self) -> &ValidatedPredicateSet {
        &self.predicates
    }

    pub fn ordering(&self) -> &ValidatedOrderingSet {
        &self.ordering
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedResultShapeArtifact {
    digest: ValidatedResultShapeDigest,
    canonical_result_shape_digest: CanonicalResultShapeDigest,
    schema_basis: SchemaBasisDigest,
    bindings: Vec<ValidatedResultShapeBinding>,
}

impl ValidatedResultShapeArtifact {
    pub fn digest(&self) -> &ValidatedResultShapeDigest {
        &self.digest
    }

    pub fn canonical_result_shape_digest(&self) -> &CanonicalResultShapeDigest {
        &self.canonical_result_shape_digest
    }

    pub fn schema_basis(&self) -> &SchemaBasisDigest {
        &self.schema_basis
    }

    pub fn bindings(&self) -> &[ValidatedResultShapeBinding] {
        &self.bindings
    }
}

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
        if self.query.digest == other.query.digest
            && self.result_shape.digest == other.result_shape.digest
        {
            CanonicalEquivalence::Equivalent
        } else {
            CanonicalEquivalence::Distinct
        }
    }

    pub fn check_invariants(&self) -> Result<(), QueryValidationError> {
        if self.query.schema_basis != self.result_shape.schema_basis {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated query and result-shape schema bases diverged",
            });
        }

        if self.report.schema_basis_digest() != self.query.schema_basis.as_str() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message:
                    "validation report schema basis does not match validated artifact schema basis",
            });
        }

        if self.report.validated_projection_entries() != self.query.projection.len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated projection count does not match validated projection length",
            });
        }

        if self.report.validated_traversal_entries() != self.query.traversal.len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated traversal count does not match validated traversal length",
            });
        }

        if self.report.validated_result_shape_bindings() != self.result_shape.bindings.len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated result-shape binding count does not match validated result-shape binding length",
            });
        }

        if self.report.validated_predicates() != self.query.predicates.entries().len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated predicate count does not match validated predicate length",
            });
        }

        if self.report.validated_ordering_fields() != self.query.ordering.entries().len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated ordering count does not match validated ordering length",
            });
        }

        if self.counters.validated_projection_entry_count() != self.query.projection.len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated projection counter does not match validated projection length",
            });
        }

        if self.counters.validated_traversal_clause_count() != self.query.traversal.len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated traversal counter does not match validated traversal length",
            });
        }

        if self.counters.validated_result_shape_binding_count() != self.result_shape.bindings.len()
        {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message:
                    "validated result-shape binding counter does not match validated binding length",
            });
        }

        if self.counters.validated_predicate_count() != self.query.predicates.entries().len() {
            return Err(QueryValidationError::ValidationInvariantViolation {
                message: "validated predicate counter does not match validated predicate length",
            });
        }

        if self.counters.validated_ordering_field_count() != self.query.ordering.entries().len() {
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

pub(crate) fn build_validated_query_artifact(
    canonical_query: &CanonicalQueryArtifact,
    schema_basis: &SchemaBasisDigest,
    projection: Vec<ValidatedProjectionEntry>,
    traversal: Vec<ValidatedTraversalEntry>,
    predicates: ValidatedPredicateSet,
    ordering: ValidatedOrderingSet,
) -> ValidatedQueryArtifact {
    let mut parts = vec![
        format!("family:{:?}", canonical_query.family()),
        format!("root:{}", canonical_query.root().as_str()),
        format!("schema_basis:{}", schema_basis.as_str()),
    ];
    parts.extend(canonical_query.identity_bindings().iter().map(|binding| {
        format!(
            "binding:{}:{:?}",
            binding.slot().as_str(),
            binding.subject()
        )
    }));
    parts.extend(projection.iter().map(ValidatedProjectionEntry::digest_part));
    parts.extend(traversal.iter().map(ValidatedTraversalEntry::digest_part));
    parts.extend(predicates.digest_parts());
    parts.extend(ordering.digest_parts());

    ValidatedQueryArtifact {
        digest: ValidatedQueryDigest::from_parts(&parts),
        canonical_query_digest: canonical_query.digest().clone(),
        schema_basis: schema_basis.clone(),
        projection,
        traversal,
        predicates,
        ordering,
    }
}

pub(crate) fn build_validated_result_shape_artifact(
    canonical_result_shape: &CanonicalResultShapeArtifact,
    schema_basis: &SchemaBasisDigest,
    bindings: Vec<ValidatedResultShapeBinding>,
) -> ValidatedResultShapeArtifact {
    let mut parts = vec![
        format!(
            "canonical_result_shape:{}",
            canonical_result_shape.digest().as_str()
        ),
        format!("schema_basis:{}", schema_basis.as_str()),
    ];
    parts.extend(
        bindings
            .iter()
            .map(ValidatedResultShapeBinding::digest_part),
    );

    ValidatedResultShapeArtifact {
        digest: ValidatedResultShapeDigest::from_parts(&parts),
        canonical_result_shape_digest: canonical_result_shape.digest().clone(),
        schema_basis: schema_basis.clone(),
        bindings,
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ValidatedPredicateEntry {
    aspect: String,
    field: String,
    predicate_family: &'static str,
    field_kind: SchemaFieldKind,
    value_kind: &'static str,
    value_basis: String,
}

impl ValidatedPredicateEntry {
    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn predicate_family(&self) -> &'static str {
        self.predicate_family
    }

    pub fn field_kind(&self) -> &SchemaFieldKind {
        &self.field_kind
    }

    pub fn value_kind(&self) -> &'static str {
        self.value_kind
    }

    pub fn value_basis(&self) -> &str {
        &self.value_basis
    }

    pub(crate) fn from_canonical(
        entry: &CanonicalPredicateEntry,
        field_kind: SchemaFieldKind,
        value_kind: &'static str,
    ) -> Self {
        Self {
            aspect: entry.aspect.clone(),
            field: entry.field.clone(),
            predicate_family: match entry.family {
                crate::canonicalization::CanonicalPredicateFamily::Equality => "equality",
                crate::canonicalization::CanonicalPredicateFamily::IntegerGreaterThan => {
                    "integer-greater-than"
                }
                crate::canonicalization::CanonicalPredicateFamily::IntegerLessThan => {
                    "integer-less-than"
                }
                crate::canonicalization::CanonicalPredicateFamily::StringContains => {
                    "string-contains"
                }
                crate::canonicalization::CanonicalPredicateFamily::ScalarMembership => {
                    "scalar-membership"
                }
                crate::canonicalization::CanonicalPredicateFamily::PresenceIsPresent => {
                    "presence-is-present"
                }
            },
            field_kind,
            value_kind,
            value_basis: canonical_operand_basis(&entry.operand),
        }
    }

    fn digest_part(&self) -> String {
        format!(
            "validated-predicate:{}:{}:{}:{:?}:{}:{}",
            self.aspect,
            self.field,
            self.predicate_family,
            self.field_kind,
            self.value_kind,
            self.value_basis
        )
    }
}

fn canonical_operand_basis(operand: &CanonicalPredicateOperand) -> String {
    match operand {
        CanonicalPredicateOperand::Scalar(value) => match value {
            crate::authoring::ScalarPredicateValue::String(value) => format!("string:{value}"),
            crate::authoring::ScalarPredicateValue::Integer(value) => format!("integer:{value}"),
            crate::authoring::ScalarPredicateValue::Boolean(value) => format!("boolean:{value}"),
        },
        CanonicalPredicateOperand::ScalarSet(values) => format!(
            "set:[{}]",
            values
                .iter()
                .map(|value| match value {
                    crate::authoring::ScalarPredicateValue::String(value) => {
                        format!("string:{value}")
                    }
                    crate::authoring::ScalarPredicateValue::Integer(value) => {
                        format!("integer:{value}")
                    }
                    crate::authoring::ScalarPredicateValue::Boolean(value) => {
                        format!("boolean:{value}")
                    }
                })
                .collect::<Vec<_>>()
                .join(",")
        ),
        CanonicalPredicateOperand::Presence(kind) => format!("presence:{kind}"),
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ValidatedOrderingEntry {
    aspect: String,
    field: String,
    direction: &'static str,
    field_kind: SchemaFieldKind,
    projected: bool,
}

impl ValidatedOrderingEntry {
    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn direction(&self) -> &'static str {
        self.direction
    }

    pub fn field_kind(&self) -> &SchemaFieldKind {
        &self.field_kind
    }

    pub fn projected(&self) -> bool {
        self.projected
    }

    pub(crate) fn from_canonical(
        entry: &CanonicalOrderingEntry,
        field_kind: SchemaFieldKind,
        projected: bool,
    ) -> Self {
        Self {
            aspect: entry.aspect.clone(),
            field: entry.field.clone(),
            direction: match entry.direction {
                crate::authoring::OrderingDirection::Ascending => "ascending",
                crate::authoring::OrderingDirection::Descending => "descending",
            },
            field_kind,
            projected,
        }
    }

    fn digest_part(&self) -> String {
        format!(
            "validated-ordering:{}:{}:{}:{:?}:{}",
            self.aspect, self.field, self.direction, self.field_kind, self.projected
        )
    }
}

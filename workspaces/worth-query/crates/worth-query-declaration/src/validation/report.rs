#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationEvent {
    PredicateValidated {
        aspect: String,
        field: String,
        predicate_family: &'static str,
        field_kind: String,
    },
    ProjectionValidated {
        aspect: String,
        field: String,
        field_kind: String,
    },
    OrderingValidated {
        aspect: String,
        field: String,
        direction: &'static str,
        field_kind: String,
        projected: bool,
    },
    TraversalValidated {
        relation: String,
        depth: u8,
        max_depth: u8,
    },
    ResultShapeBindingValidated {
        aspect: String,
        field: String,
        delivered_name: String,
        field_kind: String,
    },
    CompatibilityEstablished,
    IdentityFrozen {
        query_digest: String,
        result_shape_digest: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationWarning {}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValidationRejectionMatrix {
    projection_rejections: usize,
    ordering_rejections: usize,
    predicate_rejections: usize,
    traversal_rejections: usize,
    result_shape_rejections: usize,
    schema_basis_rejections: usize,
    compatibility_rejections: usize,
}

impl ValidationRejectionMatrix {
    pub fn projection_rejections(&self) -> usize {
        self.projection_rejections
    }

    pub fn traversal_rejections(&self) -> usize {
        self.traversal_rejections
    }

    pub fn ordering_rejections(&self) -> usize {
        self.ordering_rejections
    }

    pub fn predicate_rejections(&self) -> usize {
        self.predicate_rejections
    }

    pub fn result_shape_rejections(&self) -> usize {
        self.result_shape_rejections
    }

    pub fn schema_basis_rejections(&self) -> usize {
        self.schema_basis_rejections
    }

    pub fn compatibility_rejections(&self) -> usize {
        self.compatibility_rejections
    }

    pub fn record_projection_rejection(&mut self) {
        self.projection_rejections += 1;
    }

    pub fn record_ordering_rejection(&mut self) {
        self.ordering_rejections += 1;
    }

    pub fn record_predicate_rejection(&mut self) {
        self.predicate_rejections += 1;
    }

    pub fn record_traversal_rejection(&mut self) {
        self.traversal_rejections += 1;
    }

    pub fn record_result_shape_rejection(&mut self) {
        self.result_shape_rejections += 1;
    }
    pub fn record_compatibility_rejection(&mut self) {
        self.compatibility_rejections += 1;
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryValidationReport {
    schema_basis_digest: SchemaBasisDigest,
    validated_projection_entries: usize,
    validated_traversal_entries: usize,
    validated_result_shape_bindings: usize,
    validated_predicates: usize,
    validated_ordering_fields: usize,
    events: Vec<ValidationEvent>,
    warnings: Vec<ValidationWarning>,
    rejection_matrix: ValidationRejectionMatrix,
}

impl QueryValidationReport {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema_basis_digest: SchemaBasisDigest,
        validated_projection_entries: usize,
        validated_traversal_entries: usize,
        validated_result_shape_bindings: usize,
        validated_predicates: usize,
        validated_ordering_fields: usize,
        events: Vec<ValidationEvent>,
        warnings: Vec<ValidationWarning>,
        rejection_matrix: ValidationRejectionMatrix,
    ) -> Self {
        Self {
            schema_basis_digest,
            validated_projection_entries,
            validated_traversal_entries,
            validated_result_shape_bindings,
            validated_predicates,
            validated_ordering_fields,
            events,
            warnings,
            rejection_matrix,
        }
    }

    pub fn schema_basis_digest(&self) -> &SchemaBasisDigest {
        &self.schema_basis_digest
    }

    pub fn validated_projection_entries(&self) -> usize {
        self.validated_projection_entries
    }

    pub fn validated_traversal_entries(&self) -> usize {
        self.validated_traversal_entries
    }

    pub fn validated_result_shape_bindings(&self) -> usize {
        self.validated_result_shape_bindings
    }

    pub fn validated_predicates(&self) -> usize {
        self.validated_predicates
    }

    pub fn validated_ordering_fields(&self) -> usize {
        self.validated_ordering_fields
    }

    pub fn events(&self) -> &[ValidationEvent] {
        &self.events
    }

    pub fn warnings(&self) -> &[ValidationWarning] {
        &self.warnings
    }

    pub fn rejection_matrix(&self) -> &ValidationRejectionMatrix {
        &self.rejection_matrix
    }
}
use crate::identity::SchemaBasisDigest;

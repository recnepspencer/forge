#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QueryValidationCounters {
    validated_predicate_count: usize,
    validated_projection_entry_count: usize,
    validated_traversal_clause_count: usize,
    validated_result_shape_binding_count: usize,
    validated_ordering_field_count: usize,
    schema_lookup_count: usize,
    validation_rejection_count: usize,
    projection_widening_denial_count: usize,
    validation_warning_count: usize,
    validation_fallback_count: usize,
}

impl QueryValidationCounters {
    pub fn validated_predicate_count(&self) -> usize {
        self.validated_predicate_count
    }

    pub fn validated_projection_entry_count(&self) -> usize {
        self.validated_projection_entry_count
    }

    pub fn validated_traversal_clause_count(&self) -> usize {
        self.validated_traversal_clause_count
    }

    pub fn validated_result_shape_binding_count(&self) -> usize {
        self.validated_result_shape_binding_count
    }

    pub fn validated_ordering_field_count(&self) -> usize {
        self.validated_ordering_field_count
    }

    pub fn schema_lookup_count(&self) -> usize {
        self.schema_lookup_count
    }

    pub fn validation_rejection_count(&self) -> usize {
        self.validation_rejection_count
    }

    pub fn projection_widening_denial_count(&self) -> usize {
        self.projection_widening_denial_count
    }

    pub fn validation_warning_count(&self) -> usize {
        self.validation_warning_count
    }

    pub fn validation_fallback_count(&self) -> usize {
        self.validation_fallback_count
    }

    pub(crate) fn record_projection_validated(&mut self) {
        self.validated_projection_entry_count += 1;
    }

    pub(crate) fn record_predicate_validated(&mut self) {
        self.validated_predicate_count += 1;
    }

    pub(crate) fn set_validated_predicate_count(&mut self, count: usize) {
        self.validated_predicate_count = count;
    }

    pub(crate) fn record_traversal_validated(&mut self) {
        self.validated_traversal_clause_count += 1;
    }

    pub(crate) fn record_result_shape_binding_validated(&mut self) {
        self.validated_result_shape_binding_count += 1;
    }

    pub(crate) fn record_ordering_validated(&mut self) {
        self.validated_ordering_field_count += 1;
    }

    pub(crate) fn record_schema_lookup(&mut self) {
        self.schema_lookup_count += 1;
    }

    pub(crate) fn record_rejection(&mut self) {
        self.validation_rejection_count += 1;
    }

    pub(crate) fn record_projection_widening_denial(&mut self) {
        self.projection_widening_denial_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn absorb(&mut self, other: &Self) {
        self.validated_predicate_count += other.validated_predicate_count;
        self.validated_projection_entry_count += other.validated_projection_entry_count;
        self.validated_traversal_clause_count += other.validated_traversal_clause_count;
        self.validated_result_shape_binding_count += other.validated_result_shape_binding_count;
        self.validated_ordering_field_count += other.validated_ordering_field_count;
        self.schema_lookup_count += other.schema_lookup_count;
        self.validation_rejection_count += other.validation_rejection_count;
        self.projection_widening_denial_count += other.projection_widening_denial_count;
        self.validation_warning_count += other.validation_warning_count;
        self.validation_fallback_count += other.validation_fallback_count;
    }
}

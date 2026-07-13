#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadBreadth {
    pub(crate) planned_read_surface_count: usize,
    pub(crate) planned_traversal_clause_count: usize,
    pub(crate) planned_traversal_depth_limit: usize,
    pub(crate) execution_query_projection_count: usize,
    pub(crate) execution_read_operation_count: usize,
    pub(crate) execution_records_examined_count: usize,
    pub(crate) execution_records_emitted_count: usize,
    pub(crate) execution_page_width: usize,
    pub(crate) execution_page_truncation_count: usize,
    pub(crate) execution_cursor_advance_count: usize,
    pub(crate) execution_materialized_relation_count: usize,
}

impl WorthQueryReadBreadth {
    pub fn planned_read_surface_count(&self) -> usize {
        self.planned_read_surface_count
    }

    pub fn planned_traversal_clause_count(&self) -> usize {
        self.planned_traversal_clause_count
    }

    pub fn planned_traversal_depth_limit(&self) -> usize {
        self.planned_traversal_depth_limit
    }

    pub fn execution_query_projection_count(&self) -> usize {
        self.execution_query_projection_count
    }

    pub fn execution_read_operation_count(&self) -> usize {
        self.execution_read_operation_count
    }

    pub fn execution_records_examined_count(&self) -> usize {
        self.execution_records_examined_count
    }

    pub fn execution_records_emitted_count(&self) -> usize {
        self.execution_records_emitted_count
    }

    pub fn execution_page_width(&self) -> usize {
        self.execution_page_width
    }

    pub fn execution_page_truncation_count(&self) -> usize {
        self.execution_page_truncation_count
    }

    pub fn execution_cursor_advance_count(&self) -> usize {
        self.execution_cursor_advance_count
    }

    pub fn execution_materialized_relation_count(&self) -> usize {
        self.execution_materialized_relation_count
    }
}

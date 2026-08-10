#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PlanningCounters {
    planned_projection_entry_count: usize,
    planned_traversal_clause_count: usize,
    route_candidate_count: usize,
    planned_read_surface_count: usize,
    planned_fallback_option_count: usize,
    fallback_denial_count: usize,
    planned_materialization_edge_class_count: usize,
    planned_traversal_depth_limit: usize,
    planned_aggregate_input_breadth: usize,
    planned_cdc_family_count: usize,
}

impl PlanningCounters {
    pub fn planned_projection_entry_count(&self) -> usize {
        self.planned_projection_entry_count
    }

    pub fn planned_traversal_clause_count(&self) -> usize {
        self.planned_traversal_clause_count
    }

    pub fn route_candidate_count(&self) -> usize {
        self.route_candidate_count
    }

    pub fn planned_read_surface_count(&self) -> usize {
        self.planned_read_surface_count
    }

    pub fn planned_fallback_option_count(&self) -> usize {
        self.planned_fallback_option_count
    }

    pub fn fallback_denial_count(&self) -> usize {
        self.fallback_denial_count
    }

    pub fn planned_materialization_edge_class_count(&self) -> usize {
        self.planned_materialization_edge_class_count
    }

    pub fn planned_traversal_depth_limit(&self) -> usize {
        self.planned_traversal_depth_limit
    }

    pub fn planned_aggregate_input_breadth(&self) -> usize {
        self.planned_aggregate_input_breadth
    }

    pub fn planned_cdc_family_count(&self) -> usize {
        self.planned_cdc_family_count
    }

    pub(crate) fn new(
        planned_projection_entry_count: usize,
        planned_traversal_clause_count: usize,
        route_candidate_count: usize,
        planned_read_surface_count: usize,
        planned_fallback_option_count: usize,
        fallback_denial_count: usize,
        planned_materialization_edge_class_count: usize,
        planned_traversal_depth_limit: usize,
        planned_aggregate_input_breadth: usize,
        planned_cdc_family_count: usize,
    ) -> Self {
        Self {
            planned_projection_entry_count,
            planned_traversal_clause_count,
            route_candidate_count,
            planned_read_surface_count,
            planned_fallback_option_count,
            fallback_denial_count,
            planned_materialization_edge_class_count,
            planned_traversal_depth_limit,
            planned_aggregate_input_breadth,
            planned_cdc_family_count,
        }
    }
}

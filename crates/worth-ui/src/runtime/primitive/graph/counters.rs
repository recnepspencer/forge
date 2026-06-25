#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveGraphCounters {
    schema_count: usize,
    family_admission_count: usize,
    published_fact_count: usize,
    dependency_fact_count: usize,
    query_projection_fact_count: usize,
    obligation_selected_count: usize,
    obligation_not_applicable_count: usize,
    source_reparse_count: usize,
    renderer_prop_parse_count: usize,
}

impl WorthUiPrimitiveGraphCounters {
    pub(crate) fn new(
        schema_count: usize,
        family_admission_count: usize,
        published_fact_count: usize,
        dependency_fact_count: usize,
        query_projection_fact_count: usize,
        obligation_selected_count: usize,
        obligation_not_applicable_count: usize,
    ) -> Self {
        Self {
            schema_count,
            family_admission_count,
            published_fact_count,
            dependency_fact_count,
            query_projection_fact_count,
            obligation_selected_count,
            obligation_not_applicable_count,
            source_reparse_count: 0,
            renderer_prop_parse_count: 0,
        }
    }

    pub fn schema_count(self) -> usize {
        self.schema_count
    }

    pub fn family_admission_count(self) -> usize {
        self.family_admission_count
    }

    pub fn published_fact_count(self) -> usize {
        self.published_fact_count
    }

    pub fn dependency_fact_count(self) -> usize {
        self.dependency_fact_count
    }

    pub fn query_projection_fact_count(self) -> usize {
        self.query_projection_fact_count
    }

    pub fn obligation_selected_count(self) -> usize {
        self.obligation_selected_count
    }

    pub fn obligation_not_applicable_count(self) -> usize {
        self.obligation_not_applicable_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_prop_parse_count(self) -> usize {
        self.renderer_prop_parse_count
    }
}

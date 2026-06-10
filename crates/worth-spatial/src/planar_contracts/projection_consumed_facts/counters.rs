#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProjectionConsumedPlanarFactsCounters {
    retained_source_rows_inspected: usize,
    projection_receipts_consumed: usize,
    materialization_binding_rows: usize,
    rejected_projection_rows: usize,
    projection_consumption_breadth: usize,
}

impl ProjectionConsumedPlanarFactsCounters {
    pub(crate) fn consumed(
        retained_source_rows_inspected: usize,
        projection_receipts_consumed: usize,
        materialization_binding_rows: usize,
        projection_consumption_breadth: usize,
    ) -> Self {
        Self {
            retained_source_rows_inspected,
            projection_receipts_consumed,
            materialization_binding_rows,
            rejected_projection_rows: 0,
            projection_consumption_breadth,
        }
    }

    pub fn retained_source_rows_inspected(self) -> usize {
        self.retained_source_rows_inspected
    }

    pub fn projection_receipts_consumed(self) -> usize {
        self.projection_receipts_consumed
    }

    pub fn materialization_binding_rows(self) -> usize {
        self.materialization_binding_rows
    }

    pub fn rejected_projection_rows(self) -> usize {
        self.rejected_projection_rows
    }

    pub fn projection_consumption_breadth(self) -> usize {
        self.projection_consumption_breadth
    }
}

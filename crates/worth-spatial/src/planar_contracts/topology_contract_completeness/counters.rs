pub const REQUIRED_TOPOLOGY_COMPLETENESS_FACT_ROWS: usize = 7;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarTopologyContractCompletenessCounters {
    inspected_topology_fact_rows: usize,
    inspected_required_fact_rows: usize,
    rejected_missing_fact_rows: usize,
}

impl PlanarTopologyContractCompletenessCounters {
    pub(crate) const fn certified(
        inspected_topology_fact_rows: usize,
        inspected_required_fact_rows: usize,
    ) -> Self {
        Self {
            inspected_topology_fact_rows,
            inspected_required_fact_rows,
            rejected_missing_fact_rows: 0,
        }
    }

    pub(crate) const fn rejected_missing_fact() -> Self {
        Self {
            inspected_topology_fact_rows: 0,
            inspected_required_fact_rows: 0,
            rejected_missing_fact_rows: 1,
        }
    }

    pub fn inspected_topology_fact_rows(self) -> usize {
        self.inspected_topology_fact_rows
    }

    pub fn inspected_required_fact_rows(self) -> usize {
        self.inspected_required_fact_rows
    }

    pub fn rejected_missing_fact_rows(self) -> usize {
        self.rejected_missing_fact_rows
    }
}

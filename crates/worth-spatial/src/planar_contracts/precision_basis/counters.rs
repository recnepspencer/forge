#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarPrecisionPerformanceCounters {
    predicate_precision_rows_consumed: usize,
    precision_escalation_breadth: usize,
    local_coordinate_normalizations: usize,
    basis_digest_part_count: usize,
    scale_separation_calculations: usize,
}

impl PlanarPrecisionPerformanceCounters {
    pub(crate) const fn certified(
        basis_digest_part_count: usize,
        precision_escalation_breadth: usize,
    ) -> Self {
        Self {
            predicate_precision_rows_consumed: 1,
            precision_escalation_breadth,
            local_coordinate_normalizations: 1,
            basis_digest_part_count,
            scale_separation_calculations: 1,
        }
    }

    pub fn predicate_precision_rows_consumed(&self) -> usize {
        self.predicate_precision_rows_consumed
    }

    pub fn precision_escalation_breadth(&self) -> usize {
        self.precision_escalation_breadth
    }

    pub fn local_coordinate_normalizations(&self) -> usize {
        self.local_coordinate_normalizations
    }

    pub fn basis_digest_part_count(&self) -> usize {
        self.basis_digest_part_count
    }

    pub fn scale_separation_calculations(&self) -> usize {
        self.scale_separation_calculations
    }
}

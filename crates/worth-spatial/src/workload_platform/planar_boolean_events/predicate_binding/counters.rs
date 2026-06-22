#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanEventPredicateBindingCounters {
    required_segment_contracts: usize,
    supplied_segment_contracts: usize,
    bound_segment_pairs: usize,
    required_predicate_rows: usize,
    certified_predicate_rows: usize,
}

impl PlanarBooleanEventPredicateBindingCounters {
    pub(crate) fn new(
        required_segment_contracts: usize,
        supplied_segment_contracts: usize,
        bound_segment_pairs: usize,
        certified_predicate_rows: usize,
    ) -> Self {
        Self {
            required_segment_contracts,
            supplied_segment_contracts,
            bound_segment_pairs,
            required_predicate_rows: required_segment_contracts.saturating_mul(4),
            certified_predicate_rows,
        }
    }

    pub fn required_segment_contracts(self) -> usize {
        self.required_segment_contracts
    }

    pub fn supplied_segment_contracts(self) -> usize {
        self.supplied_segment_contracts
    }

    pub fn bound_segment_pairs(self) -> usize {
        self.bound_segment_pairs
    }

    pub fn required_predicate_rows(self) -> usize {
        self.required_predicate_rows
    }

    pub fn certified_predicate_rows(self) -> usize {
        self.certified_predicate_rows
    }
}

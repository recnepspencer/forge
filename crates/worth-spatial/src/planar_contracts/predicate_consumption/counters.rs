#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PredicateCertificateConsumptionCounters {
    certified_predicate_rows: usize,
    consumer_rows: usize,
    precision_metadata_rows: usize,
    rejected_substitute_rows: usize,
}

impl PredicateCertificateConsumptionCounters {
    pub(crate) fn certified(
        certified_predicate_rows: usize,
        consumer_rows: usize,
        precision_metadata_rows: usize,
    ) -> Self {
        Self {
            certified_predicate_rows,
            consumer_rows,
            precision_metadata_rows,
            rejected_substitute_rows: 0,
        }
    }

    pub(crate) fn rejected_substitute() -> Self {
        Self {
            certified_predicate_rows: 0,
            consumer_rows: 0,
            precision_metadata_rows: 0,
            rejected_substitute_rows: 1,
        }
    }

    pub fn certified_predicate_rows(self) -> usize {
        self.certified_predicate_rows
    }

    pub fn consumer_rows(self) -> usize {
        self.consumer_rows
    }

    pub fn precision_metadata_rows(self) -> usize {
        self.precision_metadata_rows
    }

    pub fn rejected_substitute_rows(self) -> usize {
        self.rejected_substitute_rows
    }
}

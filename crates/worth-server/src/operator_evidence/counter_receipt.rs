use worth_foundational::facade::{
    FoundationalAuthoritativePerformanceClaim, FoundationalCounterBackedPerformanceReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperatorCounterReceipt {
    inner: FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
}

impl WorthServerOperatorCounterReceipt {
    pub(crate) fn new(
        inner: FoundationalCounterBackedPerformanceReceipt<
            FoundationalAuthoritativePerformanceClaim,
        >,
    ) -> Self {
        Self { inner }
    }

    pub fn receipt(
        &self,
    ) -> &FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>
    {
        &self.inner
    }

    pub fn counter(&self, name: &str) -> Option<WorthServerObservedCounter<'_>> {
        self.inner
            .counter_rows()
            .iter()
            .find(|row| row.name().as_str() == name)
            .map(|row| WorthServerObservedCounter { row })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WorthServerObservedCounter<'a> {
    row: &'a worth_foundational::facade::FoundationalPerformanceCounterRow,
}

impl<'a> WorthServerObservedCounter<'a> {
    pub fn name(&self) -> &str {
        self.row.name().as_str()
    }

    pub fn exact_value(&self) -> u64 {
        self.row.observed_count()
    }
}

use forge_foundational::facade::{
    FoundationalAuthoritativePerformanceClaim, FoundationalCounterBackedPerformanceReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperatorCounterReceipt {
    inner: FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
}

impl ForgeServerOperatorCounterReceipt {
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

    pub fn counter(&self, name: &str) -> Option<ForgeServerObservedCounter<'_>> {
        self.inner
            .counter_rows()
            .iter()
            .find(|row| row.name().as_str() == name)
            .map(|row| ForgeServerObservedCounter { row })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ForgeServerObservedCounter<'a> {
    row: &'a forge_foundational::facade::FoundationalPerformanceCounterRow,
}

impl<'a> ForgeServerObservedCounter<'a> {
    pub fn name(&self) -> &str {
        self.row.name().as_str()
    }

    pub fn exact_value(&self) -> u64 {
        self.row.observed_count()
    }
}

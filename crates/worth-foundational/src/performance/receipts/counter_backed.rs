use crate::performance::basis::{FoundationalPerformanceBundle, FoundationalPerformanceCounterRow};
use crate::performance::claims::FoundationalPerformanceClaimSurface;
use crate::performance::FoundationalPerformanceEvidenceStrength;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalCounterBackedPerformanceReceiptConstructionDenial {
    BundleMustDescribeCounterBackedExecution,
    DuplicateCounterRow,
    MissingCounterRowForSpec,
    UnexpectedCounterRow,
    CounterValueMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalCounterBackedPerformanceReceipt<Claim> {
    bundle: FoundationalPerformanceBundle<Claim>,
    counter_rows: Vec<FoundationalPerformanceCounterRow>,
}

impl<Claim> FoundationalCounterBackedPerformanceReceipt<Claim>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    pub const fn bundle(&self) -> &FoundationalPerformanceBundle<Claim> {
        &self.bundle
    }

    pub fn counter_rows(&self) -> &[FoundationalPerformanceCounterRow] {
        &self.counter_rows
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalCounterBackedPerformanceReceiptBuilder<Claim> {
    bundle: FoundationalPerformanceBundle<Claim>,
    counter_rows: Vec<FoundationalPerformanceCounterRow>,
}

impl<Claim> FoundationalCounterBackedPerformanceReceiptBuilder<Claim>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    pub fn new(bundle: FoundationalPerformanceBundle<Claim>) -> Self {
        Self {
            bundle,
            counter_rows: Vec::new(),
        }
    }

    pub fn attach_counter_row(mut self, counter_row: FoundationalPerformanceCounterRow) -> Self {
        self.counter_rows.push(counter_row);
        self
    }

    pub fn finish(
        mut self,
    ) -> Result<
        FoundationalCounterBackedPerformanceReceipt<Claim>,
        FoundationalCounterBackedPerformanceReceiptConstructionDenial,
    > {
        if self.bundle.claim().evidence_strength()
            != FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt
        {
            return Err(
                FoundationalCounterBackedPerformanceReceiptConstructionDenial::BundleMustDescribeCounterBackedExecution,
            );
        }

        self.counter_rows
            .sort_by(|left, right| left.name().cmp(right.name()));
        if self
            .counter_rows
            .windows(2)
            .any(|window| window[0].name() == window[1].name())
        {
            return Err(
                FoundationalCounterBackedPerformanceReceiptConstructionDenial::DuplicateCounterRow,
            );
        }

        for counter_spec in self.bundle.counter_specs() {
            let Some(counter_row) = self
                .counter_rows
                .iter()
                .find(|counter_row| counter_row.name() == counter_spec.name())
            else {
                return Err(
                    FoundationalCounterBackedPerformanceReceiptConstructionDenial::MissingCounterRowForSpec,
                );
            };
            if counter_row.observed_count() != counter_spec.expected_exact_count() {
                return Err(
                    FoundationalCounterBackedPerformanceReceiptConstructionDenial::CounterValueMismatch,
                );
            }
        }

        if self.counter_rows.iter().any(|counter_row| {
            self.bundle
                .counter_specs()
                .iter()
                .all(|spec| spec.name() != counter_row.name())
        }) {
            return Err(
                FoundationalCounterBackedPerformanceReceiptConstructionDenial::UnexpectedCounterRow,
            );
        }

        Ok(FoundationalCounterBackedPerformanceReceipt {
            bundle: self.bundle,
            counter_rows: self.counter_rows,
        })
    }
}

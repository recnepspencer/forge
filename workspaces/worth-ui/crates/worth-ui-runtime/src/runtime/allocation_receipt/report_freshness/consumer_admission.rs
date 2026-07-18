use super::{UiAllocationReceiptFreshnessPosture, UiAllocationReceiptReport};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationFreshnessConsumptionDenial {
    CoalescingCannotExecute,
    RecomputePending,
    StaleReceiptOutsideAdmittedLag,
}

pub fn admit_host_paint(
    report: &UiAllocationReceiptReport,
) -> Result<(), UiAllocationFreshnessConsumptionDenial> {
    match report.freshness() {
        UiAllocationReceiptFreshnessPosture::Current
        | UiAllocationReceiptFreshnessPosture::Coalescing
        | UiAllocationReceiptFreshnessPosture::StaleButBounded => Ok(()),
        UiAllocationReceiptFreshnessPosture::RecomputePending => {
            Err(UiAllocationFreshnessConsumptionDenial::RecomputePending)
        }
    }
}

pub fn admit_execution_lowering(
    report: &UiAllocationReceiptReport,
) -> Result<(), UiAllocationFreshnessConsumptionDenial> {
    match report.freshness() {
        UiAllocationReceiptFreshnessPosture::Current => Ok(()),
        UiAllocationReceiptFreshnessPosture::StaleButBounded
            if report
                .current_lag()
                .is_some_and(super::UiAllocationReceiptLagBound::is_within_bound) =>
        {
            Ok(())
        }
        UiAllocationReceiptFreshnessPosture::StaleButBounded => {
            Err(UiAllocationFreshnessConsumptionDenial::StaleReceiptOutsideAdmittedLag)
        }
        UiAllocationReceiptFreshnessPosture::Coalescing => {
            Err(UiAllocationFreshnessConsumptionDenial::CoalescingCannotExecute)
        }
        UiAllocationReceiptFreshnessPosture::RecomputePending => {
            Err(UiAllocationFreshnessConsumptionDenial::RecomputePending)
        }
    }
}

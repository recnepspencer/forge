mod counter_backed;

pub use counter_backed::{
    FoundationalCounterBackedPerformanceReceipt,
    FoundationalCounterBackedPerformanceReceiptBuilder,
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
};

use crate::performance::basis::FoundationalPerformanceBundle;
use crate::performance::claims::FoundationalPerformanceClaimSurface;

pub fn counter_backed_performance_receipt<Claim>(
    bundle: FoundationalPerformanceBundle<Claim>,
) -> FoundationalCounterBackedPerformanceReceiptBuilder<Claim>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    FoundationalCounterBackedPerformanceReceiptBuilder::new(bundle)
}

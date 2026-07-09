mod attachments;
mod bundle;
mod canonical;
mod comparison;
mod report_canonical;
mod support;

pub use attachments::{
    FoundationalPerformanceAttachmentConstructionDenial, FoundationalPerformanceContractName,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceSupportingEvidenceCode,
    FoundationalPerformanceSupportingEvidenceRow,
};
pub use bundle::{
    FoundationalPerformanceBundle, FoundationalPerformanceBundleBuilder,
    FoundationalPerformanceBundleConstructionDenial,
};
pub use canonical::{
    foundational_performance_canonical_basis_entries,
    prepare_counter_backed_performance_receipt_for_canonical_basis,
    prepare_performance_bundle_for_canonical_basis,
};
pub use comparison::{
    compare_performance_bundles, FoundationalPerformanceComparison, FoundationalPerformanceMismatch,
};
pub use report_canonical::prepare_materialized_performance_report_for_canonical_basis;
pub use support::performance_basis_rule_version;

use crate::performance::claims::FoundationalPerformanceClaimSurface;

pub fn performance_bundle<Claim>(claim: Claim) -> FoundationalPerformanceBundleBuilder<Claim>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    FoundationalPerformanceBundleBuilder::new(claim)
}

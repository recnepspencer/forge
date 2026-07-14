use worth_store_budgets::PreExecutionBudgetEnvelope;
use worth_store_layout_indexes::{
    layout_degraded_scan_runtime, DegradedExactScanExecutionRequest, ObserveOwnerCase,
};
use worth_store_test_support::{
    admitted_layout_bootstrap_catalog, advanced_admitted_layout_bootstrap_catalog,
    SecurityScopeFixtureAuthority,
};

use super::super::LayoutOwnerObservationLedger;
use super::fixture_values::{page, page_security, segment};

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    let catalog = admitted_layout_bootstrap_catalog();
    let advanced = advanced_admitted_layout_bootstrap_catalog();
    let security = page_security(SecurityScopeFixtureAuthority::Current);

    for current in [&catalog, &advanced] {
        let outcome = layout_degraded_scan_runtime()
            .prepare(
                DegradedExactScanExecutionRequest::new(
                    &catalog,
                    security.witnesses(),
                    segment(7),
                    page(9),
                    8,
                    PreExecutionBudgetEnvelope::terminal_default(),
                )
                .against_current_catalog(current),
            )
            .expect("ordinary degraded scan must reach readiness");
        ledger.record_degraded_scan_readiness(outcome.owner_case_observation());
    }
}

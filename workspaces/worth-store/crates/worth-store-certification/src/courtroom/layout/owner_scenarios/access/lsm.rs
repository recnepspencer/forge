use worth_store_budgets::PreExecutionBudgetEnvelope;
use worth_store_contracts::WalRecordFamily;
use worth_store_layout_indexes::{layout_read_runtime, ObserveOwnerCase, WalLookupRequest};
use worth_store_test_support::{
    admitted_layout_bootstrap_catalog, advanced_admitted_layout_bootstrap_catalog,
    execute_baseline_lsm_persisted_fixture, SecurityScopeFixtureAuthority,
};
use worth_store_wal::StoreWalRecordIdentity;

use super::super::LayoutOwnerObservationLedger;
use super::fixture_values::wal_security;

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    let catalog = admitted_layout_bootstrap_catalog();
    let advanced = advanced_admitted_layout_bootstrap_catalog();
    let security = wal_security(SecurityScopeFixtureAuthority::Current);

    for current in [&catalog, &advanced] {
        let source = execute_baseline_lsm_persisted_fixture().admit_lookup_source();
        let outcome = layout_read_runtime()
            .prepare_wal_lookup(
                wal_request(&catalog, &security, source, 43).against_current_catalog(current),
            )
            .expect("ordinary WAL lookup must reach readiness");
        ledger.record_lsm_lookup_readiness(outcome.owner_case_observation());
    }

    let source = execute_baseline_lsm_persisted_fixture().admit_lookup_source();
    for probe in [43, 42, 41] {
        let outcome = layout_read_runtime()
            .execute_wal_lookup(wal_request(&catalog, &security, source.clone(), probe))
            .expect("ordinary WAL lookup must execute");
        ledger.record_lsm_lookup_execution(outcome.owner_case_observation());
    }
}

fn wal_request<'a>(
    catalog: &'a worth_store_layout_indexes::BootstrapCatalogReadAdmission,
    security: &'a worth_store_security::StoreAdmittedSecurityScope,
    source: worth_store_layout_indexes::BaselineLsmLookupSource,
    probe: u64,
) -> WalLookupRequest<'a> {
    WalLookupRequest::new(
        catalog,
        security.witnesses(),
        WalRecordFamily::DurableMutationIntent,
        StoreWalRecordIdentity::new(probe),
        probe,
        PreExecutionBudgetEnvelope::foreground_default(),
        source,
    )
}

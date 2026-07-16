use std::collections::BTreeSet;

use worth_store_budgets::{PreExecutionBudgetEnvelope, PreExecutionBudgetScope};
use worth_store_contracts::WalRecordFamily;
use worth_store_layout_indexes::{
    layout_lsm_maintenance, LsmCompactionAdmissionRequest, LsmMaintenanceOwnerCaseObservation,
    LsmReplayAdmissionRequest, LsmRunPublicationAdmissionRequest,
};
use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};
use worth_store_test_support::{
    admitted_layout_bootstrap_catalog, execute_baseline_lsm_replay_source_fixture,
    execute_frontierless_lsm_replay_source_fixture, SecurityScopeFixtureAuthority,
};
use worth_store_wal::StoreWalRecordIdentity;

use super::super::fixture_admission::security_scope;
use super::LayoutOwnerObservationLedger;

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    for observation in execute_observations() {
        ledger.record_lsm_maintenance(observation);
    }
}

pub(crate) fn execute_observations() -> Vec<LsmMaintenanceOwnerCaseObservation> {
    let current = wal_security(SecurityScopeFixtureAuthority::Current);
    let foreign = wal_security(SecurityScopeFixtureAuthority::Foreign);
    let page = page_security();
    let budgets = [
        PreExecutionBudgetEnvelope::maintenance_default(),
        zero_budget(),
    ];

    let mut observations = Vec::new();
    let mut publication = BTreeSet::new();
    let mut compaction = BTreeSet::new();
    for security in [&current, &foreign, &page] {
        for budget in budgets {
            retain_once(
                &mut observations,
                &mut publication,
                layout_lsm_maintenance()
                    .admit_run_publication(LsmRunPublicationAdmissionRequest::new(
                        security.witnesses(),
                        WalRecordFamily::DurableMutationIntent,
                        StoreWalRecordIdentity::new(43),
                        budget,
                    ))
                    .owner_case_observation(),
            );
            retain_once(
                &mut observations,
                &mut compaction,
                layout_lsm_maintenance()
                    .admit_compaction(LsmCompactionAdmissionRequest::new(
                        security.witnesses(),
                        WalRecordFamily::DurableMutationIntent,
                        StoreWalRecordIdentity::new(43),
                        budget,
                    ))
                    .owner_case_observation(),
            );
        }
    }

    let catalog = admitted_layout_bootstrap_catalog();
    let replay_sources = [
        execute_baseline_lsm_replay_source_fixture(),
        execute_frontierless_lsm_replay_source_fixture(),
    ];
    let mut replay = BTreeSet::new();
    for security in [&current, &foreign, &page] {
        for source in &replay_sources {
            for budget in budgets {
                retain_once(
                    &mut observations,
                    &mut replay,
                    layout_lsm_maintenance()
                        .admit_replay(LsmReplayAdmissionRequest::new(
                            &catalog,
                            security.witnesses(),
                            WalRecordFamily::DurableMutationIntent,
                            StoreWalRecordIdentity::new(43),
                            source,
                            budget,
                        ))
                        .owner_case_observation(),
                );
            }
        }
    }
    observations
}

fn retain_once(
    observations: &mut Vec<LsmMaintenanceOwnerCaseObservation>,
    seen: &mut BTreeSet<worth_store_layout_indexes::LsmMaintenanceOwnerCaseId>,
    observed: LsmMaintenanceOwnerCaseObservation,
) {
    if seen.insert(observed.id()) {
        observations.push(observed);
    }
}

fn zero_budget() -> PreExecutionBudgetEnvelope {
    PreExecutionBudgetEnvelope::new(PreExecutionBudgetScope::Maintenance, 0, 0, 0, 0, 0)
}

fn wal_security(
    authority: SecurityScopeFixtureAuthority,
) -> worth_store_security::StoreAdmittedSecurityScope {
    security_scope(
        authority,
        StoreKeyScope::WalCheckpointEnvelope,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

fn page_security() -> worth_store_security::StoreAdmittedSecurityScope {
    security_scope(
        SecurityScopeFixtureAuthority::Current,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

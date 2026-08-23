use worth_store_budgets::PreExecutionBudgetEnvelope;
use worth_store_contracts::WalRecordFamily;
use worth_store_layout_indexes::{
    layout_lsm_maintenance, layout_mutation_admission, LsmRunPublicationAdmissionRequest,
    ObserveOwnerCase,
};
use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};
use worth_store_test_support::SecurityScopeFixtureAuthority;
use worth_store_wal::StoreWalRecordIdentity;

use super::super::fixture_admission::security_scope;
use super::LayoutOwnerObservationLedger;

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    let security = security_scope(
        SecurityScopeFixtureAuthority::Current,
        StoreKeyScope::WalCheckpointEnvelope,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let admitted = layout_lsm_maintenance()
        .admit_run_publication(LsmRunPublicationAdmissionRequest::new(
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(43),
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .into_result()
        .expect("ordinary LSM publication must admit");
    ledger.record_layout_mutation_admission(
        layout_mutation_admission()
            .admit_lsm_append(admitted)
            .owner_case_observation(),
    );
    ledger.record_layout_mutation_admission(
        layout_mutation_admission()
            .deny_in_place_reachable_overwrite()
            .owner_case_observation(),
    );
}

use worth_store_budgets::{PreExecutionBudgetEnvelope, PreExecutionBudgetScope};
use worth_store_contracts::{DurableArtifactFamilyId, WalRecordFamily};
use worth_store_layout_indexes::{
    access_planning, access_shapes, AccessLaneClassification, AccessPlanSelector,
    DegradedExactScanRequest, ObserveOwnerCase, PhysicalMutationShape,
};
use worth_store_physical_format::{PhysicalPageId, PhysicalRootReference, PhysicalSegmentId};
use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};
use worth_store_test_support::{admitted_layout_bootstrap_catalog, SecurityScopeFixtureAuthority};

use super::fixture_admission::{admit_family, admit_key_domain, security_scope};
use super::LayoutOwnerObservationLedger;

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    let catalog = admitted_layout_bootstrap_catalog();
    let page_security = security_scope(
        SecurityScopeFixtureAuthority::Current,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let page_family = admit_family(DurableArtifactFamilyId::PhysicalPage, &page_security);
    let page_domain = admit_key_domain(page_family, &page_security);
    let page_materialization = access_planning()
        .admit_current_catalog_root_materialization(page_family, &catalog)
        .expect("catalog root must admit page materialization");
    let wal_security = security_scope(
        SecurityScopeFixtureAuthority::Current,
        StoreKeyScope::WalCheckpointEnvelope,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let wal_family = admit_family(DurableArtifactFamilyId::PublicationWalIntent, &wal_security);
    let wal_domain = admit_key_domain(wal_family, &wal_security);
    let wal_materialization = access_planning()
        .admit_current_catalog_root_materialization(wal_family, &catalog)
        .expect("catalog root must admit WAL materialization");
    let page_key = || {
        worth_store_layout_indexes::declarations::layout_declarations()
            .admit_page_key(
                page_domain,
                PhysicalSegmentId::from_raw(1).unwrap(),
                PhysicalPageId::from_raw(1).unwrap(),
            )
            .unwrap()
    };
    let foreground = PreExecutionBudgetEnvelope::foreground_default();
    let maintenance = PreExecutionBudgetEnvelope::maintenance_default();

    for shape in [
        access_planning().point_access(),
        access_planning().range_access(),
        access_planning().prefix_access(),
    ] {
        let request = AccessPlanSelector
            .admit_read_request(page_family, page_key(), page_materialization.clone(), shape)
            .unwrap();
        let outcome = AccessPlanSelector.select_read_with_budget(request, foreground);
        ledger.record_access_plan_selection(outcome.owner_case_observation());
    }

    let replay_request = AccessPlanSelector
        .admit_recovery_request(
            page_family,
            page_key(),
            page_materialization.clone(),
            access_planning()
                .rebuild_access(AccessLaneClassification::Maintenance)
                .unwrap(),
        )
        .unwrap();
    let outcome = AccessPlanSelector.select_recovery_with_budget(replay_request, maintenance);
    ledger.record_access_plan_selection(outcome.owner_case_observation());

    let cost_request = AccessPlanSelector
        .admit_read_request(
            page_family,
            page_key(),
            page_materialization.clone(),
            access_shapes()
                .explicit_degraded_exact_scan(
                    DegradedExactScanRequest::new().with_budget_rows(u16::MAX as u64 + 1),
                )
                .unwrap(),
        )
        .unwrap();
    let unbounded_terminal = PreExecutionBudgetEnvelope::new(
        PreExecutionBudgetScope::Terminal,
        u64::MAX,
        u16::MAX,
        u16::MAX,
        u16::MAX,
        u64::MAX,
    );
    let outcome = AccessPlanSelector.select_read_with_budget(cost_request, unbounded_terminal);
    ledger.record_access_plan_selection(outcome.owner_case_observation());

    let budget_request = AccessPlanSelector
        .admit_read_request(
            page_family,
            page_key(),
            page_materialization.clone(),
            access_planning().point_access(),
        )
        .unwrap();
    let zero_memory = PreExecutionBudgetEnvelope::new(
        PreExecutionBudgetScope::Foreground,
        0,
        u16::MAX,
        u16::MAX,
        u16::MAX,
        u64::MAX,
    );
    let outcome = AccessPlanSelector.select_read_with_budget(budget_request, zero_memory);
    ledger.record_access_plan_selection(outcome.owner_case_observation());

    execute_lsm_operations(
        ledger,
        wal_family,
        wal_domain,
        &wal_materialization,
        maintenance,
    );
    execute_degraded_and_unsupported(
        ledger,
        page_family,
        page_domain,
        &page_materialization,
        foreground,
        &catalog,
    );
}

fn execute_lsm_operations(
    ledger: &mut LayoutOwnerObservationLedger,
    family: worth_store_layout_indexes::AdmittedPhysicalArtifactFamily,
    domain: worth_store_layout_indexes::AdmittedPhysicalKeyDomain,
    materialization: &worth_store_layout_indexes::AdmittedLayoutMaterialization,
    budget: PreExecutionBudgetEnvelope,
) {
    let key = || {
        worth_store_layout_indexes::declarations::layout_declarations()
            .admit_wal_key(
                domain,
                WalRecordFamily::DurableMutationIntent,
                worth_store_wal::StoreWalRecordIdentity::new(1),
            )
            .unwrap()
    };
    let publication = AccessPlanSelector
        .admit_mutation_request(
            family,
            key(),
            access_shapes()
                .append(PhysicalMutationShape::LogStructuredAppend)
                .unwrap(),
        )
        .unwrap();
    let outcome = AccessPlanSelector.select_mutation_with_budget(publication, budget);
    ledger.record_access_plan_selection(outcome.owner_case_observation());

    let replay = AccessPlanSelector
        .admit_recovery_request(
            family,
            key(),
            materialization.clone(),
            access_planning()
                .rebuild_access(AccessLaneClassification::Maintenance)
                .unwrap(),
        )
        .unwrap();
    let outcome = AccessPlanSelector.select_recovery_with_budget(replay, budget);
    ledger.record_access_plan_selection(outcome.owner_case_observation());

    let compaction = AccessPlanSelector
        .admit_mutation_request(
            family,
            key(),
            access_shapes()
                .compaction_read(PhysicalMutationShape::CompactionRewrite)
                .unwrap(),
        )
        .unwrap();
    let outcome = AccessPlanSelector.select_mutation_with_budget(compaction, budget);
    ledger.record_access_plan_selection(outcome.owner_case_observation());
}

fn execute_degraded_and_unsupported(
    ledger: &mut LayoutOwnerObservationLedger,
    page_family: worth_store_layout_indexes::AdmittedPhysicalArtifactFamily,
    page_domain: worth_store_layout_indexes::AdmittedPhysicalKeyDomain,
    materialization: &worth_store_layout_indexes::AdmittedLayoutMaterialization,
    foreground: PreExecutionBudgetEnvelope,
    catalog: &worth_store_layout_indexes::BootstrapCatalogReadAdmission,
) {
    let page_key = worth_store_layout_indexes::declarations::layout_declarations()
        .admit_page_key(
            page_domain,
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
        )
        .unwrap();
    let degraded = AccessPlanSelector
        .admit_read_request(
            page_family,
            page_key,
            materialization.clone(),
            access_shapes()
                .explicit_degraded_exact_scan(DegradedExactScanRequest::new().with_budget_rows(8))
                .unwrap(),
        )
        .unwrap();
    let outcome = AccessPlanSelector
        .select_read_with_budget(degraded, PreExecutionBudgetEnvelope::terminal_default());
    ledger.record_access_plan_selection(outcome.owner_case_observation());

    let root_security = security_scope(
        SecurityScopeFixtureAuthority::Current,
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let root_family = admit_family(
        DurableArtifactFamilyId::PhysicalRootManifest,
        &root_security,
    );
    let root_domain = admit_key_domain(root_family, &root_security);
    let root_materialization = access_planning()
        .admit_current_catalog_root_materialization(root_family, catalog)
        .unwrap();
    let root_key = worth_store_layout_indexes::declarations::layout_declarations()
        .admit_root_key(root_domain, PhysicalRootReference::from_raw(1).unwrap())
        .unwrap();
    let unsupported = AccessPlanSelector
        .admit_read_request(
            root_family,
            root_key,
            root_materialization,
            access_planning().range_access(),
        )
        .unwrap();
    let outcome = AccessPlanSelector.select_read_with_budget(unsupported, foreground);
    ledger.record_access_plan_selection(outcome.owner_case_observation());
}

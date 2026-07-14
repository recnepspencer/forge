use crate::access::shape::DegradedExactScanRequest;
use crate::facade::{access_planning, deterministic_plan_selection};
use crate::strategy::tests_support::admit_strategy_scope;
use forge_store_budgets::PreExecutionBudgetEnvelope;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};

pub(crate) fn admit_page_scope() -> (
    crate::AdmittedPhysicalArtifactFamily,
    crate::AdmittedPhysicalKeyDomain,
) {
    admit_strategy_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub(super) fn ready_owner_degraded_scan() -> crate::DegradedScanReady {
    let (selected, catalog) = selected_owner_degraded_scan();
    let expected_identity = selected.request_identity();
    let runtime = crate::degraded_scan_runtime();
    let ready = runtime
        .admit_ready(
            runtime.lower(selected),
            crate::CurrentMaterializationFrontier::from_catalog(&catalog),
        )
        .into_ready()
        .expect("current exact materialization must produce degraded readiness");
    assert_eq!(ready.selected().request_identity(), expected_identity);
    ready
}

pub(super) fn rebound_owner_degraded_scan() -> crate::DegradedScanReady {
    let (selected, _) = selected_owner_degraded_scan();
    let expected_identity = selected.request_identity();
    let stale_plan = selected.fingerprint().clone();
    let runtime = crate::degraded_scan_runtime();
    let advanced = crate::bootstrap::test_support::advanced_bootstrap_catalog_read_admission();
    let stale = runtime
        .admit_ready(
            runtime.lower(selected),
            crate::CurrentMaterializationFrontier::from_catalog(&advanced),
        )
        .into_stale()
        .expect("an advanced physical frontier must issue stale degraded evidence");
    let (family, key_domain) = admit_page_scope();
    let replacement = selected_degraded_scan(family, key_domain, &advanced);
    let replacement_plan = replacement.fingerprint().clone();
    let admission = runtime
        .admit_rebind(&stale, &replacement)
        .expect("replacement selected at the observed frontier must admit rebind");
    let ready = runtime
        .rebind(stale, replacement, admission)
        .expect("owner-issued rebind must produce degraded readiness");
    assert_eq!(ready.selected().request_identity(), expected_identity);
    let trace = ready
        .rebind_trace()
        .expect("rebound readiness retains lineage");
    assert_eq!(trace.stale_plan(), &stale_plan);
    assert_eq!(trace.replacement_plan(), &replacement_plan);
    ready
}

pub(super) fn stale_owner_degraded_scan() -> super::StaleDegradedExactScan {
    let (selected, _) = selected_owner_degraded_scan();
    crate::degraded_scan_runtime()
        .admit_ready(
            crate::degraded_scan_runtime().lower(selected),
            crate::CurrentMaterializationFrontier::from_catalog(
                &crate::bootstrap::test_support::advanced_bootstrap_catalog_read_admission(),
            ),
        )
        .into_stale()
        .expect("advanced physical frontier must issue stale degraded evidence")
}

pub(super) fn observed_degraded_readiness_cases() -> [crate::DegradedScanReadinessCaseId; 2] {
    let (current_selected, catalog) = selected_owner_degraded_scan();
    let runtime = crate::degraded_scan_runtime();
    let current = runtime.admit_ready(
        runtime.lower(current_selected),
        crate::CurrentMaterializationFrontier::from_catalog(&catalog),
    );

    let (stale_selected, _) = selected_owner_degraded_scan();
    let stale = runtime.admit_ready(
        runtime.lower(stale_selected),
        crate::CurrentMaterializationFrontier::from_catalog(
            &crate::bootstrap::test_support::advanced_bootstrap_catalog_read_admission(),
        ),
    );
    [current.case_id(), stale.case_id()]
}

pub(super) fn selected_owner_degraded_scan() -> (
    crate::SelectedDegradedExactScan,
    crate::BootstrapCatalogReadAdmission,
) {
    let (lifecycle, key_domain) = admit_page_scope();
    let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    let selected = selected_degraded_scan(lifecycle, key_domain, &catalog);
    (selected, catalog)
}

pub(super) fn selected_degraded_scan(
    lifecycle: crate::AdmittedPhysicalArtifactFamily,
    key_domain: crate::AdmittedPhysicalKeyDomain,
    catalog: &crate::BootstrapCatalogReadAdmission,
) -> crate::SelectedDegradedExactScan {
    let request = crate::access_shapes()
        .explicit_degraded_exact_scan(DegradedExactScanRequest::new().with_budget_rows(8))
        .unwrap();
    deterministic_plan_selection()
        .select_admitted_with_budget(
            crate::planning::AccessPlanSelector
                .admit_read_request(
                    lifecycle,
                    crate::keyspace::admit_page_key(
                        key_domain,
                        forge_store_physical_format::PhysicalSegmentId::from_raw(1).unwrap(),
                        forge_store_physical_format::PhysicalPageId::from_raw(1).unwrap(),
                    )
                    .expect("page identity must pass ordinary key admission"),
                    {
                        access_planning()
                            .admit_current_catalog_root_materialization(lifecycle, catalog)
                            .expect("physical catalog must admit exact root materialization")
                    },
                    request,
                )
                .expect("test request must pass ordinary admission"),
            PreExecutionBudgetEnvelope::terminal_default(),
        )
        .into_degraded()
        .expect("explicit degraded selection must issue the degraded owner capability")
}

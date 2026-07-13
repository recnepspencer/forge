use forge_store_budgets::PreExecutionBudgetEnvelope;
use forge_store_layout_indexes::{layout_degraded_scan_runtime, DegradedExactScanExecutionRequest};
use forge_store_physical_format::{PhysicalPageId, PhysicalSegmentId};
use forge_store_security::admitted_tenant_page_security_scope_for_layout_partition_test;
use forge_store_test_support::{
    admitted_layout_bootstrap_catalog, foreign_layout_physical_store_identity,
    open_layout_physical_facade, open_layout_physical_facade_for_store,
};

#[test]
fn ordinary_degraded_scan_facade_admits_selects_and_executes() {
    let catalog = admitted_layout_bootstrap_catalog();
    let security = admitted_tenant_page_security_scope_for_layout_partition_test();
    let mut physical = open_layout_physical_facade();
    physical
        .publish_physical_root()
        .expect("degraded scan fixture requires a published physical root");

    let execution = layout_degraded_scan_runtime()
        .execute(
            DegradedExactScanExecutionRequest::new(
                &catalog,
                security.witnesses(),
                PhysicalSegmentId::from_raw(7).unwrap(),
                PhysicalPageId::from_raw(3).unwrap(),
                8,
                PreExecutionBudgetEnvelope::terminal_default(),
            ),
            &mut physical,
        )
        .expect("ordinary declarations must execute through the degraded scan owner facade");

    assert_eq!(execution.observed_rows(), 1);
    assert_eq!(
        execution.physical_observation().scan().counters().scans(),
        1
    );
    assert!(execution.physical_observation().scan().is_budget_exact());
    assert_eq!(
        execution
            .physical_observation()
            .scan()
            .request()
            .budget_rows(),
        8
    );
    assert_eq!(
        execution.current_materialization().materialization(),
        execution.selected().materialization().unwrap()
    );
}

#[test]
fn equal_shaped_physical_scan_from_another_store_is_rejected() {
    let catalog = admitted_layout_bootstrap_catalog();
    let security = admitted_tenant_page_security_scope_for_layout_partition_test();
    let foreign_identity = foreign_layout_physical_store_identity();
    let mut physical = open_layout_physical_facade_for_store(&foreign_identity);
    physical.publish_physical_root().unwrap();

    let denial = layout_degraded_scan_runtime()
        .execute(
            DegradedExactScanExecutionRequest::new(
                &catalog,
                security.witnesses(),
                PhysicalSegmentId::from_raw(7).unwrap(),
                PhysicalPageId::from_raw(3).unwrap(),
                8,
                PreExecutionBudgetEnvelope::terminal_default(),
            ),
            &mut physical,
        )
        .unwrap_err();

    assert!(matches!(
        denial,
        forge_store_layout_indexes::DegradedExactScanExecutionDenied::Physical(
            forge_store_layout_indexes::PhysicalDegradedExecutionDenial::StoreAuthorityMismatch { .. }
        )
    ));
}

use worth_store_budgets::{PreExecutionBudgetEnvelope, PreExecutionBudgetScope};
use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_physical_format::{PhysicalPageId, PhysicalSegmentId};
use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};

use super::{AccessPlanSelectionDenied, AccessPlanSelector};
use crate::strategy::tests_support::admit_strategy_scope;

#[test]
fn degraded_scan_cost_rejects_unrepresentable_row_demand_instead_of_clamping() {
    let (family, key_domain) = admit_strategy_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let concrete_key = crate::keyspace::admit_page_key(
        key_domain,
        PhysicalSegmentId::from_raw(1).unwrap(),
        PhysicalPageId::from_raw(1).unwrap(),
    )
    .expect("page identity must pass ordinary key admission");
    let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    let materialization = crate::access_planning()
        .admit_current_catalog_root_materialization(family, &catalog)
        .expect("catalog root must admit exact materialization");
    let degraded = crate::access_shapes()
        .explicit_degraded_exact_scan(
            crate::DegradedExactScanRequest::new().with_budget_rows(u16::MAX as u64 + 1),
        )
        .expect("nonzero degraded row declaration must admit");
    let request = AccessPlanSelector
        .admit_read_request(family, concrete_key, materialization, degraded)
        .expect("degraded request must pass ordinary admission");

    let denial = AccessPlanSelector
        .select_admitted_with_budget(
            request,
            PreExecutionBudgetEnvelope::new(
                PreExecutionBudgetScope::Terminal,
                u64::MAX,
                u16::MAX,
                u16::MAX,
                u16::MAX,
                u64::MAX,
            ),
        )
        .unwrap_err();

    assert_eq!(
        denial,
        AccessPlanSelectionDenied::CostDenied(
            crate::AccessPlanCostDenial::DegradedRowDemandNotRepresentable {
                requested_rows: u16::MAX as u64 + 1,
                maximum: u16::MAX as u64,
            }
        )
    );
}

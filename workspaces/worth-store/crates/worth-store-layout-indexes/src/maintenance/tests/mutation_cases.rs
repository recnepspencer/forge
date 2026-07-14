use std::collections::BTreeSet;

use worth_store_budgets::PreExecutionBudgetEnvelope;
use worth_store_contracts::WalRecordFamily;
use worth_store_security::{
    admitted_store_wal_checkpoint_security_scope_for_layout_partition_test, StoreKeyScope,
    StoreTenantScope,
};
use worth_store_test_support::harness::physical_isolation::publication::{
    admitted_copy_on_write_plan, publication_inputs, publication_inputs_for_store,
    successor_publication_inputs_for_store,
};
use worth_store_wal::StoreWalRecordIdentity;

use crate::maintenance::{
    copy_on_write_layout_mutation_execution, layout_lsm_maintenance, layout_mutation_admission,
    layout_mutation_admission_cases, live_exact_maintenance, CopyOnWriteLayoutMutationRequest,
    ExactBTreePublicationRequest, IndexMaintenanceFailureOutcome, IndexMaintenanceMode,
    LayoutMutationAdmissionView, LiveExactMaintenanceRequest, LsmRunPublicationAdmissionRequest,
    PhysicalMutationShape,
};
use crate::strategy::tests_support::{strategy_test_security_scope, strategy_test_store_identity};

use super::mutation_support::{
    btree_strategy, btree_strategy_with_mode, current_security_scope, source_materialization,
};

#[test]
fn mutation_admission_declares_exactly_the_cases_ordinary_owners_observe() {
    let mut observed = BTreeSet::new();
    let current_security = current_security_scope();

    let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let lsm = layout_lsm_maintenance()
        .admit_run_publication(LsmRunPublicationAdmissionRequest::new(
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(43),
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .into_result()
        .unwrap();
    observed.insert(
        layout_mutation_admission()
            .admit_lsm_append(lsm)
            .case_id()
            .as_str(),
    );

    let strategy = btree_strategy(PhysicalMutationShape::PointRewrite);
    let matching_inputs =
        publication_inputs_for_store(&strategy_test_store_identity(), "layout-cow", 811);
    let matching_materialization = source_materialization(
        strategy.admitted_strategy().admitted_family(),
        &matching_inputs,
    );
    let matching_plan = admitted_copy_on_write_plan(&matching_inputs);
    observed.insert(
        layout_mutation_admission()
            .admit_copy_on_write(CopyOnWriteLayoutMutationRequest::new(
                strategy.clone(),
                matching_plan,
                &matching_materialization,
                current_security.witnesses(),
            ))
            .case_id()
            .as_str(),
    );

    let wrong_shape = btree_strategy(PhysicalMutationShape::ObservationOnly);
    let shape_plan = admitted_copy_on_write_plan(&matching_inputs);
    let shape_denial =
        layout_mutation_admission().admit_copy_on_write(CopyOnWriteLayoutMutationRequest::new(
            wrong_shape,
            shape_plan,
            &matching_materialization,
            current_security.witnesses(),
        ));
    assert!(matches!(
        shape_denial.view(),
        LayoutMutationAdmissionView::Denied(IndexMaintenanceFailureOutcome::MutationShapeMismatch)
    ));
    observed.insert(shape_denial.case_id().as_str());

    let wrong_security = strategy_test_security_scope(
        StoreKeyScope::ArtifactEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
    );
    let security_plan = admitted_copy_on_write_plan(&matching_inputs);
    let security_denial =
        layout_mutation_admission().admit_copy_on_write(CopyOnWriteLayoutMutationRequest::new(
            strategy.clone(),
            security_plan,
            &matching_materialization,
            wrong_security.witnesses(),
        ));
    assert!(matches!(
        security_denial.view(),
        LayoutMutationAdmissionView::Denied(IndexMaintenanceFailureOutcome::SecurityScopeMismatch)
    ));
    observed.insert(security_denial.case_id().as_str());

    let foreign_inputs = publication_inputs();
    let foreign_plan = admitted_copy_on_write_plan(&foreign_inputs);
    let foreign_materialization = source_materialization(
        strategy.admitted_strategy().admitted_family(),
        &foreign_inputs,
    );
    let authority_denial =
        layout_mutation_admission().admit_copy_on_write(CopyOnWriteLayoutMutationRequest::new(
            strategy,
            foreign_plan,
            &foreign_materialization,
            current_security.witnesses(),
        ));
    assert!(matches!(
        authority_denial.view(),
        LayoutMutationAdmissionView::Denied(
            IndexMaintenanceFailureOutcome::PhysicalPublicationAuthorityMismatch
        )
    ));
    observed.insert(authority_denial.case_id().as_str());

    let stale_source = successor_publication_inputs_for_store(
        &matching_inputs,
        &strategy_test_store_identity(),
        "layout-cow-stale-source",
        812,
    );
    let stale_denial =
        layout_mutation_admission().admit_copy_on_write(CopyOnWriteLayoutMutationRequest::new(
            btree_strategy(PhysicalMutationShape::PointRewrite),
            admitted_copy_on_write_plan(&stale_source),
            &matching_materialization,
            current_security.witnesses(),
        ));
    assert!(matches!(
        stale_denial.view(),
        LayoutMutationAdmissionView::Denied(
            IndexMaintenanceFailureOutcome::MutationSourceMaterializationMismatch
        )
    ));
    observed.insert(stale_denial.case_id().as_str());

    observed.insert(
        layout_mutation_admission()
            .deny_in_place_reachable_overwrite()
            .case_id()
            .as_str(),
    );

    let declared = layout_mutation_admission_cases()
        .map(|case| case.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(observed, declared);
}

#[test]
fn copy_on_write_mutation_executes_the_admitted_physical_publication() {
    let strategy = btree_strategy(PhysicalMutationShape::PointRewrite);
    let current_security = current_security_scope();
    let family = strategy.admitted_strategy().admitted_family();
    let inputs =
        publication_inputs_for_store(&strategy_test_store_identity(), "layout-cow-execution", 813);
    let source_materialization = source_materialization(family, &inputs);
    let plan = layout_mutation_admission()
        .admit_copy_on_write(CopyOnWriteLayoutMutationRequest::new(
            strategy,
            admitted_copy_on_write_plan(&inputs),
            &source_materialization,
            current_security.witnesses(),
        ))
        .into_planned()
        .expect("copy-on-write mutation must plan")
        .into_copy_on_write()
        .expect("copy-on-write plan must retain exact operation capability");
    let mut runtime =
        worth_store_physical_isolation::PhysicalRootPublicationRuntime::from_current_root(
            inputs.old_root,
        );
    let receipt = copy_on_write_layout_mutation_execution()
        .execute(&mut runtime, plan)
        .into_published()
        .expect("copy-on-write mutation must publish");

    assert_eq!(receipt.publication().counters().intent_validations(), 1);
    assert_eq!(receipt.publication().counters().readiness_joins(), 1);
    assert_eq!(receipt.publication().counters().root_swaps(), 1);

    let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    let materialization = crate::access_planning()
        .admit_btree_publication_materialization(
            family,
            &catalog,
            receipt.publication().new_root_validation(),
        )
        .into_result()
        .expect("published B-tree root must admit exact materialization");
    let exact = crate::layout_exact_publication()
        .observe_btree(ExactBTreePublicationRequest::new(
            &receipt,
            &materialization,
        ))
        .into_published()
        .expect("the executed root publication must own exact coverage");
    let live = live_exact_maintenance()
        .admit(LiveExactMaintenanceRequest::from_btree_publication(&exact))
        .into_admitted();
    assert_eq!(
        live.publication_protocol(),
        crate::IndexPublicationProtocol::CopyOnWriteRootSwap
    );
}

#[test]
fn non_exact_maintenance_modes_cannot_publish_exact_btree_authority() {
    let current_security = current_security_scope();

    for (index, mode) in [
        IndexMaintenanceMode::AsynchronousLagged,
        IndexMaintenanceMode::RebuildOnly,
        IndexMaintenanceMode::LazyMaterializedOnDemand,
        IndexMaintenanceMode::AdvisoryOnly,
        IndexMaintenanceMode::VerifierOnly,
        IndexMaintenanceMode::MigrationOnly,
    ]
    .into_iter()
    .enumerate()
    {
        let strategy = btree_strategy_with_mode(PhysicalMutationShape::PointRewrite, mode);
        let family = strategy.admitted_strategy().admitted_family();
        let inputs = publication_inputs_for_store(
            &strategy_test_store_identity(),
            &format!("layout-cow-non-exact-{index}"),
            900 + index as u64,
        );
        let source_materialization = source_materialization(family, &inputs);
        let plan = layout_mutation_admission()
            .admit_copy_on_write(CopyOnWriteLayoutMutationRequest::new(
                strategy,
                admitted_copy_on_write_plan(&inputs),
                &source_materialization,
                current_security.witnesses(),
            ))
            .into_planned()
            .expect("ordinary non-exact mutation must still reach its own publication lane")
            .into_copy_on_write()
            .expect("B-tree mutation must retain copy-on-write capability");
        let mut runtime =
            worth_store_physical_isolation::PhysicalRootPublicationRuntime::from_current_root(
                inputs.old_root,
            );
        let receipt = copy_on_write_layout_mutation_execution()
            .execute(&mut runtime, plan)
            .into_published()
            .expect("ordinary non-exact mutation must execute physically");
        let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
        let materialization = crate::access_planning()
            .admit_btree_publication_materialization(
                family,
                &catalog,
                receipt.publication().new_root_validation(),
            )
            .into_result()
            .expect("executed B-tree publication must admit its materialization");

        assert_eq!(
            crate::layout_exact_publication()
                .observe_btree(ExactBTreePublicationRequest::new(
                    &receipt,
                    &materialization
                ))
                .into_published(),
            Err(crate::ExactBTreePublicationDenied::MaintenanceModeIsNotSynchronousExact),
            "{mode:?} crossed into exact publication authority"
        );
    }
}

#[test]
fn copy_on_write_mutation_rejects_cross_tenant_and_cross_key_scope() {
    let strategy = btree_strategy(PhysicalMutationShape::PointRewrite);
    let inputs = publication_inputs_for_store(
        &strategy_test_store_identity(),
        "layout-cow-security-substitution",
        971,
    );
    let source_materialization =
        source_materialization(strategy.admitted_strategy().admitted_family(), &inputs);

    for current_security in [
        strategy_test_security_scope(
            StoreKeyScope::ArtifactEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
        ),
        strategy_test_security_scope(
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::MultiTenantPhysicalBoundary,
        ),
    ] {
        let outcome =
            layout_mutation_admission().admit_copy_on_write(CopyOnWriteLayoutMutationRequest::new(
                strategy.clone(),
                admitted_copy_on_write_plan(&inputs),
                &source_materialization,
                current_security.witnesses(),
            ));
        assert!(matches!(
            outcome.view(),
            LayoutMutationAdmissionView::Denied(
                IndexMaintenanceFailureOutcome::SecurityScopeMismatch
            )
        ));
    }
}

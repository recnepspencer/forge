use worth_store_budgets::PreExecutionBudgetEnvelope;
use worth_store_contracts::{DurableArtifactFamilyId, WalRecordFamily};
use worth_store_layout_indexes::{
    access_planning, copy_on_write_layout_mutation_execution, layout_exact_publication,
    layout_lsm_maintenance, layout_mutation_admission, live_exact_maintenance,
    CopyOnWriteLayoutMutationRequest, ExactBTreePublicationRequest, IndexMaintenanceMode,
    LiveExactMaintenanceRequest, LsmRunPublicationAdmissionRequest, ObserveOwnerCase,
    PhysicalMutationShape,
};
use worth_store_physical_format::PhysicalStoreIdentity;
use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};
use worth_store_test_support::harness::physical_isolation::publication::{
    admitted_copy_on_write_plan, publication_inputs_for_store,
    successor_publication_inputs_for_store, PublicationInputs,
};
use worth_store_test_support::{admitted_layout_bootstrap_catalog, SecurityScopeFixtureAuthority};
use worth_store_wal::StoreWalRecordIdentity;

use super::super::fixture_admission::{admit_family, security_scope};
use super::strategy::{btree_strategy, page_security, wal_security};
use super::LayoutOwnerObservationLedger;

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    execute_mutation_admission(ledger);
    execute_copy_on_write_and_exact_publication(ledger);
}

fn execute_mutation_admission(ledger: &mut LayoutOwnerObservationLedger) {
    let wal = wal_security();
    let lsm = layout_lsm_maintenance()
        .admit_run_publication(LsmRunPublicationAdmissionRequest::new(
            wal.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(43),
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .into_result()
        .expect("ordinary LSM publication must admit");
    let outcome = layout_mutation_admission().admit_lsm_append(lsm);
    ledger.record_layout_mutation_admission(outcome.owner_case_observation());

    let strategy = btree_strategy(
        IndexMaintenanceMode::SynchronousExact,
        PhysicalMutationShape::PointRewrite,
    );
    let matching_inputs = inputs("layout-owner-cow", 811);
    let materialization = source_materialization(
        strategy.admitted_strategy().admitted_family(),
        &matching_inputs,
    );
    record_copy_on_write_admission(
        ledger,
        strategy.clone(),
        &matching_inputs,
        &materialization,
        &page_security(),
    );

    record_copy_on_write_admission(
        ledger,
        btree_strategy(
            IndexMaintenanceMode::SynchronousExact,
            PhysicalMutationShape::ObservationOnly,
        ),
        &matching_inputs,
        &materialization,
        &page_security(),
    );

    let wrong_security = security_scope(
        SecurityScopeFixtureAuthority::Current,
        StoreKeyScope::ArtifactEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    record_copy_on_write_admission(
        ledger,
        strategy.clone(),
        &matching_inputs,
        &materialization,
        &wrong_security,
    );

    let foreign_inputs = publication_inputs_for_store(
        &worth_store_test_support::foreign_layout_physical_store_identity(),
        "layout-owner-cow-foreign",
        812,
    );
    let foreign_materialization = source_materialization(
        strategy.admitted_strategy().admitted_family(),
        &foreign_inputs,
    );
    record_copy_on_write_admission(
        ledger,
        strategy.clone(),
        &foreign_inputs,
        &foreign_materialization,
        &page_security(),
    );

    let stale_inputs = successor_publication_inputs_for_store(
        &matching_inputs,
        &PhysicalStoreIdentity::physical_format_default(),
        "layout-owner-cow-stale",
        813,
    );
    record_copy_on_write_admission(
        ledger,
        strategy,
        &stale_inputs,
        &materialization,
        &page_security(),
    );

    let outcome = layout_mutation_admission().deny_in_place_reachable_overwrite();
    ledger.record_layout_mutation_admission(outcome.owner_case_observation());
}

fn record_copy_on_write_admission(
    ledger: &mut LayoutOwnerObservationLedger,
    strategy: worth_store_layout_indexes::LayoutStrategyRegistrySnapshot,
    inputs: &PublicationInputs,
    materialization: &worth_store_layout_indexes::AdmittedLayoutMaterialization,
    security: &worth_store_security::StoreAdmittedSecurityScope,
) {
    let outcome =
        layout_mutation_admission().admit_copy_on_write(CopyOnWriteLayoutMutationRequest::new(
            strategy,
            admitted_copy_on_write_plan(inputs),
            materialization,
            security.witnesses(),
        ));
    ledger.record_layout_mutation_admission(outcome.owner_case_observation());
}

fn execute_copy_on_write_and_exact_publication(ledger: &mut LayoutOwnerObservationLedger) {
    let exact = executed_btree_mutation(
        IndexMaintenanceMode::SynchronousExact,
        "layout-owner-exact",
        901,
        Some(ledger),
    );
    let catalog = admitted_layout_bootstrap_catalog();
    let exact_materialization = publication_materialization(&catalog, &exact);
    let outcome = layout_exact_publication().observe_btree(ExactBTreePublicationRequest::new(
        &exact,
        &exact_materialization,
    ));
    ledger.record_exact_btree_publication(outcome.owner_case_observation());
    let exact_evidence = outcome
        .into_published()
        .expect("exact publication must publish");
    let live = live_exact_maintenance().admit(LiveExactMaintenanceRequest::from_btree_publication(
        &exact_evidence,
    ));
    ledger.record_live_exact_maintenance(live.owner_case_observation());

    let lagged = executed_btree_mutation(
        IndexMaintenanceMode::AsynchronousLagged,
        "layout-owner-lagged",
        902,
        None,
    );
    let lagged_materialization = publication_materialization(&catalog, &lagged);
    let outcome = layout_exact_publication().observe_btree(ExactBTreePublicationRequest::new(
        &lagged,
        &lagged_materialization,
    ));
    ledger.record_exact_btree_publication(outcome.owner_case_observation());

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
    let root_materialization = access_planning()
        .admit_current_catalog_root_materialization(root_family, &catalog)
        .expect("root family must materialize");
    let outcome = layout_exact_publication().observe_btree(ExactBTreePublicationRequest::new(
        &exact,
        &root_materialization,
    ));
    ledger.record_exact_btree_publication(outcome.owner_case_observation());

    let other = executed_btree_mutation(
        IndexMaintenanceMode::SynchronousExact,
        "layout-owner-other",
        903,
        None,
    );
    let other_materialization = publication_materialization(&catalog, &other);
    let outcome = layout_exact_publication().observe_btree(ExactBTreePublicationRequest::new(
        &exact,
        &other_materialization,
    ));
    ledger.record_exact_btree_publication(outcome.owner_case_observation());
}

fn executed_btree_mutation(
    mode: IndexMaintenanceMode,
    digest: &str,
    generation: u64,
    mut ledger: Option<&mut LayoutOwnerObservationLedger>,
) -> worth_store_layout_indexes::CopyOnWriteLayoutMutationReceipt {
    let strategy = btree_strategy(mode, PhysicalMutationShape::PointRewrite);
    let family = strategy.admitted_strategy().admitted_family();
    let inputs = inputs(digest, generation);
    let materialization = source_materialization(family, &inputs);
    let plan = layout_mutation_admission()
        .admit_copy_on_write(CopyOnWriteLayoutMutationRequest::new(
            strategy,
            admitted_copy_on_write_plan(&inputs),
            &materialization,
            page_security().witnesses(),
        ))
        .into_planned()
        .expect("ordinary mutation must plan")
        .into_copy_on_write()
        .expect("B-tree mutation must retain copy-on-write authority");
    let mut runtime =
        worth_store_physical_isolation::PhysicalRootPublicationRuntime::from_current_root(
            inputs.old_root,
        );
    let first = copy_on_write_layout_mutation_execution().execute(&mut runtime, plan.clone());
    if let Some(ledger) = ledger.as_deref_mut() {
        ledger.record_copy_on_write_mutation_execution(first.owner_case_observation());
    }
    let receipt = first
        .into_published()
        .expect("first publication must succeed");
    let repeated = copy_on_write_layout_mutation_execution().execute(&mut runtime, plan);
    if let Some(ledger) = ledger {
        ledger.record_copy_on_write_mutation_execution(repeated.owner_case_observation());
    }
    receipt
}

fn source_materialization(
    family: worth_store_layout_indexes::AdmittedPhysicalArtifactFamily,
    inputs: &PublicationInputs,
) -> worth_store_layout_indexes::AdmittedLayoutMaterialization {
    access_planning()
        .admit_btree_publication_materialization(
            family,
            &admitted_layout_bootstrap_catalog(),
            inputs.old_candidate.validation(),
        )
        .expect("physical source must materialize")
}

fn publication_materialization(
    catalog: &worth_store_layout_indexes::BootstrapCatalogReadAdmission,
    receipt: &worth_store_layout_indexes::CopyOnWriteLayoutMutationReceipt,
) -> worth_store_layout_indexes::AdmittedLayoutMaterialization {
    access_planning()
        .admit_btree_publication_materialization(
            receipt.admitted_family(),
            catalog,
            receipt.publication().new_root_validation(),
        )
        .expect("published root must materialize")
}

fn inputs(digest: &str, generation: u64) -> PublicationInputs {
    publication_inputs_for_store(
        &PhysicalStoreIdentity::physical_format_default(),
        digest,
        generation,
    )
}

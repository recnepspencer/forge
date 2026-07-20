use worth_store_formal_models::{
    map_import_publication_readiness, map_published_import, ImportPublicationAction,
    ImportPublicationModel, ImportPublicationModelDenial, ImportPublicationState,
};
use worth_store_operations::{admit_import_publication_readiness, complete_import_publication};
use worth_store_physical_format::PhysicalStoreIdentity;

use super::scenario::execute_ordinary_import_publication;

#[test]
fn ordinary_owner_execution_covers_every_import_publication_action() {
    let mut observed = execute_ordinary_import_publication();
    observed.sort_by_key(|action| *action as u8);
    observed.dedup();

    assert_eq!(observed, ImportPublicationAction::all());
}

#[test]
fn real_import_owner_outcomes_map_to_pending_and_durable_actions() {
    let catalog = worth_store_test_support::harness::layout::admitted_layout_bootstrap_catalog();
    let reopened = worth_store_test_support::harness::recovery::reopened_recovery_artifact_fixture(
        "formal-import-publication",
    );
    let preparation = worth_store_operations::certification_test_authority::prepare_import_publication_owner_scenario(
        &catalog,
        &reopened,
    );
    let authority = preparation.authority().clone();
    let store = PhysicalStoreIdentity::from_aspect_identity(authority.identity().clone());
    let inputs = worth_store_test_support::harness::physical_isolation::publication::publication_inputs_for_store(
        &store,
        "formal-import-publication-root",
        93,
    );
    let plan = worth_store_test_support::harness::physical_isolation::publication::admitted_copy_on_write_plan(&inputs);
    let readiness =
        admit_import_publication_readiness(preparation.into_materialization(), &plan, &authority)
            .into_result()
            .unwrap();
    let pending = map_import_publication_readiness(&readiness);
    assert_eq!(
        pending.actions().last(),
        Some(ImportPublicationAction::PublicationPending)
    );

    let publication =
        worth_store_test_support::harness::physical_isolation::publish_in_temporary_store(plan)
            .unwrap();
    let published = complete_import_publication(readiness, publication)
        .into_result()
        .unwrap();
    let durable = map_published_import(&published);
    assert_eq!(durable.physical_binding(), pending.physical_binding());
    assert_eq!(
        durable.actions().last(),
        Some(ImportPublicationAction::PublicationDurable)
    );
}

#[test]
fn crash_before_physical_publication_never_becomes_current() {
    let mut model = publication_pending_model();
    model.crash();

    assert_eq!(model.state(), ImportPublicationState::LayoutMaterialized);
    assert!(model
        .actions()
        .any(|action| action == ImportPublicationAction::CrashBeforePublication));
    assert!(!model
        .actions()
        .any(|action| action == ImportPublicationAction::PublicationDurable));
}

#[test]
fn publication_requires_the_exact_physical_owner_outcome() {
    let mut model = publication_pending_model();

    assert_eq!(
        model.complete_publication(false),
        Err(ImportPublicationModelDenial::ExactPhysicalPublicationRequired)
    );
    assert_eq!(model.state(), ImportPublicationState::PublicationDenied);
}

#[test]
fn raw_or_materialized_values_cannot_skip_owner_frontiers() {
    let mut raw = ImportPublicationModel::from_raw_declaration();
    assert_eq!(
        raw.admit_layout_materialization(),
        Err(ImportPublicationModelDenial::RecoveredArtifactAdmissionRequired)
    );
    assert_eq!(
        raw.admit_publication_readiness(),
        Err(ImportPublicationModelDenial::LayoutMaterializationRequired)
    );
    assert_eq!(
        raw.complete_publication(true),
        Err(ImportPublicationModelDenial::PublicationReadinessRequired)
    );
}

fn publication_pending_model() -> ImportPublicationModel {
    let mut model = ImportPublicationModel::from_raw_declaration();
    model.readmit_current_scope();
    model.admit_recovered_artifact().unwrap();
    model.admit_layout_materialization().unwrap();
    model.admit_publication_readiness().unwrap();
    model
}

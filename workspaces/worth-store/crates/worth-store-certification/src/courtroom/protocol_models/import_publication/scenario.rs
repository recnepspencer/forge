use worth_store_formal_models::{
    map_import_publication_crash_attempt, map_import_publication_denial,
    map_import_publication_readiness, map_published_import, ImportPublicationAction,
};
use worth_store_operations::{admit_import_publication_readiness, complete_import_publication};
use worth_store_physical_backend::{
    ProductionStorageBoundarySeam, ScriptedStorageBoundaryControl, StorageBoundaryFault,
};
use worth_store_physical_format::PhysicalStoreIdentity;

pub(in crate::courtroom::protocol_models) fn execute_ordinary_import_publication(
) -> Vec<ImportPublicationAction> {
    execute_ordinary_import_publication_traces()
        .into_iter()
        .flatten()
        .collect()
}

pub(in crate::courtroom::protocol_models) fn execute_ordinary_import_publication_traces(
) -> Vec<Vec<ImportPublicationAction>> {
    let [crash, denial] = execute_crash_and_denial(93);
    vec![crash, denial, execute_durable_publication()]
}

pub(in crate::courtroom::protocol_models) fn replay_import_publication_guard(
    seed: u64,
) -> Vec<ImportPublicationAction> {
    execute_crash_and_denial(seed.max(1))[0].clone()
}

fn execute_crash_and_denial(generation: u64) -> [Vec<ImportPublicationAction>; 2] {
    let catalog = worth_store_test_support::harness::layout::admitted_layout_bootstrap_catalog();
    let reopened = worth_store_test_support::harness::recovery::reopened_recovery_artifact_fixture(
        "protocol-import-crash-and-denial",
    );
    let preparation = worth_store_operations::certification_test_authority::prepare_import_publication_owner_scenario(
        &catalog,
        &reopened,
    );
    let authority = preparation.authority().clone();
    let store = PhysicalStoreIdentity::from_aspect_identity(authority.identity().clone());
    let inputs = worth_store_test_support::harness::physical_isolation::publication::publication_inputs_for_store(
        &store,
        generation,
    );
    let plan = worth_store_test_support::harness::physical_isolation::publication::admitted_copy_on_write_plan(&inputs);
    let readiness =
        admit_import_publication_readiness(preparation.into_materialization(), &plan, &authority)
            .into_result()
            .unwrap();
    let prefix = map_import_publication_readiness(&readiness)
        .actions()
        .collect::<Vec<_>>();

    let control = ScriptedStorageBoundaryControl::inject(
        ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
        StorageBoundaryFault::AbortBeforeDurabilityBarrier,
    );
    let mut runtime = worth_store_test_support::harness::physical_isolation::PhysicalRootPublicationFixture::open(inputs.old_root).unwrap();
    let attempt = runtime.attempt_with_boundary_control(plan, &control);
    let crash = prefix
        .iter()
        .copied()
        .chain([map_import_publication_crash_attempt(&attempt, &control.trace()).unwrap()])
        .collect();

    let substituted = worth_store_test_support::harness::physical_isolation::publication::publication_inputs_for_store(
        &store,
        generation.saturating_add(1),
    );
    let substituted_publication = worth_store_test_support::harness::physical_isolation::publish_in_temporary_store(
        worth_store_test_support::harness::physical_isolation::publication::admitted_copy_on_write_plan(&substituted),
    )
    .unwrap();
    let denial = complete_import_publication(readiness, substituted_publication)
        .into_result()
        .unwrap_err();
    let denial = prefix
        .into_iter()
        .chain([map_import_publication_denial(&denial)])
        .collect();
    [crash, denial]
}

fn execute_durable_publication() -> Vec<ImportPublicationAction> {
    let catalog = worth_store_test_support::harness::layout::admitted_layout_bootstrap_catalog();
    let reopened = worth_store_test_support::harness::recovery::reopened_recovery_artifact_fixture(
        "protocol-import-durable",
    );
    let preparation = worth_store_operations::certification_test_authority::prepare_import_publication_owner_scenario(
        &catalog,
        &reopened,
    );
    let authority = preparation.authority().clone();
    let store = PhysicalStoreIdentity::from_aspect_identity(authority.identity().clone());
    let inputs = worth_store_test_support::harness::physical_isolation::publication::publication_inputs_for_store(
        &store,
        95,
    );
    let plan = worth_store_test_support::harness::physical_isolation::publication::admitted_copy_on_write_plan(&inputs);
    let readiness =
        admit_import_publication_readiness(preparation.into_materialization(), &plan, &authority)
            .into_result()
            .unwrap();
    let publication =
        worth_store_test_support::harness::physical_isolation::publish_in_temporary_store(plan)
            .unwrap();
    let published = complete_import_publication(readiness, publication)
        .into_result()
        .unwrap();
    map_published_import(&published).actions().collect()
}

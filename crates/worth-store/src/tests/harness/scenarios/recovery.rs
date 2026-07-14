use crate::{modes::SimulatedCrashPoint, DurableMutationRequest, WORTHStore, WORTHStoreBuilder};

use super::super::fixtures::{
    runtime::{create_entity_commit, runtime_with_demo_schema},
    stores::unique_test_sqlite_path,
};

pub fn create_alpha_commit(
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
) -> Result<worth_relational::facade::history::CommitId, crate::StoreError> {
    Ok(create_entity_commit(runtime, "alpha"))
}

pub fn create_beta_commit(
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
) -> Result<worth_relational::facade::history::CommitId, crate::StoreError> {
    Ok(create_entity_commit(runtime, "beta"))
}

pub struct RecoveryScenarioResult {
    pub recovered: crate::modes::DurableStoreHandle,
    pub rebuilt_export_json: String,
}

pub fn recovery_and_rebuild_equivalence() -> RecoveryScenarioResult {
    let path = unique_test_sqlite_path("worth-store-m3-certification");
    let durable_runtime = runtime_with_demo_schema();
    let mut durable = WORTHStoreBuilder::new()
        .sqlite_file(path.clone())
        .durable_mode(durable_runtime)
        .build()
        .unwrap();

    durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .unwrap();
    durable
        .execute_mutation_until_crash(
            DurableMutationRequest::new("create-beta", create_beta_commit),
            SimulatedCrashPoint::AfterCanonicalResultRecorded,
        )
        .unwrap();
    drop(durable);

    let recovered = WORTHStoreBuilder::new()
        .sqlite_file(path)
        .durable_mode(runtime_with_demo_schema())
        .build_pending()
        .unwrap()
        .recover()
        .unwrap();

    let recovered_export = recovered.store().export_authoritative_records();
    let rebuilt =
        WORTHStore::restore_from_authoritative_export(recovered_export.admit_restore()).unwrap();

    RecoveryScenarioResult {
        recovered,
        rebuilt_export_json: rebuilt.export_authoritative_records().canonical_json(),
    }
}

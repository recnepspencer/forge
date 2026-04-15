use crate::{DurableMutationRequest, ForgeStoreBuilder, PublicationWriteOutcome};

use super::super::fixtures::{
    runtime::{create_entity_commit, runtime_with_demo_schema},
    stores::{unique_test_sqlite_path, unique_test_store_path},
};

pub struct DurablePublicationScenarioResult {
    pub local_report: PublicationWriteOutcome,
    pub sqlite_report: PublicationWriteOutcome,
}

pub fn create_alpha_commit(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
) -> Result<forge_relational::facade::history::CommitId, crate::StoreError> {
    Ok(create_entity_commit(runtime, "alpha"))
}

pub fn durable_publication_reports() -> DurablePublicationScenarioResult {
    let mut local = ForgeStoreBuilder::new()
        .local_file(unique_test_store_path("forge-store-publication-local"))
        .durable_mode(runtime_with_demo_schema())
        .build()
        .unwrap();
    let mut sqlite = ForgeStoreBuilder::new()
        .sqlite_file(unique_test_sqlite_path("forge-store-publication-sqlite"))
        .durable_mode(runtime_with_demo_schema())
        .build()
        .unwrap();

    let local_ack = local
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .unwrap();
    let sqlite_ack = sqlite
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .unwrap();

    DurablePublicationScenarioResult {
        local_report: local
            .store()
            .durable_publication_report(
                local_ack.durable_mutation_id(),
                Some(local_ack.persisted().envelope().commit.commit_id),
            )
            .unwrap(),
        sqlite_report: sqlite
            .store()
            .durable_publication_report(
                sqlite_ack.durable_mutation_id(),
                Some(sqlite_ack.persisted().envelope().commit.commit_id),
            )
            .unwrap(),
    }
}

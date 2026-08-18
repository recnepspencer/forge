use super::handles::{HandleBindingError, SupplyChainSemanticHandles};
use super::production_world::ProductionSeededSupplyChainWorld;
use super::program::CompiledSupplyChainProgram;
use std::collections::BTreeMap;
use worth_relational::facade::config::PublicationConfig;
use worth_relational::facade::runtime::{
    RelationalInitialSchemaInstallationDenial, RelationalRuntime, RelationalRuntimeApi,
};
use worth_relational::facade::transactions::{
    BulkEntityCreateIntent, BulkRelationCreateIntent, CreateIntent, MutationIntent,
    TransactionCommitError, WorkerIntentBatch,
};

fn main_options(
    runtime: &RelationalRuntime,
) -> worth_relational::facade::transactions::TransactionOptions {
    let identity = runtime.main_branch_identity();
    runtime
        .transaction_options_for(&identity)
        .expect("configured main branch must remain owner-admissible")
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum SupplyChainCompilationError {
    SchemaInstallation(RelationalInitialSchemaInstallationDenial),
    Transaction(TransactionCommitError),
    HandleBinding(HandleBindingError),
}

const SUPPLY_CHAIN_BASELINE_PATCH_BUDGET: usize = 16_384;

pub(crate) fn compile_supply_chain_baseline(
    program: CompiledSupplyChainProgram,
) -> Result<ProductionSeededSupplyChainWorld, SupplyChainCompilationError> {
    compile_supply_chain_baseline_with_budget(program, SUPPLY_CHAIN_BASELINE_PATCH_BUDGET)
}

pub(crate) fn compile_supply_chain_baseline_with_budget(
    program: CompiledSupplyChainProgram,
    max_patch_records_per_commit: usize,
) -> Result<ProductionSeededSupplyChainWorld, SupplyChainCompilationError> {
    let mut runtime = RelationalRuntimeApi::builder()
        .publication(PublicationConfig {
            coherent_publication_required: true,
            max_patch_records_per_commit,
            max_published_snapshot_handles: 256,
        })
        .build();
    let schema_receipt = runtime
        .prepare_initial_schema_installation()
        .map_err(SupplyChainCompilationError::SchemaInstallation)?
        .install(program.schema_registry().clone())
        .map_err(SupplyChainCompilationError::SchemaInstallation)?;

    let commit_result = commit_definition(&mut runtime, &program)?;
    let snapshot = commit_result.snapshot.clone();
    let handles = SupplyChainSemanticHandles::bind(&program, &commit_result, snapshot)
        .map_err(SupplyChainCompilationError::HandleBinding)?;

    Ok(ProductionSeededSupplyChainWorld {
        runtime,
        program,
        handles,
        commit: commit_result.commit.clone(),
        commit_result,
        schema_receipt,
    })
}

fn commit_definition(
    runtime: &mut RelationalRuntime,
    program: &CompiledSupplyChainProgram,
) -> Result<worth_relational::facade::transactions::CommitResult, SupplyChainCompilationError> {
    if program.entity_specs().is_empty() && program.relation_specs().is_empty() {
        let mut transaction = runtime.begin_transaction(main_options(runtime));
        transaction.push_batch(WorkerIntentBatch::new("supply-chain-empty-baseline"));
        return transaction.commit().map_err(transaction_error);
    }

    let mut batch = WorkerIntentBatch::new("supply-chain-baseline");
    for intent in bulk_entity_intents(program) {
        batch = batch.push(intent);
    }
    for intent in bulk_relation_intents(program) {
        batch = batch.push(intent);
    }

    let mut transaction = runtime.begin_transaction(main_options(runtime));
    transaction.push_batch(batch);
    transaction.commit().map_err(transaction_error)
}

fn bulk_entity_intents(program: &CompiledSupplyChainProgram) -> Vec<MutationIntent> {
    let mut grouped = BTreeMap::<_, BulkEntityCreateIntent>::new();
    for spec in program.all_entity_specs() {
        let entry = grouped
            .entry(spec.kind_id)
            .or_insert_with(|| BulkEntityCreateIntent {
                partition_id: spec.partition_id,
                kind_id: spec.kind_id,
                client_keys: Vec::new(),
                field_patches: Vec::new(),
            });
        entry.client_keys.push(spec.client_key.clone());
        entry.field_patches.push(spec.fields.clone());
    }
    grouped
        .into_values()
        .map(|intent| MutationIntent::Create(CreateIntent::BulkEntities(intent)))
        .collect()
}

fn bulk_relation_intents(program: &CompiledSupplyChainProgram) -> Vec<MutationIntent> {
    let mut grouped = BTreeMap::<_, BulkRelationCreateIntent>::new();
    for spec in program.all_relation_specs() {
        let entry = grouped
            .entry(spec.kind_id)
            .or_insert_with(|| BulkRelationCreateIntent {
                partition_id: spec.partition_id,
                kind_id: spec.kind_id,
                client_keys: Vec::new(),
                endpoints: Vec::new(),
                field_patches: Vec::new(),
            });
        entry.client_keys.push(spec.client_key.clone());
        entry
            .endpoints
            .push((spec.source.clone(), spec.target.clone()));
        entry.field_patches.push(spec.fields.clone());
    }
    grouped
        .into_values()
        .map(|intent| MutationIntent::Create(CreateIntent::BulkRelations(intent)))
        .collect()
}

fn transaction_error(error: TransactionCommitError) -> SupplyChainCompilationError {
    SupplyChainCompilationError::Transaction(error)
}

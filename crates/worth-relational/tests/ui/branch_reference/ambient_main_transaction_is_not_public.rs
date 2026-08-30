use worth_relational::facade::{
    mvcc::RelationalTransactionIntent,
    runtime::RelationalRuntimeApi,
    schema::RelationalSchemaRegistry,
};

fn main() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(RelationalSchemaRegistry::new())
        .build();

    let _ = runtime.begin_transaction(RelationalTransactionIntent::ordinary());
}

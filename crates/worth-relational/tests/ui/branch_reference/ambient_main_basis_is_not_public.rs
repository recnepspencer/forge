use worth_relational::facade::{
    runtime::RelationalRuntimeApi,
    schema::RelationalSchemaRegistry,
};

fn main() {
    let runtime = RelationalRuntimeApi::builder()
        .schema_registry(RelationalSchemaRegistry::new())
        .build();

    let _ = runtime.admit_main_branch_basis();
}

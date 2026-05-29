mod support;

use forge_relational::facade::{
    history::BranchId,
    indexes::{DerivedIndexBuildRequest, DerivedIndexDefinition, DerivedIndexId, DerivedIndexKind},
    runtime::RelationalRuntimeApi,
};

fn main() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(support::demo_schema_registry())
        .build();

    let (created, _entity_id) = support::create_entity(&mut runtime, "indexed");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity.name".to_string(),
        kind: DerivedIndexKind::EntityField {
            field: support::field_key("name"),
        },
        branch_scoped: true,
    });

    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: created.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![index.index_id],
        });

    let indexes = runtime.index_access();
    let generation = indexes
        .latest_generation(index.index_id, &BranchId("main".to_string()))
        .expect("latest generation");

    println!(
        "built_generations={} failed_indexes={} latest_generation={} source_commit={}",
        build.generations.len(),
        build.failed_indexes.len(),
        generation.generation_id.0,
        generation.source_commit_id.0
    );
}

use forge_query::facade::{
    ForgeQueryGraphRelationMutationBuilder, ForgeQuerySymbolicTargetReference,
};

fn main() {
    let reference = ForgeQuerySymbolicTargetReference::new("draft-task")
        .expect("symbolic reference should build");
    let _ = ForgeQueryGraphRelationMutationBuilder::new()
        .symbolic_entity_identity("edge.source_identity", &reference);
}

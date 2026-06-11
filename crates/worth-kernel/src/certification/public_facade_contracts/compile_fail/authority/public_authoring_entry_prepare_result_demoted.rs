use worth_kernel::facade::authoring::construction::{
    author_primitive_construction_declaration, PrimitiveConstructionIntent, WireBodySpec,
};

fn main() {
    let runtime = topology::certification::milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology::runtime_support::topology_runtime(
        topology::runtime_support::TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-authoring-entry-prepare-result-demoted".to_string(),
    )
    .expect("workspace");
    let entry = author_primitive_construction_declaration(
        &workspace,
        PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 8 }),
    )
        .expect("construction declaration");
    let _ = entry.prepare_result();
}

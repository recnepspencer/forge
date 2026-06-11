use topology::certification::milestone_one_runtime_builder;
use topology::runtime_support::{topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::authoring::construction::{
    author_primitive_construction_declaration, PrimitiveConstructionIntent, WireBodySpec,
};

fn main() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.compile-fail.prepare-outcome".to_string(),
    )
    .expect("workspace");
    let entry = author_primitive_construction_declaration(
        &workspace,
        PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 2 }),
    )
        .expect("construction declaration");
    let _ = entry.prepare_outcome();
}

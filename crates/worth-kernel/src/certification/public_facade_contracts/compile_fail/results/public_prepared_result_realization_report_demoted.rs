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
        "worth-kernel.compile-fail.realization-report".to_string(),
    )
    .expect("workspace");
    let prepared = author_primitive_construction_declaration(
        &workspace,
        PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 8 }),
    )
        .and_then(|entry| entry.prepare_result())
        .expect("prepared result");
    let _ = prepared.realization_report();
}

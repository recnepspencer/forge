use worth_kernel::facade::{lower_scaffold_to_topology, PrimitiveConstructionRequest};

fn main() {
    let intent = PrimitiveConstructionRequest::orthotope([0.0, 0.0, 0.0], [1.0, 1.0, 1.0])
        .admit()
        .unwrap();
    let scaffold = intent.build_scaffold().unwrap();
    let (_birth, lowering) = lower_scaffold_to_topology(&scaffold).unwrap();
    let _ = lowering.plan_certification();
}

use worth_kernel::facade::PrimitiveConstructionRequest;

fn main() {
    let intent = PrimitiveConstructionRequest::orthotope([0.0, 0.0, 0.0], [1.0, 1.0, 1.0])
        .admit()
        .unwrap();
    let scaffold = intent.build_scaffold().unwrap();
    let _ = scaffold.lower_to_topology();
}

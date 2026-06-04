use worth_kernel::facade::authoring::construction::PrimitiveConstructionRequest;

fn main() {
    let request = PrimitiveConstructionRequest::orthotope([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let _ = request.admit();
}

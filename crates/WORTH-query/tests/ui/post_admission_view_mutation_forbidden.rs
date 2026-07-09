use worth_query::facade::{AdmittedViewShape, ViewShapeDescriptor};

fn mutate(admitted: &mut AdmittedViewShape) {
    admitted.descriptor = ViewShapeDescriptor::table();
}

fn main() {
    let _ = mutate as fn(&mut AdmittedViewShape);
}

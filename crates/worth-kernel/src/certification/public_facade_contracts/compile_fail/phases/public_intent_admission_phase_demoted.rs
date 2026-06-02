use worth_kernel::facade::{OrthotopeSpec, PrimitiveConstructionIntent};

fn main() {
    let intent = PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
        half_extents: [1.0, 1.0, 1.0],
    });
    let _ = intent.admit();
}

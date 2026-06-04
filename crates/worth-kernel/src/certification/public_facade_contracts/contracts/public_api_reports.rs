use worth_kernel::facade::{authoring::construction::*, diagnostics::rejection::*};

#[test]
fn kernel_public_facade_exports_rejection_reports() {
    let locality = prepare_primitive_construction_rejection_locality_report(vec![
        PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
            half_extents: [1.0, 2.0, 3.0],
        }),
        PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 2 }),
    ]);
    assert_eq!(locality.accepted_count(), 1);
    assert_eq!(locality.rejected_count(), 1);
}

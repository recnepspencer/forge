use worth_ui::facade::{
    WorthUiPrimitiveHostAppearanceObservation, WorthUiPrimitiveProofReceipt,
};

fn main() {
    let proof = proof();
    let _paint_plan = proof.paint_plan(
        1000.0,
        600.0,
        WorthUiPrimitiveHostAppearanceObservation::new(true, true, true),
    );

    panic!("compile-fail fixture only checks paint plan observation boundary");
}

fn proof() -> WorthUiPrimitiveProofReceipt {
    panic!("fixture only checks paint plan observation boundary")
}

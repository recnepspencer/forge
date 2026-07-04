use worth_ui::facade::inspection::{
    UiInspectionObligationDecision, UiInspectionObligationReasonProjection,
};

fn main() {
    let _ = UiInspectionObligationReasonProjection::new(
        1,
        2,
        None,
        None,
        UiInspectionObligationDecision::Admission,
        None,
        None,
        None,
        None,
        Box::new([]),
        Box::new([]),
        None,
        None,
    );
}

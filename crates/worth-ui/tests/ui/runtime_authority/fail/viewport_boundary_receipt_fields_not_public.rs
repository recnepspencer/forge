use worth_ui::facade::{
    WorthUiViewportBoundaryCounters, WorthUiViewportBoundaryReceipt,
    WorthUiViewportDescendantParticipationReceipt,
};

fn main() {
    let _receipt = WorthUiViewportBoundaryReceipt {
        boundaries: Vec::new(),
        consumed_facts: Vec::new(),
        counters: WorthUiViewportBoundaryCounters::default(),
        receipt_digest: 0,
    };

    let _descendant = WorthUiViewportDescendantParticipationReceipt {
        node_id: "child".to_owned(),
        visual_frame: panic!("field construction should be impossible"),
        visible: true,
        hit_participates: true,
        focus_participates: true,
        accessibility_participates: true,
        measurement_participates: true,
        receipt_digest: 0,
    };
}

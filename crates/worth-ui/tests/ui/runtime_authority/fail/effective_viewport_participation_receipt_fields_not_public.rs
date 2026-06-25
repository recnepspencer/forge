use worth_ui::facade::{
    WorthUiEffectiveViewportParticipationCounters, WorthUiEffectiveViewportParticipationReceipt,
    WorthUiEffectiveViewportParticipationRow,
};

fn main() {
    let _receipt = WorthUiEffectiveViewportParticipationReceipt {
        rows: Vec::new(),
        consumed_facts: Vec::new(),
        counters: WorthUiEffectiveViewportParticipationCounters::default(),
        receipt_digest: 0,
    };

    let _row = WorthUiEffectiveViewportParticipationRow {
        node_id: "node".to_owned(),
        visual_frame: panic!("field construction should be impossible"),
        visible: true,
        hit_participates: true,
        focus_participates: true,
        accessibility_participates: true,
        measurement_participates: true,
        governing_boundary_count: 0,
        receipt_digest: 0,
    };
}

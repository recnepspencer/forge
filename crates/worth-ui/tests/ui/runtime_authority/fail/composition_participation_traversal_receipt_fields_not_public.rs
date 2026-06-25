use worth_ui::facade::{
    WorthUiCompositionParticipationTraversalCounters,
    WorthUiCompositionParticipationTraversalReceipt,
};

fn main() {
    let _receipt = WorthUiCompositionParticipationTraversalReceipt {
        rows: Vec::new(),
        counters: WorthUiCompositionParticipationTraversalCounters::default(),
        receipt_digest: 0,
    };
}

use worth_ui::facade::{
    WorthUiCompositionParticipationDenialCounters, WorthUiCompositionParticipationDenialReport,
};

fn main() {
    let _report = WorthUiCompositionParticipationDenialReport {
        denials: Vec::new(),
        counters: WorthUiCompositionParticipationDenialCounters::default(),
        denial_set_digest: 0,
    };
}

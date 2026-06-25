use worth_ui::facade::WorthUiLiveViewParticipationReceipt;

fn main() {
    let _forged = WorthUiLiveViewParticipationReceipt {
        posture: panic!("fixture only checks receipt field privacy"),
        layout: true,
        paint: true,
        events: true,
        accessibility: true,
        retained_state: panic!("fixture only checks receipt field privacy"),
        participation_digest: 1,
    };
}

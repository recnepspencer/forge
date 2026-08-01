use worth_ui::facade::intent::{
    UiIntentOperabilityDecision, UiIntentOperabilityProof, UiPreparedIntentPayload,
};

fn forge(
    candidate: UiPreparedIntentPayload,
    decision: UiIntentOperabilityDecision,
) -> UiIntentOperabilityProof {
    UiIntentOperabilityProof {
        candidate,
        decision,
    }
}

fn main() {}

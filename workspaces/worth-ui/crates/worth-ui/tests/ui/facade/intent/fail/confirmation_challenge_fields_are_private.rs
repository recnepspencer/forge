use worth_ui::facade::intent::UiIntentConfirmationChallenge;

fn forge() -> UiIntentConfirmationChallenge {
    UiIntentConfirmationChallenge {
        candidate: panic!(),
        decision: panic!(),
        policy_identity: panic!(),
        issued_at_millis: 0,
        expires_at_millis: 1,
        lineage: panic!(),
        slot_identity: panic!(),
    }
}

fn main() {}

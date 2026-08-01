use worth_ui::facade::intent::UiIntentConfirmationChallenge;

fn requires_clone<T: Clone>() {}

fn main() {
    requires_clone::<UiIntentConfirmationChallenge>();
}

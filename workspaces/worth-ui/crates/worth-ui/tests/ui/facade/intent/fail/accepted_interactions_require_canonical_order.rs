use worth_ui::facade::intent::{
    UiIntentAcceptedInteractions, UiSemanticInteractionFamily,
};

const INVALID: UiIntentAcceptedInteractions = UiIntentAcceptedInteractions::new(&[
    UiSemanticInteractionFamily::Submit,
    UiSemanticInteractionFamily::Activate,
]);

fn main() {
    let _ = INVALID;
}

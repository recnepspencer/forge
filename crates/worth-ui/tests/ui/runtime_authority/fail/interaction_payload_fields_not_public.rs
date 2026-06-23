use worth_ui::facade::{WorthUiInteractionField, WorthUiInteractionKind, WorthUiInteractionPayload};

fn main() {
    let _forged = WorthUiInteractionPayload {
        kind: WorthUiInteractionKind::Submit,
        fields: Vec::<WorthUiInteractionField>::new(),
        authored_facts_digest: 1,
    };
}

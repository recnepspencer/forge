use worth_ui::facade::{WorthUiInteractionField, WorthUiInteractionFieldValue};

fn main() {
    let _forged = WorthUiInteractionField {
        name: "payload".to_owned(),
        value: WorthUiInteractionFieldValue::Text("forged".to_owned()),
    };
}

use worth_ui::facade::{WorthUiLiveViewStateEditIntent, WorthUiLiveViewStateValue};

fn main() {
    let _forged = WorthUiLiveViewStateEditIntent {
        binding: panic!("fixture only checks edit intent field privacy"),
        value: WorthUiLiveViewStateValue::text("Ada"),
    };
}

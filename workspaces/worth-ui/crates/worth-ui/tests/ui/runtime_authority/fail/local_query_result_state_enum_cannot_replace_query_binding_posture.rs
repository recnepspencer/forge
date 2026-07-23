use worth_ui::facade::WorthUiQueryBindingUiRequirements;

enum LocalQueryResultState {
    Loading,
    Retry,
    Cancelled,
}

fn accepts_query_posture(_posture: WorthUiQueryBindingUiRequirements) {}

fn main() {
    accepts_query_posture(LocalQueryResultState::Loading);
}

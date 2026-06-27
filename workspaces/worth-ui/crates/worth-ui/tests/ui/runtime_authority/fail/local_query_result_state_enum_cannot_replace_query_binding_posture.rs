use worth_ui::facade::WorthUiQueryBindingPosture;

enum LocalQueryResultState {
    Loading,
    Retry,
    Cancelled,
}

fn accepts_query_posture(_posture: WorthUiQueryBindingPosture) {}

fn main() {
    accepts_query_posture(LocalQueryResultState::Loading);
}

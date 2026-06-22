use worth_ui::facade::WorthUiQueryInspectionLinks;

struct LocalQueryExplanationRecord {
    explanation: String,
}

fn main() {
    let local = LocalQueryExplanationRecord {
        explanation: "looks query-ish".to_owned(),
    };
    let _links = WorthUiQueryInspectionLinks::from_local_explanation(local.explanation);
}

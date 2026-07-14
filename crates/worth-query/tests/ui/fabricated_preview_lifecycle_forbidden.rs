use worth_query::facade::policy::{PreviewEvaluationClass, PreviewSessionQueryContext};

fn main() {
    let _ = PreviewSessionQueryContext {
        source: todo!(),
        evaluation_class: PreviewEvaluationClass::read_only(),
        replay_bundle: None,
        promotion_record: None,
    };
}

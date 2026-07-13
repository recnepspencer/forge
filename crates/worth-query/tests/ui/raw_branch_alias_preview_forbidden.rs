use worth_query::facade::policy::{PreviewEvaluationClass, PreviewSessionQueryContext};

fn main() {
    let _ = PreviewSessionQueryContext::declared("main", PreviewEvaluationClass::read_only());
}

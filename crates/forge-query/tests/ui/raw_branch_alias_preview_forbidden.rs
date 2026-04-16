use forge_query::facade::{PreviewEvaluationClass, PreviewSessionQueryContext};

fn main() {
    let _ = PreviewSessionQueryContext::declared("main", PreviewEvaluationClass::read_only());
}

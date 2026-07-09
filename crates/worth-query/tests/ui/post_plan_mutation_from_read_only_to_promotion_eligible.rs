use worth_query::facade::{PreviewEvaluationClass, PreviewSessionPlanBinding};

fn main() {
    let mut binding: PreviewSessionPlanBinding = todo!();
    binding.query_context = todo!();
    let _ = PreviewEvaluationClass::promotion_eligible();
}

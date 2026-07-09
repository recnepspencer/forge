use worth_query::facade::{
    execute_promotion_eligible_preview_session_plan, PreviewSessionPlanBinding,
};

fn main() {
    let binding: PreviewSessionPlanBinding = todo!();
    let _ = execute_promotion_eligible_preview_session_plan(&binding);
}

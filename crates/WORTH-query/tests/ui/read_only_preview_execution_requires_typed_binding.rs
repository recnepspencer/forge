use worth_query::facade::{
    execute_read_only_preview_session_plan, PreviewSessionPlanBinding,
};

fn main() {
    let binding: PreviewSessionPlanBinding = todo!();
    let _ = execute_read_only_preview_session_plan(&binding);
}

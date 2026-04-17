use forge_query::facade::{execute_preview_session_plan, PreviewExecutionEnvelope};

fn main() {
    let _ = execute_preview_session_plan;
    let _ = core::mem::size_of::<PreviewExecutionEnvelope>();
}

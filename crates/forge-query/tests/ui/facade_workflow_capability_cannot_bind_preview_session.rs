use forge_query::facade::{ForgeQueryApplicationFacade, PreviewSessionQueryContext};

fn main() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let workflow = facade.workflow_query_capability().unwrap();
    let preflight: forge_query::facade::ExecutionPreflightBundle = todo!();
    let context: PreviewSessionQueryContext = todo!();

    let _ = workflow.capability().bind_preflight(preflight, context);
}

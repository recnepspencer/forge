use worth_query::facade::foundation::WorthQueryApplicationFacade;
use worth_query::facade::policy::PreviewSessionQueryContext;

fn main() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let workflow = facade.workflow_query_capability().unwrap();
    let preflight: worth_query::facade::foundation::ExecutionPreflightBundle = todo!();
    let context: PreviewSessionQueryContext = todo!();

    let _ = workflow.capability().bind_preflight(preflight, context);
}

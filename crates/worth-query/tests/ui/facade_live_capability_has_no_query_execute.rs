use worth_query::facade::foundation::WorthQueryApplicationFacade;

fn main() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let live = facade.live_query_capability().unwrap();
    let preflight: worth_query::facade::foundation::ExecutionPreflightBundle = todo!();

    let _ = live.capability().execute_preflight(&preflight);
}

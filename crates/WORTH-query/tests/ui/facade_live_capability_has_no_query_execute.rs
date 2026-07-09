use worth_query::facade::WorthQueryApplicationFacade;

fn main() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let live = facade.live_query_capability().unwrap();
    let preflight: worth_query::facade::ExecutionPreflightBundle = todo!();

    let _ = live.capability().execute_preflight(&preflight);
}

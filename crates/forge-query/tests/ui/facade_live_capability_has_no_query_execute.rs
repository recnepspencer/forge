use forge_query::facade::ForgeQueryApplicationFacade;

fn main() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let live = facade.live_query_capability().unwrap();
    let preflight: forge_query::facade::ExecutionPreflightBundle = todo!();

    let _ = live.capability().execute_preflight(&preflight);
}

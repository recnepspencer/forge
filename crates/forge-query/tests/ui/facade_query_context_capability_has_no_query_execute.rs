use forge_query::facade::ForgeQueryApplicationFacade;

fn main() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let contexts = facade.query_context_capability().unwrap();
    let _ = contexts.capability().execute_preflight;
}

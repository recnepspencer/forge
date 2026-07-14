use worth_query::facade::foundation::WorthQueryApplicationFacade;

fn main() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let contexts = facade.query_context_capability().unwrap();
    let _ = contexts.capability().execute_preflight;
}

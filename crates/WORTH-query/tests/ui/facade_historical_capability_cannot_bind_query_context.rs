use worth_query::facade::WorthQueryApplicationFacade;

fn main() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let historical = facade.historical_query_capability().unwrap();
    let _ = historical.capability().bind_basis_context;
}

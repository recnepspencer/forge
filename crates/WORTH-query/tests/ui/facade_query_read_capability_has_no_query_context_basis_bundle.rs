use worth_query::facade::WorthQueryApplicationFacade;

fn main() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let reads = facade.query_read_capability().unwrap();
    let _ = reads.capability().execute_basis_result_bundle;
}

use forge_query::facade::ForgeQueryApplicationFacade;

fn main() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let reads = facade.query_read_capability().unwrap();
    let _ = reads.capability().execute_basis_result_bundle;
}

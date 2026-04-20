use forge_query::facade::ForgeQueryApplicationFacade;

fn main() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let historical = facade.historical_query_capability().unwrap();
    let _ = historical.capability().shape_diff_result_bundle;
}

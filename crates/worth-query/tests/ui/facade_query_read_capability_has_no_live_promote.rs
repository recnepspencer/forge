use worth_query::facade::WorthQueryApplicationFacade;

fn main() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let query_reads = facade.query_read_capability().unwrap();
    let preflight: worth_query::facade::ExecutionPreflightBundle = todo!();

    let _ = query_reads.capability().promote_preflight(&preflight);
}

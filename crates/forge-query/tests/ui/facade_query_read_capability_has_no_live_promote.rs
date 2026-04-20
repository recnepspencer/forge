use forge_query::facade::ForgeQueryApplicationFacade;

fn main() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let query_reads = facade.query_read_capability().unwrap();
    let preflight: forge_query::facade::ExecutionPreflightBundle = todo!();

    let _ = query_reads.capability().promote_preflight(&preflight);
}

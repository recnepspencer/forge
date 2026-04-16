use forge_query::facade::{
    execute_parallel_admission_route, ParallelAdmissionRoute, SerialFallbackRoute,
};

fn main() {
    let route: SerialFallbackRoute = todo!();
    let _parallel: ParallelAdmissionRoute = todo!();
    let _ = execute_parallel_admission_route(&route);
}

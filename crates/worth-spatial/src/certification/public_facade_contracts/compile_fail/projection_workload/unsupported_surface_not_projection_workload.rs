use worth_spatial::facade::projection_workload::ProjectionWorkload;
use worth_spatial::facade::surface_support::UnsupportedSurfaceSupport;

fn main() {
    let unsupported = unconstructible::<UnsupportedSurfaceSupport>();
    let _ = ProjectionWorkload::for_certified_surface_support(unsupported);
}

fn unconstructible<T>() -> T {
    panic!("compile-fail input is never executed")
}

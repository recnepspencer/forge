use worth_spatial::facade::projection_workload::ProjectedPlanarWorkload;

fn main() {
    let _ = ProjectedPlanarWorkload::new(
        unconstructible(),
        Vec::new(),
        unconstructible(),
        Vec::new(),
        unconstructible(),
    );
}

fn unconstructible<T>() -> T {
    panic!("compile-fail input is never executed")
}

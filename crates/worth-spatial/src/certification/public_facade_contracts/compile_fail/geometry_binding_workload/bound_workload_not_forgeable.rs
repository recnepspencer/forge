use worth_spatial::facade::workload_binding::BoundGeometryWorkload;

fn main() {
    let _ = BoundGeometryWorkload::new(
        unconstructible(),
        unconstructible(),
        unconstructible(),
        unconstructible(),
    );
}

fn unconstructible<T>() -> T {
    panic!("compile-fail input is never executed")
}

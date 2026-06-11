use worth_spatial::facade::transform_workload::TransformedWorkload;

fn main() {
    let _ = TransformedWorkload::new(
        Vec::new(),
        Vec::new(),
        Vec::new(),
        unconstructible(),
        unconstructible(),
        unconstructible(),
    );
}

fn unconstructible<T>() -> T {
    panic!("compile-fail input is never executed")
}

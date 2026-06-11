use worth_spatial::facade::user_response::WorthUserOutcome;

fn main() {
    let _ = WorthUserOutcome::admitted(unconstructible(), unconstructible());
}

fn unconstructible<T>() -> T {
    panic!("compile-fail input is never executed")
}

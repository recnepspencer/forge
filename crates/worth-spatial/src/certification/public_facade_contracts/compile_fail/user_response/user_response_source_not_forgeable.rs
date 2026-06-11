use worth_spatial::facade::user_response::WorthUserResponseSource;

fn main() {
    let _ = WorthUserResponseSource {
        kind: unconstructible(),
    };
}

fn unconstructible<T>() -> T {
    panic!("compile-fail input is never executed")
}

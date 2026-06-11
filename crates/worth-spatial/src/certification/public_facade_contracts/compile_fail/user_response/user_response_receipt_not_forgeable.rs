use worth_spatial::facade::user_response::WorthUserResponseReceipt;

fn main() {
    let _ = WorthUserResponseReceipt {
        stage_receipt: unconstructible(),
        outcome: unconstructible(),
    };
}

fn unconstructible<T>() -> T {
    panic!("compile-fail input is never executed")
}

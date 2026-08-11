use worth_store_recovery_runtime::RecoveryCleanupEligibility;

fn main() {
    let _forged = RecoveryCleanupEligibility {
        artifact: fake(),
        range: fake(),
        byte_count: 1,
    };
}

fn fake<T>() -> T {
    panic!("compile-only forged value")
}

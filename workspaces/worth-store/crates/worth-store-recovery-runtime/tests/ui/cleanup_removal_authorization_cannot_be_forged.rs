use worth_store::physical_runtime::PhysicalRecoveryCleanupRemovalCommand;

fn fake<T>() -> T {
    panic!("compile-only specimen")
}

fn main() {
    let _forged = PhysicalRecoveryCleanupRemovalCommand::admit(
        fake(),
        fake(),
        [1; 32],
        fake(),
        fake(),
        fake(),
    );
}

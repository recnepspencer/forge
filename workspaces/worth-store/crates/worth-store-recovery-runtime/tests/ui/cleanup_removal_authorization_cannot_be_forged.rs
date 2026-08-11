use worth_store::physical_runtime::PhysicalRecoveryCleanupAuthorization;

fn fake<T>() -> T {
    panic!("compile-only specimen")
}

fn main() {
    let _forged = PhysicalRecoveryCleanupAuthorization {
        store: fake(),
        media_generation: fake(),
        session: [0; 16],
        observed_published_generation: 1,
        cleanup_plan_identity: [1; 32],
        sealed_publication_basis: [2; 32],
        policy_identity: [3; 32],
    };
}

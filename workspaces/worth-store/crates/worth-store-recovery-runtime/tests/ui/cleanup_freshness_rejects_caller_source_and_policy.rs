use worth_store::physical_runtime::PhysicalRecoveryFreshnessPort;

fn substitute_source_generation_and_policy() {
    let _ = PhysicalRecoveryFreshnessPort::sample_cleanup(
        7_u64,
        [0_u8; 32],
        [1_u8; 32],
        [2_u8; 32],
        [3_u8; 32],
    );
}

fn main() {}

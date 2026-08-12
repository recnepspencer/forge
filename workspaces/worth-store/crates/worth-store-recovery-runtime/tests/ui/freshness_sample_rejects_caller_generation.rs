use worth_store::physical_runtime::PhysicalRecoveryFreshnessPort;

fn substitute_generation_and_policy() {
    let _ = PhysicalRecoveryFreshnessPort::sample_binding(7_u64, [0_u8; 32]);
}

fn main() {}

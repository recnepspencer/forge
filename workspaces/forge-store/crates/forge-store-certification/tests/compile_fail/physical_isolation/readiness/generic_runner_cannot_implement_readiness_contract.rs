struct GenericRunner;

impl forge_store_physical_isolation::PhysicalIsolationHarnessReadinessContract for GenericRunner {
    fn does_not_claim_physical_isolation_correctness(&self) -> bool {
        true
    }
}

fn main() {}

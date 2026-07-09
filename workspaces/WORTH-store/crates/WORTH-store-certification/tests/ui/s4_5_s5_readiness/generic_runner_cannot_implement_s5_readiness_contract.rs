struct GenericRunner;

impl worth_store_physical_isolation::S5SimulationHarnessReadinessContract for GenericRunner {
    fn does_not_claim_s5_correctness(&self) -> bool {
        true
    }
}

fn main() {}

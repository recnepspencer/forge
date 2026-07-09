use worth_store_physical_certification::S5SimulationHarnessReadiness;
use worth_store_readiness::S5CorrectnessNonClaimEvidence;

fn main() {
    let _readiness = S5SimulationHarnessReadiness::from_generated_maturity(
        Vec::new(),
        S5CorrectnessNonClaimEvidence::shape_probe_only(),
    );
}

use forge_store_physical_certification::S5SimulationHarnessReadiness;
use forge_store_readiness::S5CorrectnessNonClaimEvidence;

fn main() {
    let _readiness = S5SimulationHarnessReadiness {
        dependencies: Vec::new(),
        non_claim: S5CorrectnessNonClaimEvidence::shape_probe_only(),
    };
}

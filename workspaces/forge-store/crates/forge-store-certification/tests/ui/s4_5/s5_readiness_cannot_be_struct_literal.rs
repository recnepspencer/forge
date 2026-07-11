use forge_store_physical_certification::PhysicalIsolationHarnessReadiness;
use forge_store_readiness::PhysicalIsolationCorrectnessNonClaimEvidence;

fn main() {
    let _readiness = PhysicalIsolationHarnessReadiness {
        dependencies: Vec::new(),
        non_claim: PhysicalIsolationCorrectnessNonClaimEvidence::shape_probe_only(),
    };
}

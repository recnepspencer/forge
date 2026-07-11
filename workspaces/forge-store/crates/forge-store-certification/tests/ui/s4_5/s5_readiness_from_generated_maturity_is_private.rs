use forge_store_physical_certification::PhysicalIsolationHarnessReadiness;
use forge_store_readiness::PhysicalIsolationCorrectnessNonClaimEvidence;

fn main() {
    let _readiness = PhysicalIsolationHarnessReadiness::from_generated_maturity(
        Vec::new(),
        PhysicalIsolationCorrectnessNonClaimEvidence::shape_probe_only(),
    );
}

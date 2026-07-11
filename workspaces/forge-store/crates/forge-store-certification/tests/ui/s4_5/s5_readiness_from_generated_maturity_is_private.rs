use forge_store_physical_certification::{
    PhysicalIsolationCorrectnessNonClaimEvidence, PhysicalIsolationHarnessReadiness,
};

fn main() {
    let _readiness = PhysicalIsolationHarnessReadiness::from_generated_maturity(
        Vec::new(),
        PhysicalIsolationCorrectnessNonClaimEvidence::shape_probe_only(),
    );
}

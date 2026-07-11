use forge_store_physical_certification::{
    PhysicalIsolationCorrectnessNonClaimEvidence, PhysicalIsolationHarnessReadiness,
};

fn main() {
    let _readiness = PhysicalIsolationHarnessReadiness {
        dependencies: Vec::new(),
        non_claim: PhysicalIsolationCorrectnessNonClaimEvidence::shape_probe_only(),
    };
}

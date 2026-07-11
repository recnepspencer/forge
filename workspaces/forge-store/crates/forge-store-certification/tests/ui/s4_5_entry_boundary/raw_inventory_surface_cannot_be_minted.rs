use forge_store_physical_certification::{
    ExistingSimulationHarnessSurface, SimulationHarnessSurfaceClassification,
};

fn main() {
    let _ = ExistingSimulationHarnessSurface::new(
        "forge-store-test-support::pretend_certification_meaning",
        SimulationHarnessSurfaceClassification::CertificationMeaning,
    );
}

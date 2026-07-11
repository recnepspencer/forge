use forge_store_physical_certification::{DriverCapabilityProfile, PhysicalSimulationDriver};

fn main() {
    let _forged = PhysicalSimulationDriver {
        profile: DriverCapabilityProfile::memory_pressure_boundary(),
        yieldpoints: Vec::new(),
        backend_profile: None,
    };
}

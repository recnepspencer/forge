use worth_store_physical_certification::{DriverCapabilityProfile, PhysicalSimulationDriver};

fn main() {
    let _WORTHd = PhysicalSimulationDriver {
        profile: DriverCapabilityProfile::memory_pressure_boundary(),
        yieldpoints: Vec::new(),
        backend_profile: None,
    };
}

use forge_store_physical_certification::{
    accept_store_owned_s5_harness_readiness, PhysicalIsolationHarnessFutureExtensionReservation,
    PhysicalIsolationHarnessFutureExtensionSlot,
};
use forge_store_physical_isolation::s5_simulation_harness_readiness_requirement;

fn main() {
    let slot = PhysicalIsolationHarnessFutureExtensionReservation::reserved(
        PhysicalIsolationHarnessFutureExtensionSlot::FullS12Campaign,
    );
    let _accepted = accept_store_owned_s5_harness_readiness(
        slot,
        s5_simulation_harness_readiness_requirement(),
    );
}

use worth_store_physical_certification::{
    accept_store_owned_s5_harness_readiness, S5HarnessFutureExtensionReservation,
    S5HarnessFutureExtensionSlot,
};
use worth_store_physical_isolation::s5_simulation_harness_readiness_requirement;

fn main() {
    let slot = S5HarnessFutureExtensionReservation::reserved(
        S5HarnessFutureExtensionSlot::FullS12Campaign,
    );
    let _accepted = accept_store_owned_s5_harness_readiness(
        slot,
        s5_simulation_harness_readiness_requirement(),
    );
}

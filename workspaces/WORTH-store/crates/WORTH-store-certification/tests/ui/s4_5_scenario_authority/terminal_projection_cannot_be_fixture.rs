use worth_store_aspect_native::StoreTerminalProjectionText;
use worth_store_physical_certification::{
    physical_scenario, PhysicalScenarioIntent, PhysicalSimulationScenarioFamily,
};

fn main() {
    let terminal_projection =
        StoreTerminalProjectionText::new_terminal_projection_text("terminal output");
    let _scenario = physical_scenario("store.physical.s5.readiness")
        .family(PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(terminal_projection);
}

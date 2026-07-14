use worth_store_physical_certification::{
    physical_scenario, PhysicalScenarioIntent, PhysicalSimulationScenarioFamily,
};

fn main() {
    let _scenario = physical_scenario("store.physical.s5.readiness")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture("segment-header-alpha");
}

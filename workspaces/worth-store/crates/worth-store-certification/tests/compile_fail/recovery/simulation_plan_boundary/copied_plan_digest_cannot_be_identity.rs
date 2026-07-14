use worth_store_physical_certification::PhysicalSimulationPlanIdentity;

fn requires_plan_identity(_: PhysicalSimulationPlanIdentity) {}

fn main() {
    requires_plan_identity("copied-plan-digest");
}

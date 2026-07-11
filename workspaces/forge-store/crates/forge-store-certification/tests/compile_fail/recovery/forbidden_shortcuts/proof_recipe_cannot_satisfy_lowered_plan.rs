use forge_proof::prelude::*;
use forge_store_physical_certification::PhysicalSimulationPlan;

fn requires_lowered_plan(_: PhysicalSimulationPlan) {}

fn main() {
    requires_lowered_plan(recipe("shortcut-shaped-plan"));
}

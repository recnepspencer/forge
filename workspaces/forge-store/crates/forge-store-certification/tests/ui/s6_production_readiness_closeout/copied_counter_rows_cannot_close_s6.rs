use forge_store_certification::S6ProductionReadinessClosureInput;
use forge_store_readiness::S6ReadinessCertificationCounterEvidence;

fn main() {
    let counters: Vec<S6ReadinessCertificationCounterEvidence> = Vec::new();
    let _ = S6ProductionReadinessClosureInput::from_phase13_adoption(counters);
}

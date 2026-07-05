use forge_store_certification::S6ProductionReadinessClosureInput;
use forge_store_readiness::S6ReadinessCertificationProofSummary;

fn main() {
    let proof: S6ReadinessCertificationProofSummary = todo!();
    let _ = S6ProductionReadinessClosureInput::from_phase13_adoption(proof);
}

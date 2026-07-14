use worth_store_layout_indexes::S8ExecutedAccessEvidence;
use worth_store_physical_certification::layout_harness::transcripts::S8LayoutExecutedScenarioWitness;

fn require_executed(_: S8ExecutedAccessEvidence) {}

fn main() {
    let harness: S8LayoutExecutedScenarioWitness = todo!();
    require_executed(harness);
}

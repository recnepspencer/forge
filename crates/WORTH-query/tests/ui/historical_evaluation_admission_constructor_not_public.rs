use worth_query::facade::{
    HistoricalEvaluationAdmission, HistoricalPathCompatibilityOutcome, HistoricalPathCostPosture,
};

fn main() {
    let _ = HistoricalEvaluationAdmission {
        requested_path: todo!(),
        admitted_path: todo!(),
        compatibility_outcome: HistoricalPathCompatibilityOutcome::Admitted,
        cost_posture: HistoricalPathCostPosture::HistoricalRetainedFastPath,
        replay_budget: todo!(),
        reconstruction_budget: todo!(),
        reuse_descriptor: todo!(),
        complexity_contract: todo!(),
        counters: todo!(),
    };
}

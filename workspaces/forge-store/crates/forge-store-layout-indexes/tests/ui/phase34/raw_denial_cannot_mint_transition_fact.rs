use forge_store_layout_indexes::access_planning::S8PlanSelectionDenied;
use forge_store_layout_indexes::layout_strategy_admission::S8StrategyDenial;

fn main() {
    let _planning = S8PlanSelectionDenied::NoEligibleAlternative.production_transition();
    let _strategy = S8StrategyDenial::UnsupportedFamily.production_transition();
}

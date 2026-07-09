use worth_store::{
    AuthoritativeTierMovePlan, PlacementBudgetClass, PlacementExecutionOrigin,
    TierResidenceClass,
};

fn main() {
    let _ = AuthoritativeTierMovePlan::new(
        "authoritative_branch_head:main",
        TierResidenceClass::Hot,
        PlacementBudgetClass::BackgroundOnly,
        PlacementExecutionOrigin::Background,
    );
}

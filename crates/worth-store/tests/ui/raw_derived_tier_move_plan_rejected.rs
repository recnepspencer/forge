use worth_store::{
    DerivedTierMovePlan, PlacementArtifactFamily, PlacementBudgetClass,
    PlacementExecutionOrigin, TierResidenceClass,
};

fn main() {
    let _ = DerivedTierMovePlan::new(
        PlacementArtifactFamily::SnapshotFamily,
        "42",
        TierResidenceClass::Cold,
        PlacementBudgetClass::BackgroundOnly,
        PlacementExecutionOrigin::Background,
    );
}

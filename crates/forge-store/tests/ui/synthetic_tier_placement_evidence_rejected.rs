use forge_store::{
    PlacementBudgetClass, PlacementExecutionOrigin, TierPlacementEvidence, TierResidenceClass,
};

fn main() {
    let _ = TierPlacementEvidence::new(
        TierResidenceClass::Warm,
        PlacementBudgetClass::ForegroundResidentOnly,
        PlacementExecutionOrigin::Foreground,
    );
}

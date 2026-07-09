use worth_store::{
    PlacementBoundArtifactRef, PlacementBudgetClass, PlacementExecutionOrigin, ResidentReadLease,
    TierResidenceClass,
};

fn main() {
    let artifact_ref = PlacementBoundArtifactRef::authoritative_branch_head("main");
    let _ = ResidentReadLease::new(
        artifact_ref,
        TierResidenceClass::Hot,
        PlacementBudgetClass::ForegroundResidentOnly,
        PlacementExecutionOrigin::Foreground,
    );
}

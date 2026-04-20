use forge_store::{
    ColdRecallLease, PlacementBoundArtifactRef, PlacementExecutionOrigin,
    RecallAmplificationBudget, RecallCostClass,
};

fn main() {
    let artifact_ref = PlacementBoundArtifactRef::snapshot_family("42");
    let _ = ColdRecallLease::new(
        artifact_ref,
        RecallCostClass::Bounded,
        RecallAmplificationBudget::SingleFamilyLocalUnit,
        PlacementExecutionOrigin::Foreground,
    );
}

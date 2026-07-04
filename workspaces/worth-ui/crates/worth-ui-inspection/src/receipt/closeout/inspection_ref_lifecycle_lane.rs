#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionRefLifecycleLane {
    MaterializationPostureBoundRef,
    FollowupQueryExpansion,
    RetainedDetailExpansion,
    NotMaterializedExpansion,
    WrongGenerationExpansion,
    DiscardedTombstoneExpansion,
}

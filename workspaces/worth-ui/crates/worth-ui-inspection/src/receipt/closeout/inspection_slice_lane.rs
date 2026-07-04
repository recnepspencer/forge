#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionSliceLane {
    DeclarationIdentity,
    AuthoredSourceProvenance,
    GraphNodeIdentity,
    AspectNeighborhood,
    ObligationNeighborhood,
    FamilySummaries,
    OmissionByScope,
    OmissionByBudget,
}

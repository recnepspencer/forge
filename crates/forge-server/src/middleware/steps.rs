#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeServerPipelineStep {
    BudgetPosture,
    AuthorizationPosture,
    ValidationPosture,
    QueryHandoffPreparation,
}

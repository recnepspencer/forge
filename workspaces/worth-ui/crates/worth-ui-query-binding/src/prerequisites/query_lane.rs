#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryProjectionConsumptionLane {
    ConsumeProjectionFacts,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryInspectionLane {
    WorkspaceInspect,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryCausalExplanationLane {
    AdmitAndRequestCausalInspection,
}

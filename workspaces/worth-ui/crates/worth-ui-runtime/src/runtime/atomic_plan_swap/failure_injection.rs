#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiPlanSwapFailureInjection {
    BeforeCommit,
    AfterArtifactMutation,
}

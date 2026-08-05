#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MilestoneEightPerturbationClass {
    ScopeTemplateDirectParity,
    DirectScopeParity,
    DirectTemplateParity,
    SavedQueryFreezeParity,
    ViewShapePlanningLiveSemantics,
    KanbanDesiredStateDeltaParity,
    KanbanDeltaAdmissionBoundary,
    GroupedDeltaHonesty,
    GroupedBridgeTruthViewAuthority,
    GroupedExecutionSurfaceAuthority,
    GroupedProofChainNoPayloadRediscovery,
    InspectorSemanticDistinction,
    IdentityAwareInspectorParity,
    IdentityBreakInspectorExplicitness,
    SupportProfileHonesty,
    UnsupportedScopeFamily,
    UnsupportedTemplateFamily,
    SavedQuerySupportProfileDrift,
    DurableSavedQueryDeferredDebt,
    GroupedHiddenRefreshForbidden,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MilestoneEightFailureClass {
    UnsupportedScopeFamily,
    UnsupportedTemplateFamily,
    SavedQuerySupportProfileDrift,
    DurableSavedQueryDeferredDebt,
    GroupedHiddenRefreshForbidden,
}

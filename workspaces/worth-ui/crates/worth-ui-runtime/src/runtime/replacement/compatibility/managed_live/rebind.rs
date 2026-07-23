use crate::runtime::replacement::query_binding::{
    WorthUiQueryBindingIdentity, WorthUiQueryBindingUiRequirements,
    WorthUiQueryBindingUiRequirementsDriftFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryBindingRebindReason {
    FreshCandidateBinding,
    QueryIdentityChanged,
    QueryAuthorityChanged,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryRebindRequiredSurface {
    LiveViewsAndLivePromotion,
    SubscriptionSelectionAndDiagnostics,
    BasisCapabilityLifecycle,
    AsyncResourcesAndResultState,
    Recovery,
    Inspection,
    ProjectionConsumption,
    ContinuationPipeline,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingRebind {
    identity: WorthUiQueryBindingIdentity,
    candidate_ui_requirements: WorthUiQueryBindingUiRequirements,
    reason: WorthUiQueryBindingRebindReason,
    drift_families: Vec<WorthUiQueryBindingUiRequirementsDriftFamily>,
    required_query_surfaces: Vec<WorthUiQueryRebindRequiredSurface>,
}

impl WorthUiQueryBindingRebind {
    pub(crate) fn new(
        identity: WorthUiQueryBindingIdentity,
        candidate_ui_requirements: WorthUiQueryBindingUiRequirements,
        reason: WorthUiQueryBindingRebindReason,
        drift_families: Vec<WorthUiQueryBindingUiRequirementsDriftFamily>,
    ) -> Self {
        let required_query_surfaces = required_query_surfaces_for(reason, &drift_families);
        Self {
            identity,
            candidate_ui_requirements,
            reason,
            drift_families,
            required_query_surfaces,
        }
    }

    pub fn identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.identity
    }

    pub fn candidate_ui_requirements(&self) -> &WorthUiQueryBindingUiRequirements {
        &self.candidate_ui_requirements
    }

    pub fn reason(&self) -> WorthUiQueryBindingRebindReason {
        self.reason
    }

    pub fn drift_families(&self) -> &[WorthUiQueryBindingUiRequirementsDriftFamily] {
        &self.drift_families
    }

    pub fn required_query_surfaces(&self) -> &[WorthUiQueryRebindRequiredSurface] {
        &self.required_query_surfaces
    }
}

fn required_query_surfaces_for(
    reason: WorthUiQueryBindingRebindReason,
    drifts: &[WorthUiQueryBindingUiRequirementsDriftFamily],
) -> Vec<WorthUiQueryRebindRequiredSurface> {
    let mut surfaces = Vec::new();
    if matches!(
        reason,
        WorthUiQueryBindingRebindReason::FreshCandidateBinding
            | WorthUiQueryBindingRebindReason::QueryIdentityChanged
            | WorthUiQueryBindingRebindReason::QueryAuthorityChanged
    ) {
        push_surface(
            &mut surfaces,
            WorthUiQueryRebindRequiredSurface::LiveViewsAndLivePromotion,
        );
        push_surface(
            &mut surfaces,
            WorthUiQueryRebindRequiredSurface::SubscriptionSelectionAndDiagnostics,
        );
    }
    for drift in drifts {
        let surface = match drift {
            WorthUiQueryBindingUiRequirementsDriftFamily::LifecycleDeclaration => {
                WorthUiQueryRebindRequiredSurface::LiveViewsAndLivePromotion
            }
            WorthUiQueryBindingUiRequirementsDriftFamily::AsyncResultPresentation => {
                WorthUiQueryRebindRequiredSurface::AsyncResourcesAndResultState
            }
            WorthUiQueryBindingUiRequirementsDriftFamily::RecoveryPresentation => {
                push_surface(
                    &mut surfaces,
                    WorthUiQueryRebindRequiredSurface::ContinuationPipeline,
                );
                WorthUiQueryRebindRequiredSurface::Recovery
            }
            WorthUiQueryBindingUiRequirementsDriftFamily::InspectionRelevance => {
                WorthUiQueryRebindRequiredSurface::Inspection
            }
            WorthUiQueryBindingUiRequirementsDriftFamily::ProjectionConsumption => {
                WorthUiQueryRebindRequiredSurface::ProjectionConsumption
            }
            WorthUiQueryBindingUiRequirementsDriftFamily::DenialPresentation => continue,
        };
        push_surface(&mut surfaces, surface);
    }
    surfaces
}

fn push_surface(
    surfaces: &mut Vec<WorthUiQueryRebindRequiredSurface>,
    surface: WorthUiQueryRebindRequiredSurface,
) {
    if !surfaces.contains(&surface) {
        surfaces.push(surface);
    }
}

use crate::runtime::query_binding::{
    WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture, WorthUiQueryBindingPostureDriftFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryBindingRebindReason {
    FreshCandidateBinding,
    QueryIdentityChanged,
    QueryOwnedPostureDrift,
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
    candidate_posture: WorthUiQueryBindingPosture,
    reason: WorthUiQueryBindingRebindReason,
    drift_families: Vec<WorthUiQueryBindingPostureDriftFamily>,
    required_query_surfaces: Vec<WorthUiQueryRebindRequiredSurface>,
}

impl WorthUiQueryBindingRebind {
    pub(crate) fn new(
        identity: WorthUiQueryBindingIdentity,
        candidate_posture: WorthUiQueryBindingPosture,
        reason: WorthUiQueryBindingRebindReason,
        drift_families: Vec<WorthUiQueryBindingPostureDriftFamily>,
    ) -> Self {
        let required_query_surfaces = required_query_surfaces_for(reason, &drift_families);
        Self {
            identity,
            candidate_posture,
            reason,
            drift_families,
            required_query_surfaces,
        }
    }

    pub fn identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.identity
    }

    pub fn candidate_posture(&self) -> &WorthUiQueryBindingPosture {
        &self.candidate_posture
    }

    pub fn reason(&self) -> WorthUiQueryBindingRebindReason {
        self.reason
    }

    pub fn drift_families(&self) -> &[WorthUiQueryBindingPostureDriftFamily] {
        &self.drift_families
    }

    pub fn required_query_surfaces(&self) -> &[WorthUiQueryRebindRequiredSurface] {
        &self.required_query_surfaces
    }
}

fn required_query_surfaces_for(
    reason: WorthUiQueryBindingRebindReason,
    drifts: &[WorthUiQueryBindingPostureDriftFamily],
) -> Vec<WorthUiQueryRebindRequiredSurface> {
    let mut surfaces = Vec::new();
    if matches!(
        reason,
        WorthUiQueryBindingRebindReason::FreshCandidateBinding
            | WorthUiQueryBindingRebindReason::QueryIdentityChanged
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
            WorthUiQueryBindingPostureDriftFamily::SupportAdmission => {
                WorthUiQueryRebindRequiredSurface::SubscriptionSelectionAndDiagnostics
            }
            WorthUiQueryBindingPostureDriftFamily::BasisCapability => {
                WorthUiQueryRebindRequiredSurface::BasisCapabilityLifecycle
            }
            WorthUiQueryBindingPostureDriftFamily::LiveCompatibility => {
                WorthUiQueryRebindRequiredSurface::LiveViewsAndLivePromotion
            }
            WorthUiQueryBindingPostureDriftFamily::AsyncResultState => {
                WorthUiQueryRebindRequiredSurface::AsyncResourcesAndResultState
            }
            WorthUiQueryBindingPostureDriftFamily::Recovery => {
                push_surface(
                    &mut surfaces,
                    WorthUiQueryRebindRequiredSurface::ContinuationPipeline,
                );
                WorthUiQueryRebindRequiredSurface::Recovery
            }
            WorthUiQueryBindingPostureDriftFamily::Inspection => {
                WorthUiQueryRebindRequiredSurface::Inspection
            }
            WorthUiQueryBindingPostureDriftFamily::ProjectionConsumption => {
                WorthUiQueryRebindRequiredSurface::ProjectionConsumption
            }
            WorthUiQueryBindingPostureDriftFamily::DenialPresentation => continue,
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

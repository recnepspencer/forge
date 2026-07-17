use crate::runtime::{
    WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture, WorthUiQueryLaneSupportLinks,
    WorthUiQueryRebindRequiredSurface,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryPatchPosture {
    plan_index: u32,
    binding_identity: WorthUiQueryBindingIdentity,
    posture: WorthUiQueryBindingPosture,
    required_surfaces: Vec<WorthUiQueryRebindRequiredSurface>,
}

impl WorthUiQueryPatchPosture {
    pub(crate) fn from_query_support_links(links: &WorthUiQueryLaneSupportLinks) -> Self {
        Self {
            plan_index: links.plan_index(),
            binding_identity: links.binding_identity().clone(),
            posture: links.posture().clone(),
            required_surfaces: links.required_surfaces().to_vec(),
        }
    }

    pub fn plan_index(&self) -> u32 {
        self.plan_index
    }

    pub fn binding_identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.binding_identity
    }

    pub fn view_binding_id(&self) -> &str {
        self.binding_identity.view_binding_id()
    }

    pub fn posture(&self) -> &WorthUiQueryBindingPosture {
        &self.posture
    }

    pub fn required_surfaces(&self) -> &[WorthUiQueryRebindRequiredSurface] {
        &self.required_surfaces
    }

    pub(crate) fn canonical_digest(&self) -> u64 {
        let mut digest = 0x7669_7274_7165_7279u64;
        digest = fold_u64(digest, u64::from(self.plan_index));
        digest = fold_u64(digest, self.binding_identity.canonical_identity());
        digest = fold_u64(digest, self.posture.canonical_identity());
        self.required_surfaces
            .iter()
            .fold(digest, |digest, surface| {
                fold_u64(digest, query_required_surface_tag(*surface))
            })
    }
}

fn fold_u64(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x100000001b3)
}

fn query_required_surface_tag(surface: WorthUiQueryRebindRequiredSurface) -> u64 {
    match surface {
        WorthUiQueryRebindRequiredSurface::LiveViewsAndLivePromotion => 1,
        WorthUiQueryRebindRequiredSurface::SubscriptionSelectionAndDiagnostics => 2,
        WorthUiQueryRebindRequiredSurface::BasisCapabilityLifecycle => 3,
        WorthUiQueryRebindRequiredSurface::AsyncResourcesAndResultState => 4,
        WorthUiQueryRebindRequiredSurface::Recovery => 5,
        WorthUiQueryRebindRequiredSurface::Inspection => 6,
        WorthUiQueryRebindRequiredSurface::ProjectionConsumption => 7,
        WorthUiQueryRebindRequiredSurface::ContinuationPipeline => 8,
    }
}

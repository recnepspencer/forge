use crate::runtime::{WorthUiQueryLaneSupportLinks, WorthUiQueryRebindRequiredSurface};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryPatchPosture {
    plan_index: u32,
    view_binding_id: String,
    support_admission_digest: String,
    live_compatibility_digest: String,
    async_result_state_digest: String,
    inspection_digest: String,
    projection_consumption_digest: String,
    recovery_digest: String,
    required_surfaces: Vec<WorthUiQueryRebindRequiredSurface>,
}

impl WorthUiQueryPatchPosture {
    pub(crate) fn from_query_support_links(links: &WorthUiQueryLaneSupportLinks) -> Self {
        Self {
            plan_index: links.plan_index(),
            view_binding_id: links.view_binding_id().to_owned(),
            support_admission_digest: links.support_admission_digest().to_owned(),
            live_compatibility_digest: links.live_compatibility_digest().to_owned(),
            async_result_state_digest: links.async_result_state_digest().to_owned(),
            inspection_digest: links.inspection_digest().to_owned(),
            projection_consumption_digest: links.projection_consumption_digest().to_owned(),
            recovery_digest: links.recovery_digest().to_owned(),
            required_surfaces: links.required_surfaces().to_vec(),
        }
    }

    pub fn plan_index(&self) -> u32 {
        self.plan_index
    }

    pub fn view_binding_id(&self) -> &str {
        &self.view_binding_id
    }

    pub fn support_admission_digest(&self) -> &str {
        &self.support_admission_digest
    }

    pub fn live_compatibility_digest(&self) -> &str {
        &self.live_compatibility_digest
    }

    pub fn async_result_state_digest(&self) -> &str {
        &self.async_result_state_digest
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }

    pub fn projection_consumption_digest(&self) -> &str {
        &self.projection_consumption_digest
    }

    pub fn recovery_digest(&self) -> &str {
        &self.recovery_digest
    }

    pub fn required_surfaces(&self) -> &[WorthUiQueryRebindRequiredSurface] {
        &self.required_surfaces
    }

    pub(crate) fn canonical_digest(&self) -> u64 {
        let mut digest = 0x7669_7274_7165_7279u64;
        digest = fold_u64(digest, u64::from(self.plan_index));
        digest = fold_text(digest, &self.view_binding_id);
        digest = fold_text(digest, &self.support_admission_digest);
        digest = fold_text(digest, &self.live_compatibility_digest);
        digest = fold_text(digest, &self.async_result_state_digest);
        digest = fold_text(digest, &self.inspection_digest);
        digest = fold_text(digest, &self.projection_consumption_digest);
        digest = fold_text(digest, &self.recovery_digest);
        self.required_surfaces
            .iter()
            .fold(digest, |digest, surface| {
                fold_u64(digest, query_required_surface_tag(*surface))
            })
    }
}

fn fold_text(digest: u64, text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(digest, |digest, byte| fold_u64(digest, u64::from(*byte)))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_digest_binds_query_owned_patch_posture() {
        let baseline = posture_with_digests("live", "async", "inspect", "project", "recover");

        for changed in [
            posture_with_digests("live-changed", "async", "inspect", "project", "recover"),
            posture_with_digests("live", "async-changed", "inspect", "project", "recover"),
            posture_with_digests("live", "async", "inspect-changed", "project", "recover"),
            posture_with_digests("live", "async", "inspect", "project-changed", "recover"),
            posture_with_digests("live", "async", "inspect", "project", "recover-changed"),
            posture_with_surfaces(vec![WorthUiQueryRebindRequiredSurface::Recovery]),
        ] {
            assert_ne!(baseline.canonical_digest(), changed.canonical_digest());
        }
    }

    fn posture_with_digests(
        live_compatibility_digest: &str,
        async_result_state_digest: &str,
        inspection_digest: &str,
        projection_consumption_digest: &str,
        recovery_digest: &str,
    ) -> WorthUiQueryPatchPosture {
        WorthUiQueryPatchPosture {
            plan_index: 7,
            view_binding_id: "orders".to_owned(),
            support_admission_digest: "support".to_owned(),
            live_compatibility_digest: live_compatibility_digest.to_owned(),
            async_result_state_digest: async_result_state_digest.to_owned(),
            inspection_digest: inspection_digest.to_owned(),
            projection_consumption_digest: projection_consumption_digest.to_owned(),
            recovery_digest: recovery_digest.to_owned(),
            required_surfaces: vec![WorthUiQueryRebindRequiredSurface::ProjectionConsumption],
        }
    }

    fn posture_with_surfaces(
        required_surfaces: Vec<WorthUiQueryRebindRequiredSurface>,
    ) -> WorthUiQueryPatchPosture {
        WorthUiQueryPatchPosture {
            required_surfaces,
            ..posture_with_digests("live", "async", "inspect", "project", "recover")
        }
    }
}

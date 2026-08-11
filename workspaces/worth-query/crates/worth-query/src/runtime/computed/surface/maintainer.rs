use super::*;

pub trait WorthQueryDerivedViewMaintainer {
    fn maintain(
        &mut self,
        view: &WorthQueryDerivedView,
        delta: &WorthQueryMutationDelta,
        materialization: &mut WorthQueryDerivedViewMaterialization,
    ) -> WorthQueryDerivedPatch;

    fn refresh_from_upstreams(
        &mut self,
        _view: &WorthQueryDerivedView,
        _refresh: &WorthQueryRetainedRefreshContext,
        _upstreams: &WorthQueryRetainedUpstreamInputs,
        _materialization: &mut WorthQueryDerivedViewMaterialization,
    ) -> Option<WorthQueryDerivedPatch> {
        None
    }
}

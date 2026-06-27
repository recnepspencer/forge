use crate::capability::{
    AdmittedCapability, QueryBasisPostureReference, QueryDenialPresentation,
    QueryLiveCompatibility, QueryResultShapeReference, QueryViewCapabilityReference, ViewBindingId,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum WorthUiQueryInspectionLinkRole {
    BindingViewBindingQuery,
    SurfaceViewBindingQuery,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiQueryInspectionLink {
    role: WorthUiQueryInspectionLinkRole,
    view_binding: AdmittedCapability<ViewBindingId>,
    query_capability: QueryViewCapabilityReference,
    query_composition_profile_digest: String,
    result_shape: QueryResultShapeReference,
    basis_posture: QueryBasisPostureReference,
    live_compatibility: QueryLiveCompatibility,
    denial_presentation: QueryDenialPresentation,
}

impl WorthUiQueryInspectionLink {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        role: WorthUiQueryInspectionLinkRole,
        view_binding: AdmittedCapability<ViewBindingId>,
        query_capability: QueryViewCapabilityReference,
        query_composition_profile_digest: String,
        result_shape: QueryResultShapeReference,
        basis_posture: QueryBasisPostureReference,
        live_compatibility: QueryLiveCompatibility,
        denial_presentation: QueryDenialPresentation,
    ) -> Self {
        Self {
            role,
            view_binding,
            query_capability,
            query_composition_profile_digest,
            result_shape,
            basis_posture,
            live_compatibility,
            denial_presentation,
        }
    }

    pub(crate) fn role(&self) -> WorthUiQueryInspectionLinkRole {
        self.role
    }

    pub(crate) fn view_binding(&self) -> &AdmittedCapability<ViewBindingId> {
        &self.view_binding
    }

    pub(crate) fn query_capability(&self) -> &QueryViewCapabilityReference {
        &self.query_capability
    }

    pub(crate) fn query_composition_profile_digest(&self) -> &str {
        &self.query_composition_profile_digest
    }

    pub(crate) fn result_shape(&self) -> &QueryResultShapeReference {
        &self.result_shape
    }

    pub(crate) fn basis_posture(&self) -> &QueryBasisPostureReference {
        &self.basis_posture
    }

    pub(crate) fn live_compatibility(&self) -> &QueryLiveCompatibility {
        &self.live_compatibility
    }

    pub(crate) fn denial_presentation(&self) -> &QueryDenialPresentation {
        &self.denial_presentation
    }
}

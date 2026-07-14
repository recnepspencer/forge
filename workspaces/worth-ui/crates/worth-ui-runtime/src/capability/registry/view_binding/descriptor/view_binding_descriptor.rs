use worth_query::facade::foundation::QueryCompositionSupportProfile;
use worth_query::facade::runtime::ViewShapeDescriptor;

use crate::capability::ViewBindingId;

use super::{
    QueryBasisPostureReference, QueryDenialPresentation, QueryLiveCompatibility,
    QueryResultShapeReference, QueryViewCapabilityReference, ViewBindingFamily,
    VisibleStateBindingDeclaration,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewBindingDescriptor {
    id: ViewBindingId,
    family: ViewBindingFamily,
    query_capability: Option<QueryViewCapabilityReference>,
    query_composition_profile_digest: Option<String>,
    view_shape: Option<ViewShapeDescriptor>,
    result_shape: Option<QueryResultShapeReference>,
    basis_posture: Option<QueryBasisPostureReference>,
    live_compatibility: Option<QueryLiveCompatibility>,
    visible_state_bindings: Vec<VisibleStateBindingDeclaration>,
    denial_presentation: Option<QueryDenialPresentation>,
    local_pseudo_query_claim: Option<String>,
}

impl ViewBindingDescriptor {
    pub fn query_owned(id: ViewBindingId, family: ViewBindingFamily) -> Self {
        Self {
            id,
            family,
            query_capability: None,
            query_composition_profile_digest: None,
            view_shape: None,
            result_shape: None,
            basis_posture: None,
            live_compatibility: None,
            visible_state_bindings: Vec::new(),
            denial_presentation: None,
            local_pseudo_query_claim: None,
        }
    }

    pub fn local_pseudo_query_for_diagnostics(
        id: ViewBindingId,
        family: ViewBindingFamily,
        claim: impl Into<String>,
    ) -> Self {
        let mut descriptor = Self::query_owned(id, family);
        descriptor.local_pseudo_query_claim = Some(claim.into());
        descriptor
    }

    pub fn with_query_capability_posture(
        mut self,
        query_capability: QueryViewCapabilityReference,
    ) -> Self {
        self.query_capability = Some(query_capability);
        self
    }

    pub fn with_query_composition_support(
        mut self,
        profile: &QueryCompositionSupportProfile,
    ) -> Self {
        self.query_composition_profile_digest = Some(profile.profile_digest().to_string());
        self
    }

    pub fn with_view_shape(mut self, view_shape: ViewShapeDescriptor) -> Self {
        self.view_shape = Some(view_shape);
        self
    }

    pub fn with_result_shape(mut self, result_shape: QueryResultShapeReference) -> Self {
        self.result_shape = Some(result_shape);
        self
    }

    pub fn with_basis_posture(mut self, basis_posture: QueryBasisPostureReference) -> Self {
        self.basis_posture = Some(basis_posture);
        self
    }

    pub fn with_live_compatibility(mut self, live_compatibility: QueryLiveCompatibility) -> Self {
        self.live_compatibility = Some(live_compatibility);
        self
    }

    pub fn with_visible_state_binding(
        mut self,
        visible_state_binding: VisibleStateBindingDeclaration,
    ) -> Self {
        self.visible_state_bindings.push(visible_state_binding);
        self
    }

    pub fn with_denial_presentation(
        mut self,
        denial_presentation: QueryDenialPresentation,
    ) -> Self {
        self.denial_presentation = Some(denial_presentation);
        self
    }

    pub fn id(&self) -> &ViewBindingId {
        &self.id
    }

    pub fn family(&self) -> &ViewBindingFamily {
        &self.family
    }

    pub fn query_capability(&self) -> Option<&QueryViewCapabilityReference> {
        self.query_capability.as_ref()
    }

    pub fn query_composition_profile_digest(&self) -> Option<&str> {
        self.query_composition_profile_digest.as_deref()
    }

    pub fn view_shape(&self) -> Option<&ViewShapeDescriptor> {
        self.view_shape.as_ref()
    }

    pub fn result_shape(&self) -> Option<&QueryResultShapeReference> {
        self.result_shape.as_ref()
    }

    pub fn basis_posture(&self) -> Option<&QueryBasisPostureReference> {
        self.basis_posture.as_ref()
    }

    pub fn live_compatibility(&self) -> Option<&QueryLiveCompatibility> {
        self.live_compatibility.as_ref()
    }

    pub fn visible_state_bindings(&self) -> &[VisibleStateBindingDeclaration] {
        &self.visible_state_bindings
    }

    pub fn denial_presentation(&self) -> Option<&QueryDenialPresentation> {
        self.denial_presentation.as_ref()
    }

    pub(crate) fn has_local_pseudo_query_claim(&self) -> bool {
        self.local_pseudo_query_claim.is_some()
    }
}

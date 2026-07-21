use std::rc::Rc;

use crate::runtime::{
    WorthUiNodeLifecycleTransition, WorthUiPlanNodeInput, WorthUiPlanNodeInputFamily,
    WorthUiPlanNodeTopologyInput, WorthUiQueryRebindRequiredSurface,
};

impl WorthUiPlanNodeInput {
    pub(crate) fn with_authored_provenance_digest_for_test(mut self, digest: u64) -> Self {
        self.authored_provenance_digest = Some(digest);
        self
    }

    pub(crate) fn without_topology_input_for_test(mut self) -> Self {
        self.topology_input = WorthUiPlanNodeTopologyInput::empty();
        self
    }

    pub(crate) fn with_declared_topology_input_for_test(mut self) -> Self {
        self.topology_input = WorthUiPlanNodeTopologyInput::declared_for_test();
        self
    }

    pub(crate) fn with_family_for_test(mut self, family: WorthUiPlanNodeInputFamily) -> Self {
        if self.family == WorthUiPlanNodeInputFamily::QueryViewBinding
            && family != WorthUiPlanNodeInputFamily::QueryViewBinding
        {
            self.query_binding_identity = None;
            self.query_installed_reference = None;
            self.query_binding_posture = None;
            self.query_required_surfaces.clear();
            self.query_preservation_receipt = None;
        }
        if family != WorthUiPlanNodeInputFamily::CanvasSpatial {
            self.spatial_meaning = None;
        }
        if family != WorthUiPlanNodeInputFamily::RealtimeOverlay {
            self.realtime_meaning = None;
        }
        self.family = family;
        self
    }

    pub(crate) fn with_identity_basis_for_test(
        mut self,
        identity_basis: impl Into<String>,
    ) -> Self {
        self.identity_basis = identity_basis.into();
        self
    }

    pub(crate) fn with_transition_for_test(
        mut self,
        transition: WorthUiNodeLifecycleTransition,
    ) -> Self {
        self.transition = Some(transition);
        self
    }

    pub(crate) fn with_owner_identity_basis_for_test(
        mut self,
        owner_identity_basis: impl Into<String>,
    ) -> Self {
        self.owner_identity_basis = Some(owner_identity_basis.into());
        self
    }

    pub(crate) fn with_query_installed_reference_for_test(
        mut self,
        reference: worth_ui_query_binding::WorthUiInstalledQueryBindingReference,
    ) -> Self {
        self.query_installed_reference = Some(Rc::new(reference));
        self
    }

    pub(crate) fn without_query_binding_identity_for_test(mut self) -> Self {
        self.query_binding_identity = None;
        self
    }

    pub(crate) fn without_query_installed_reference_for_test(mut self) -> Self {
        self.query_installed_reference = None;
        self
    }

    pub(crate) fn without_query_binding_posture_for_test(mut self) -> Self {
        self.query_binding_posture = None;
        self
    }

    pub(crate) fn with_query_required_surface_for_test(
        mut self,
        surface: WorthUiQueryRebindRequiredSurface,
    ) -> Self {
        self.query_required_surfaces.push(surface);
        self
    }

    pub(crate) fn with_owned_region_identity_for_test(
        mut self,
        identity: impl Into<String>,
    ) -> Self {
        self.owned_region_identity_bases.push(identity.into());
        self
    }

    pub(crate) fn without_ordinary_meaning_for_test(mut self) -> Self {
        self.ordinary_meaning = None;
        self
    }
}

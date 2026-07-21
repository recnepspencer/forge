use std::rc::Rc;

use crate::runtime::{
    WorthUiNodeLifecycleTransition, WorthUiPlanNodeInput, WorthUiPlanNodeInputFamily,
    WorthUiPlanNodeTopologyInput,
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
            self.query_settled_fact_link = None;
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
        self.query_settled_fact_link = self
            .query_settled_fact_link
            .as_ref()
            .map(|link| Rc::new(link.with_installed_reference_for_test(reference)));
        self
    }

    pub(crate) fn without_query_binding_identity_for_test(mut self) -> Self {
        self.query_binding_identity = None;
        self
    }

    pub(crate) fn without_query_installed_reference_for_test(mut self) -> Self {
        self.query_settled_fact_link = None;
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

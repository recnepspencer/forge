use std::rc::Rc;

use crate::runtime::{
    WorthUiComponentLoweringHook, WorthUiNodeLifecycleTransition, WorthUiPlanNodeInputFamily,
    WorthUiPlanNodeTopologyInput, WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture,
    WorthUiQueryLiveRebindEntry, WorthUiQueryRebindRequiredSurface,
};

// Exact executable equivalence deliberately excludes lifecycle action,
// source-only provenance, and a predecessor-authority receipt. Those fields
// remain validation/inspection truth and cannot change executable meaning.
// non-executable-schema-fields: authored_provenance_digest, transition,
// query_preservation_receipt

#[derive(Clone, Debug)]
pub struct WorthUiPlanNodeInput {
    pub(super) identity_basis: String,
    pub(super) authored_provenance_digest: Option<u64>,
    pub(super) family: WorthUiPlanNodeInputFamily,
    pub(super) transition: Option<WorthUiNodeLifecycleTransition>,
    pub(super) query_binding_identity: Option<Rc<WorthUiQueryBindingIdentity>>,
    pub(super) query_installed_reference:
        Option<Rc<worth_ui_query_binding::WorthUiInstalledQueryBindingReference>>,
    pub(super) query_binding_posture: Option<WorthUiQueryBindingPosture>,
    pub(super) query_required_surfaces: Vec<WorthUiQueryRebindRequiredSurface>,
    pub(super) query_preservation_receipt:
        Option<crate::runtime::WorthUiQueryBindingPreservationReceipt>,
    pub(super) topology_input: WorthUiPlanNodeTopologyInput,
    pub(super) owner_identity_basis: Option<String>,
    pub(super) owned_region_identity_bases: Vec<String>,
    pub(super) ordinary_meaning: Option<Rc<super::WorthUiPlanOrdinaryMeaning>>,
    pub(super) spatial_meaning: Option<Rc<super::WorthUiSpatialPlanMeaning>>,
    pub(super) realtime_meaning: Option<Rc<super::WorthUiRealtimePlanMeaning>>,
}

impl PartialEq for WorthUiPlanNodeInput {
    fn eq(&self, other: &Self) -> bool {
        self.identity_basis == other.identity_basis
            && self.authored_provenance_digest == other.authored_provenance_digest
            && self.family == other.family
            && self.transition == other.transition
            && self.query_binding_identity == other.query_binding_identity
            && self.query_installed_reference == other.query_installed_reference
            && self.query_binding_posture == other.query_binding_posture
            && self.query_required_surfaces == other.query_required_surfaces
            && self.query_preservation_receipt == other.query_preservation_receipt
            && self.topology_input == other.topology_input
            && self.owner_identity_basis == other.owner_identity_basis
            && self.owned_region_identity_bases == other.owned_region_identity_bases
            && self.ordinary_meaning == other.ordinary_meaning
            && self.spatial_meaning == other.spatial_meaning
            && self.realtime_meaning == other.realtime_meaning
    }
}

impl Eq for WorthUiPlanNodeInput {}

impl WorthUiPlanNodeInput {
    pub(crate) fn executable_schema_matches(&self, other: &Self) -> bool {
        self.identity_basis == other.identity_basis
            && self.family == other.family
            && self.query_binding_identity == other.query_binding_identity
            && self.query_installed_reference == other.query_installed_reference
            && self.query_binding_posture == other.query_binding_posture
            && self.query_required_surfaces == other.query_required_surfaces
            && self.topology_input == other.topology_input
            && self.owner_identity_basis == other.owner_identity_basis
            && self.owned_region_identity_bases == other.owned_region_identity_bases
            && self.ordinary_meaning == other.ordinary_meaning
            && self.spatial_meaning == other.spatial_meaning
            && self.realtime_meaning == other.realtime_meaning
    }

    pub(crate) fn from_launch_query_binding(
        identity: &WorthUiQueryBindingIdentity,
        installed_reference: Option<worth_ui_query_binding::WorthUiInstalledQueryBindingReference>,
        posture: &WorthUiQueryBindingPosture,
        topology_input: WorthUiPlanNodeTopologyInput,
    ) -> Self {
        Self {
            identity_basis: identity.view_binding_id().to_owned(),
            authored_provenance_digest: None,
            family: WorthUiPlanNodeInputFamily::QueryViewBinding,
            transition: Some(WorthUiNodeLifecycleTransition::Create),
            query_binding_identity: Some(Rc::new(identity.clone())),
            query_installed_reference: installed_reference.map(Rc::new),
            query_binding_posture: Some(posture.clone()),
            query_required_surfaces: Vec::new(),
            query_preservation_receipt: None,
            topology_input,
            owner_identity_basis: None,
            owned_region_identity_bases: Vec::new(),
            ordinary_meaning: None,
            spatial_meaning: None,
            realtime_meaning: None,
        }
    }

    pub(crate) fn from_query_rebind_entry(
        entry: &WorthUiQueryLiveRebindEntry,
        installed_reference: Option<worth_ui_query_binding::WorthUiInstalledQueryBindingReference>,
        topology_input: WorthUiPlanNodeTopologyInput,
    ) -> Self {
        let query_binding_posture = super::query_rebind_node_input::posture(entry);
        let query_required_surfaces = super::query_rebind_node_input::required_surfaces(entry);
        let query_preservation_receipt =
            super::query_rebind_node_input::preservation_receipt(entry);
        Self {
            identity_basis: entry.identity().view_binding_id().to_owned(),
            authored_provenance_digest: None,
            family: WorthUiPlanNodeInputFamily::QueryViewBinding,
            transition: Some(super::query_rebind_node_input::transition(entry)),
            query_binding_identity: Some(Rc::new(entry.identity().clone())),
            query_installed_reference: installed_reference.map(Rc::new),
            query_binding_posture,
            query_required_surfaces,
            query_preservation_receipt,
            topology_input,
            owner_identity_basis: None,
            owned_region_identity_bases: Vec::new(),
            ordinary_meaning: None,
            spatial_meaning: None,
            realtime_meaning: None,
        }
    }

    pub(crate) fn from_component_hook(
        hook: &WorthUiComponentLoweringHook,
        family: WorthUiPlanNodeInputFamily,
    ) -> Self {
        Self {
            identity_basis: hook.hook_id().to_owned(),
            authored_provenance_digest: None,
            family,
            transition: None,
            query_binding_identity: None,
            query_installed_reference: None,
            query_binding_posture: None,
            query_required_surfaces: Vec::new(),
            query_preservation_receipt: None,
            topology_input: WorthUiPlanNodeTopologyInput::empty(),
            owner_identity_basis: None,
            owned_region_identity_bases: Vec::new(),
            ordinary_meaning: None,
            spatial_meaning: None,
            realtime_meaning: None,
        }
    }

    pub(crate) fn from_ordinary_row(
        identity_basis: String,
        authored_provenance_digest: Option<u64>,
        family: WorthUiPlanNodeInputFamily,
        transition: WorthUiNodeLifecycleTransition,
        topology_input: WorthUiPlanNodeTopologyInput,
        owner_identity_basis: Option<String>,
        ordinary_meaning: super::WorthUiPlanOrdinaryMeaning,
    ) -> Self {
        Self {
            identity_basis,
            authored_provenance_digest,
            family,
            transition: Some(transition),
            query_binding_identity: None,
            query_installed_reference: None,
            query_binding_posture: None,
            query_required_surfaces: Vec::new(),
            query_preservation_receipt: None,
            topology_input,
            owner_identity_basis,
            owned_region_identity_bases: Vec::new(),
            ordinary_meaning: Some(Rc::new(ordinary_meaning)),
            spatial_meaning: None,
            realtime_meaning: None,
        }
    }

    pub(crate) fn from_spatial_component(
        identity_basis: String,
        authored_provenance_digest: Option<u64>,
        transition: WorthUiNodeLifecycleTransition,
        topology_input: WorthUiPlanNodeTopologyInput,
        meaning: super::WorthUiSpatialPlanMeaning,
    ) -> Self {
        Self {
            identity_basis,
            authored_provenance_digest,
            family: WorthUiPlanNodeInputFamily::CanvasSpatial,
            transition: Some(transition),
            query_binding_identity: None,
            query_installed_reference: None,
            query_binding_posture: None,
            query_required_surfaces: Vec::new(),
            query_preservation_receipt: None,
            topology_input,
            owner_identity_basis: None,
            owned_region_identity_bases: Vec::new(),
            ordinary_meaning: None,
            spatial_meaning: Some(Rc::new(meaning)),
            realtime_meaning: None,
        }
    }

    pub(crate) fn from_realtime_component(
        identity_basis: String,
        authored_provenance_digest: Option<u64>,
        transition: WorthUiNodeLifecycleTransition,
        topology_input: WorthUiPlanNodeTopologyInput,
        meaning: super::WorthUiRealtimePlanMeaning,
    ) -> Self {
        Self {
            identity_basis,
            authored_provenance_digest,
            family: WorthUiPlanNodeInputFamily::RealtimeOverlay,
            transition: Some(transition),
            query_binding_identity: None,
            query_installed_reference: None,
            query_binding_posture: None,
            query_required_surfaces: Vec::new(),
            query_preservation_receipt: None,
            topology_input,
            owner_identity_basis: None,
            owned_region_identity_bases: Vec::new(),
            ordinary_meaning: None,
            spatial_meaning: None,
            realtime_meaning: Some(Rc::new(meaning)),
        }
    }

    pub(crate) fn set_owned_region_identity_bases(&mut self, identities: Vec<String>) {
        self.owned_region_identity_bases = identities;
    }

    pub fn identity_basis(&self) -> &str {
        &self.identity_basis
    }

    pub fn family(&self) -> WorthUiPlanNodeInputFamily {
        self.family
    }

    pub fn authored_provenance_digest(&self) -> Option<u64> {
        self.authored_provenance_digest
    }

    pub fn transition(&self) -> Option<WorthUiNodeLifecycleTransition> {
        self.transition
    }

    pub fn query_binding_identity(&self) -> Option<&WorthUiQueryBindingIdentity> {
        self.query_binding_identity.as_deref()
    }

    pub(crate) fn query_binding_identity_reference(
        &self,
    ) -> Option<Rc<WorthUiQueryBindingIdentity>> {
        self.query_binding_identity.as_ref().map(Rc::clone)
    }

    pub(crate) fn query_installed_reference(
        &self,
    ) -> Option<&worth_ui_query_binding::WorthUiInstalledQueryBindingReference> {
        self.query_installed_reference.as_deref()
    }

    pub(crate) fn query_installed_reference_shared(
        &self,
    ) -> Option<Rc<worth_ui_query_binding::WorthUiInstalledQueryBindingReference>> {
        self.query_installed_reference.as_ref().map(Rc::clone)
    }

    pub fn query_binding_posture(&self) -> Option<&WorthUiQueryBindingPosture> {
        self.query_binding_posture.as_ref()
    }

    pub fn query_required_surfaces(&self) -> &[WorthUiQueryRebindRequiredSurface] {
        &self.query_required_surfaces
    }

    pub fn query_preservation_receipt(
        &self,
    ) -> Option<crate::runtime::WorthUiQueryBindingPreservationReceipt> {
        self.query_preservation_receipt
    }

    pub fn topology_input(&self) -> WorthUiPlanNodeTopologyInput {
        self.topology_input
    }

    pub(crate) fn owner_identity_basis(&self) -> Option<&str> {
        self.owner_identity_basis.as_deref()
    }

    pub(crate) fn owned_region_identity_bases(&self) -> &[String] {
        &self.owned_region_identity_bases
    }

    pub(crate) fn ordinary_meaning(&self) -> Option<&super::WorthUiPlanOrdinaryMeaning> {
        self.ordinary_meaning.as_deref()
    }

    pub(crate) fn ordinary_meaning_reference(
        &self,
    ) -> Option<Rc<super::WorthUiPlanOrdinaryMeaning>> {
        self.ordinary_meaning.as_ref().map(Rc::clone)
    }

    pub(crate) fn spatial_meaning_reference(&self) -> Option<Rc<super::WorthUiSpatialPlanMeaning>> {
        self.spatial_meaning.as_ref().map(Rc::clone)
    }

    pub(crate) fn realtime_meaning_reference(
        &self,
    ) -> Option<Rc<super::WorthUiRealtimePlanMeaning>> {
        self.realtime_meaning.as_ref().map(Rc::clone)
    }

    pub(crate) fn dependency_identity_bases(&self) -> Vec<&str> {
        self.ordinary_meaning().map_or_else(
            Vec::new,
            super::WorthUiPlanOrdinaryMeaning::dependency_identities,
        )
    }
}

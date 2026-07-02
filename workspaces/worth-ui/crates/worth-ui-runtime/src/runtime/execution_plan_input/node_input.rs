use crate::runtime::{
    WorthUiComponentLoweringHook, WorthUiEguiBoundaryInput, WorthUiIdentityMatchNodeKind,
    WorthUiNodeLifecycleTransition, WorthUiNodeReplacementClassification,
    WorthUiPlanNodeInputFamily, WorthUiPlanNodeTopologyInput, WorthUiQueryBindingIdentity,
    WorthUiQueryBindingPosture, WorthUiQueryLiveRebindEntry, WorthUiQueryLiveRebindOutcome,
    WorthUiQueryRebindRequiredSurface,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanNodeInput {
    identity_basis: String,
    authored_provenance_digest: Option<u64>,
    family: WorthUiPlanNodeInputFamily,
    transition: Option<WorthUiNodeLifecycleTransition>,
    query_binding_identity: Option<WorthUiQueryBindingIdentity>,
    query_binding_posture: Option<WorthUiQueryBindingPosture>,
    query_required_surfaces: Vec<WorthUiQueryRebindRequiredSurface>,
    query_preservation_receipt: Option<String>,
    egui_boundary_input: Option<WorthUiEguiBoundaryInput>,
    topology_input: WorthUiPlanNodeTopologyInput,
}

impl WorthUiPlanNodeInput {
    pub(crate) fn from_replacement_classification(
        classification: &WorthUiNodeReplacementClassification,
        topology_input: WorthUiPlanNodeTopologyInput,
    ) -> Self {
        let family = family_for_classification(classification);
        Self {
            identity_basis: classification.identity_basis().to_owned(),
            authored_provenance_digest: classification.authored_provenance_digest(),
            family,
            transition: Some(classification.transition()),
            query_binding_identity: None,
            query_binding_posture: None,
            query_required_surfaces: Vec::new(),
            query_preservation_receipt: None,
            egui_boundary_input: egui_boundary_for_family(family),
            topology_input,
        }
    }

    pub(crate) fn from_query_rebind_entry(
        entry: &WorthUiQueryLiveRebindEntry,
        topology_input: WorthUiPlanNodeTopologyInput,
    ) -> Self {
        let query_binding_posture = posture_for_query_rebind_entry(entry);
        let query_required_surfaces = required_surfaces_for_query_rebind_entry(entry);
        let query_preservation_receipt = preservation_receipt_for_query_rebind_entry(entry);
        Self {
            identity_basis: entry.identity().view_binding_id().to_owned(),
            authored_provenance_digest: None,
            family: WorthUiPlanNodeInputFamily::QueryViewBinding,
            transition: None,
            query_binding_identity: Some(entry.identity().clone()),
            query_binding_posture,
            query_required_surfaces,
            query_preservation_receipt,
            egui_boundary_input: Some(WorthUiEguiBoundaryInput::QueryBinding),
            topology_input,
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
            query_binding_posture: None,
            query_required_surfaces: Vec::new(),
            query_preservation_receipt: None,
            egui_boundary_input: egui_boundary_for_family(family),
            topology_input: WorthUiPlanNodeTopologyInput::empty(),
        }
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
        self.query_binding_identity.as_ref()
    }

    pub fn query_binding_posture(&self) -> Option<&WorthUiQueryBindingPosture> {
        self.query_binding_posture.as_ref()
    }

    pub fn query_required_surfaces(&self) -> &[WorthUiQueryRebindRequiredSurface] {
        &self.query_required_surfaces
    }

    pub fn query_preservation_receipt(&self) -> Option<&str> {
        self.query_preservation_receipt.as_deref()
    }

    pub fn egui_boundary_input(&self) -> Option<WorthUiEguiBoundaryInput> {
        self.egui_boundary_input
    }

    pub fn topology_input(&self) -> WorthUiPlanNodeTopologyInput {
        self.topology_input
    }

    pub fn query_projection_consumption_digest(&self) -> Option<&str> {
        self.query_binding_posture
            .as_ref()
            .map(WorthUiQueryBindingPosture::projection_consumption_digest)
    }

    pub fn query_async_result_state_digest(&self) -> Option<&str> {
        self.query_binding_posture
            .as_ref()
            .map(WorthUiQueryBindingPosture::async_result_state_digest)
    }

    pub fn query_recovery_digest(&self) -> Option<&str> {
        self.query_binding_posture
            .as_ref()
            .map(WorthUiQueryBindingPosture::recovery_digest)
    }

    #[cfg(test)]
    pub(crate) fn without_egui_boundary_for_test(mut self) -> Self {
        self.egui_boundary_input = None;
        self
    }

    #[cfg(test)]
    pub(crate) fn without_topology_input_for_test(mut self) -> Self {
        self.topology_input = WorthUiPlanNodeTopologyInput::empty();
        self
    }

    #[cfg(test)]
    pub(crate) fn with_family_for_test(mut self, family: WorthUiPlanNodeInputFamily) -> Self {
        self.family = family;
        self.egui_boundary_input = egui_boundary_for_family(family);
        self
    }

    #[cfg(test)]
    pub(crate) fn with_authored_provenance_digest_for_test(
        mut self,
        authored_provenance_digest: Option<u64>,
    ) -> Self {
        self.authored_provenance_digest = authored_provenance_digest;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_identity_basis_for_test(
        mut self,
        identity_basis: impl Into<String>,
    ) -> Self {
        self.identity_basis = identity_basis.into();
        self
    }
}

fn family_for_classification(
    classification: &WorthUiNodeReplacementClassification,
) -> WorthUiPlanNodeInputFamily {
    let kind = classification
        .candidate_kind()
        .or_else(|| classification.active_kind());
    match kind {
        Some(WorthUiIdentityMatchNodeKind::Import) => WorthUiPlanNodeInputFamily::ChildRange,
        Some(WorthUiIdentityMatchNodeKind::Component) => {
            WorthUiPlanNodeInputFamily::ComponentInvocation
        }
        Some(WorthUiIdentityMatchNodeKind::Surface) => WorthUiPlanNodeInputFamily::LayoutRegion,
        Some(WorthUiIdentityMatchNodeKind::Binding) => WorthUiPlanNodeInputFamily::QueryViewBinding,
        Some(WorthUiIdentityMatchNodeKind::Token) => WorthUiPlanNodeInputFamily::TokenStyle,
        None => WorthUiPlanNodeInputFamily::DiagnosticsRef,
    }
}

fn posture_for_query_rebind_entry(
    entry: &WorthUiQueryLiveRebindEntry,
) -> Option<WorthUiQueryBindingPosture> {
    match entry.outcome() {
        WorthUiQueryLiveRebindOutcome::Preserve(preservation) => {
            Some(preservation.preserved_posture().clone())
        }
        WorthUiQueryLiveRebindOutcome::Rebind(rebind) => Some(rebind.candidate_posture().clone()),
        WorthUiQueryLiveRebindOutcome::Retire(retirement) => {
            Some(retirement.active_posture().clone())
        }
        WorthUiQueryLiveRebindOutcome::Deny(denial) => denial
            .candidate_posture()
            .or_else(|| denial.active_posture())
            .cloned(),
    }
}

fn required_surfaces_for_query_rebind_entry(
    entry: &WorthUiQueryLiveRebindEntry,
) -> Vec<WorthUiQueryRebindRequiredSurface> {
    match entry.outcome() {
        WorthUiQueryLiveRebindOutcome::Rebind(rebind) => rebind.required_query_surfaces().to_vec(),
        _ => Vec::new(),
    }
}

fn preservation_receipt_for_query_rebind_entry(
    entry: &WorthUiQueryLiveRebindEntry,
) -> Option<String> {
    match entry.outcome() {
        WorthUiQueryLiveRebindOutcome::Preserve(preservation) => {
            Some(preservation.preservation_receipt().to_owned())
        }
        _ => None,
    }
}

fn egui_boundary_for_family(
    family: WorthUiPlanNodeInputFamily,
) -> Option<WorthUiEguiBoundaryInput> {
    match family {
        WorthUiPlanNodeInputFamily::ComponentInvocation => {
            Some(WorthUiEguiBoundaryInput::Component)
        }
        WorthUiPlanNodeInputFamily::LayoutRegion => Some(WorthUiEguiBoundaryInput::Surface),
        WorthUiPlanNodeInputFamily::QueryViewBinding => {
            Some(WorthUiEguiBoundaryInput::QueryBinding)
        }
        WorthUiPlanNodeInputFamily::TokenStyle => Some(WorthUiEguiBoundaryInput::Token),
        WorthUiPlanNodeInputFamily::DiagnosticsRef => Some(WorthUiEguiBoundaryInput::Diagnostics),
        WorthUiPlanNodeInputFamily::EguiBoundaryRef => Some(WorthUiEguiBoundaryInput::Diagnostics),
        _ => None,
    }
}

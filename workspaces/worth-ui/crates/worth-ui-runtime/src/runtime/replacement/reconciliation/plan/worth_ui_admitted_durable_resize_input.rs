use crate::capability::MosaicResizePermission;
use crate::declaration::stable_text_digest;
use crate::runtime::{WorthUiDurableStateFamilyId, WorthUiNodeLifecycleTransition};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiDurableResizeInputPosture {
    AdmittedPlanningTimeOnly,
    RemappedForChangedResizeLane,
    DeniedIncompatibleCarryForwardShape,
}

/// Reconciliation disposition retained for durable-state inspection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDurableResizeInputDisposition {
    identity_basis: String,
    authored_provenance_digest: Option<u64>,
    family_id: WorthUiDurableStateFamilyId,
    transition: WorthUiNodeLifecycleTransition,
    resize_permission: MosaicResizePermission,
    resize_contract_id: crate::capability::MosaicSizingContractId,
    resize_shape_digest: u64,
    posture: WorthUiDurableResizeInputPosture,
    planning_time_only: bool,
    identity_digest: u64,
}

pub(crate) struct WorthUiDurableResizeInputDispositionInput {
    pub identity_basis: String,
    pub authored_provenance_digest: Option<u64>,
    pub family_id: WorthUiDurableStateFamilyId,
    pub transition: WorthUiNodeLifecycleTransition,
    pub resize_permission: MosaicResizePermission,
    pub resize_contract_id: crate::capability::MosaicSizingContractId,
    pub resize_shape_digest: u64,
    pub posture: WorthUiDurableResizeInputPosture,
}

/// Move-only durable resize truth admitted by reconciliation itself.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAdmittedDurableResizeInput {
    disposition: WorthUiDurableResizeInputDisposition,
    authority_generation: u64,
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiAdmittedDurableResizeSourceFact {
    input: WorthUiAdmittedDurableResizeInput,
    extent: crate::runtime::UiResizeLogicalExtent,
    source_identity: u64,
    source_generation: u64,
    source_order: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiDurableResizeSourceAdmissionDenial {
    SourceOrderExhausted,
    InactiveReconciliation,
    ForeignReconciliationGeneration,
    InputNotActive,
}

#[derive(Debug)]
pub(crate) struct WorthUiDurableResizeSourceAuthority {
    generation: u64,
    next_order: u64,
    active_inputs: Vec<(u64, u64)>,
}

impl WorthUiDurableResizeInputDisposition {
    pub(crate) fn new(input: WorthUiDurableResizeInputDispositionInput) -> Self {
        let WorthUiDurableResizeInputDispositionInput {
            identity_basis,
            authored_provenance_digest,
            family_id,
            transition,
            resize_permission,
            resize_contract_id,
            resize_shape_digest,
            posture,
        } = input;
        let planning_time_only = true;
        let identity_digest = stable_text_digest("worth-ui.runtime.durable-resize-input")
            ^ stable_text_digest(&identity_basis).rotate_left(7)
            ^ authored_provenance_digest
                .unwrap_or_default()
                .rotate_left(9)
            ^ family_digest(&family_id).rotate_left(13)
            ^ transition_digest(transition).rotate_left(19)
            ^ resize_permission_digest(&resize_permission).rotate_left(23)
            ^ stable_text_digest(resize_contract_id.as_str()).rotate_left(27)
            ^ resize_shape_digest.rotate_left(31)
            ^ posture_digest(posture).rotate_left(29)
            ^ bool_digest(planning_time_only).rotate_left(37);
        Self {
            identity_basis,
            authored_provenance_digest,
            family_id,
            transition,
            resize_permission,
            resize_contract_id,
            resize_shape_digest,
            posture,
            planning_time_only,
            identity_digest,
        }
    }

    pub fn identity_basis(&self) -> &str {
        &self.identity_basis
    }
    pub fn family_id(&self) -> &WorthUiDurableStateFamilyId {
        &self.family_id
    }
    pub fn authored_provenance_digest(&self) -> Option<u64> {
        self.authored_provenance_digest
    }
    pub fn transition(&self) -> WorthUiNodeLifecycleTransition {
        self.transition
    }
    pub fn resize_permission(&self) -> &MosaicResizePermission {
        &self.resize_permission
    }
    pub fn resize_contract_id(&self) -> &crate::capability::MosaicSizingContractId {
        &self.resize_contract_id
    }
    pub fn resize_shape_digest(&self) -> u64 {
        self.resize_shape_digest
    }
    pub fn posture(&self) -> WorthUiDurableResizeInputPosture {
        self.posture
    }
    pub fn is_planning_time_only(&self) -> bool {
        self.planning_time_only
    }
    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}

impl WorthUiAdmittedDurableResizeInput {
    pub(super) fn from_reconciliation(
        disposition: WorthUiDurableResizeInputDisposition,
        authority_generation: u64,
    ) -> Self {
        Self {
            disposition,
            authority_generation,
        }
    }

    pub fn identity_basis(&self) -> &str {
        self.disposition.identity_basis()
    }
    pub fn family_id(&self) -> &WorthUiDurableStateFamilyId {
        self.disposition.family_id()
    }
    pub fn authored_provenance_digest(&self) -> Option<u64> {
        self.disposition.authored_provenance_digest()
    }
    pub fn transition(&self) -> WorthUiNodeLifecycleTransition {
        self.disposition.transition()
    }
    pub fn resize_permission(&self) -> &MosaicResizePermission {
        self.disposition.resize_permission()
    }
    pub fn posture(&self) -> WorthUiDurableResizeInputPosture {
        WorthUiDurableResizeInputPosture::AdmittedPlanningTimeOnly
    }
    pub fn is_planning_time_only(&self) -> bool {
        self.disposition.is_planning_time_only()
    }
    pub fn identity_digest(&self) -> u64 {
        self.disposition.identity_digest()
    }
    pub fn authority_generation(&self) -> u64 {
        self.authority_generation
    }
}

impl Default for WorthUiDurableResizeSourceAuthority {
    fn default() -> Self {
        Self {
            generation: 1,
            next_order: 1,
            active_inputs: Vec::new(),
        }
    }
}

impl WorthUiDurableResizeSourceAuthority {
    pub(in crate::runtime) fn prepare_successor(
        reconciliation: &crate::runtime::WorthUiDurableStateReconciliationPlan,
    ) -> Self {
        Self {
            active_inputs: reconciliation
                .durable_resize_inputs()
                .iter()
                .map(|input| (input.identity_digest(), input.authority_generation()))
                .collect(),
            generation: reconciliation.authority_generation(),
            next_order: 1,
        }
    }

    pub(crate) fn admit(
        &mut self,
        intent: crate::runtime::UiDurableResizeCommitIntent,
    ) -> Result<WorthUiAdmittedDurableResizeSourceFact, WorthUiDurableResizeSourceAdmissionDenial>
    {
        let input = intent.authority();
        if self.active_inputs.is_empty() {
            return Err(WorthUiDurableResizeSourceAdmissionDenial::InactiveReconciliation);
        }
        if input.authority_generation() != self.generation {
            return Err(WorthUiDurableResizeSourceAdmissionDenial::ForeignReconciliationGeneration);
        }
        if !self
            .active_inputs
            .contains(&(input.identity_digest(), input.authority_generation()))
        {
            return Err(WorthUiDurableResizeSourceAdmissionDenial::InputNotActive);
        }
        let order = self.next_order;
        self.next_order = self
            .next_order
            .checked_add(1)
            .ok_or(WorthUiDurableResizeSourceAdmissionDenial::SourceOrderExhausted)?;
        Ok(WorthUiAdmittedDurableResizeSourceFact {
            input: intent.authority().clone(),
            extent: intent.extent(),
            source_identity: order,
            source_generation: self.generation,
            source_order: order,
        })
    }
}

impl WorthUiAdmittedDurableResizeSourceFact {
    pub fn input(&self) -> &WorthUiAdmittedDurableResizeInput {
        &self.input
    }
    pub fn source_identity(&self) -> u64 {
        self.source_identity
    }
    pub fn source_generation(&self) -> u64 {
        self.source_generation
    }
    pub fn source_order(&self) -> u64 {
        self.source_order
    }
    pub fn extent(&self) -> crate::runtime::UiResizeLogicalExtent {
        self.extent
    }
}

#[cfg(test)]
impl Clone for WorthUiAdmittedDurableResizeSourceFact {
    fn clone(&self) -> Self {
        Self {
            input: self.input.clone(),
            extent: self.extent,
            source_identity: self.source_identity,
            source_generation: self.source_generation,
            source_order: self.source_order,
        }
    }
}

fn family_digest(family_id: &WorthUiDurableStateFamilyId) -> u64 {
    stable_text_digest(match family_id {
        WorthUiDurableStateFamilyId::FocusChain => "worth-ui.runtime.resize.family.focus-chain",
        WorthUiDurableStateFamilyId::ScrollAnchor => "worth-ui.runtime.resize.family.scroll-anchor",
        WorthUiDurableStateFamilyId::SelectionRange => {
            "worth-ui.runtime.resize.family.selection-range"
        }
        WorthUiDurableStateFamilyId::TextEditBuffer => {
            "worth-ui.runtime.resize.family.text-edit-buffer"
        }
        WorthUiDurableStateFamilyId::SplitterPosition => {
            "worth-ui.runtime.resize.family.splitter-position"
        }
        WorthUiDurableStateFamilyId::TabState => "worth-ui.runtime.resize.family.tab-state",
        WorthUiDurableStateFamilyId::PanelVisibility => {
            "worth-ui.runtime.resize.family.panel-visibility"
        }
        WorthUiDurableStateFamilyId::Custom(_) => "worth-ui.runtime.resize.family.custom",
    })
}

fn transition_digest(transition: WorthUiNodeLifecycleTransition) -> u64 {
    stable_text_digest(match transition {
        WorthUiNodeLifecycleTransition::Preserve => "worth-ui.runtime.resize.transition.preserve",
        WorthUiNodeLifecycleTransition::Move => "worth-ui.runtime.resize.transition.move",
        WorthUiNodeLifecycleTransition::Rebind => "worth-ui.runtime.resize.transition.rebind",
        WorthUiNodeLifecycleTransition::Replace => "worth-ui.runtime.resize.transition.replace",
        WorthUiNodeLifecycleTransition::Create => "worth-ui.runtime.resize.transition.create",
        WorthUiNodeLifecycleTransition::Drop => "worth-ui.runtime.resize.transition.drop",
        WorthUiNodeLifecycleTransition::LaneChange => {
            "worth-ui.runtime.resize.transition.lane-change"
        }
    })
}

fn resize_permission_digest(permission: &MosaicResizePermission) -> u64 {
    stable_text_digest(match permission {
        MosaicResizePermission::FixedByRuntime => {
            "worth-ui.runtime.resize.permission.fixed-by-runtime"
        }
        MosaicResizePermission::UserResizable => {
            "worth-ui.runtime.resize.permission.user-resizable"
        }
        MosaicResizePermission::ContentDriven => {
            "worth-ui.runtime.resize.permission.content-driven"
        }
        MosaicResizePermission::MissingForDiagnostics => {
            "worth-ui.runtime.resize.permission.missing"
        }
    })
}

fn posture_digest(posture: WorthUiDurableResizeInputPosture) -> u64 {
    stable_text_digest(match posture {
        WorthUiDurableResizeInputPosture::AdmittedPlanningTimeOnly => {
            "worth-ui.runtime.resize.posture.admitted"
        }
        WorthUiDurableResizeInputPosture::RemappedForChangedResizeLane => {
            "worth-ui.runtime.resize.posture.remapped"
        }
        WorthUiDurableResizeInputPosture::DeniedIncompatibleCarryForwardShape => {
            "worth-ui.runtime.resize.posture.denied"
        }
    })
}

fn bool_digest(value: bool) -> u64 {
    stable_text_digest(if value {
        "worth-ui.runtime.resize.bool.true"
    } else {
        "worth-ui.runtime.resize.bool.false"
    })
}

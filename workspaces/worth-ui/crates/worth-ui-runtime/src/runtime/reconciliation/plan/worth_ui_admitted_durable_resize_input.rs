use crate::capability::MosaicResizePermission;
use crate::declaration::stable_text_digest;
use crate::runtime::{WorthUiDurableStateFamilyId, WorthUiNodeLifecycleTransition};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiDurableResizeInputPosture {
    AdmittedPlanningTimeOnly,
    RemappedForChangedResizeLane,
    DeniedIncompatibleCarryForwardShape,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAdmittedDurableResizeInput {
    identity_basis: String,
    authored_provenance_digest: Option<u64>,
    family_id: WorthUiDurableStateFamilyId,
    transition: WorthUiNodeLifecycleTransition,
    resize_permission: MosaicResizePermission,
    posture: WorthUiDurableResizeInputPosture,
    planning_time_only: bool,
    identity_digest: u64,
}

impl WorthUiAdmittedDurableResizeInput {
    pub(crate) fn new(
        identity_basis: String,
        authored_provenance_digest: Option<u64>,
        family_id: WorthUiDurableStateFamilyId,
        transition: WorthUiNodeLifecycleTransition,
        resize_permission: MosaicResizePermission,
        posture: WorthUiDurableResizeInputPosture,
        planning_time_only: bool,
    ) -> Self {
        let identity_digest = stable_text_digest("worth-ui.runtime.durable-resize-input")
            ^ stable_text_digest(&identity_basis).rotate_left(7)
            ^ authored_provenance_digest
                .unwrap_or_default()
                .rotate_left(9)
            ^ family_digest(&family_id).rotate_left(13)
            ^ transition_digest(transition).rotate_left(19)
            ^ resize_permission_digest(&resize_permission).rotate_left(23)
            ^ posture_digest(posture).rotate_left(29)
            ^ bool_digest(planning_time_only).rotate_left(31);
        Self {
            identity_basis,
            authored_provenance_digest,
            family_id,
            transition,
            resize_permission,
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

    pub fn posture(&self) -> WorthUiDurableResizeInputPosture {
        self.posture
    }

    pub fn is_admitted(&self) -> bool {
        matches!(
            self.posture,
            WorthUiDurableResizeInputPosture::AdmittedPlanningTimeOnly
        )
    }

    pub fn is_planning_time_only(&self) -> bool {
        self.planning_time_only
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
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
    if value {
        stable_text_digest("worth-ui.runtime.resize.bool.true")
    } else {
        stable_text_digest("worth-ui.runtime.resize.bool.false")
    }
}

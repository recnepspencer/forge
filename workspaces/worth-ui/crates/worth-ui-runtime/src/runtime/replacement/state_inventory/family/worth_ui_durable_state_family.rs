use crate::runtime::{
    WorthUiDurableStateFamilyId, WorthUiDurableStateReplacementPolicy, WorthUiStateOwnerIdentity,
    WorthUiStatePersistencePosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDurableStateFamily {
    id: WorthUiDurableStateFamilyId,
    owner_identity: WorthUiStateOwnerIdentity,
    replacement_policy: WorthUiDurableStateReplacementPolicy,
    persistence_posture: WorthUiStatePersistencePosture,
    lane_constrained: bool,
    contract_digest: u64,
}

pub(in crate::runtime::replacement::state_inventory) struct WorthUiDurableStateFamilyDefinition {
    pub(super) id: WorthUiDurableStateFamilyId,
    pub(super) owner_identity: WorthUiStateOwnerIdentity,
    pub(super) replacement_policy: WorthUiDurableStateReplacementPolicy,
    pub(super) persistence_posture: WorthUiStatePersistencePosture,
    pub(super) lane_constrained: bool,
    pub(super) contract_digest: u64,
}

impl WorthUiDurableStateFamily {
    pub fn focus_chain() -> Self {
        Self::platform(
            WorthUiDurableStateFamilyId::FocusChain,
            WorthUiDurableStateReplacementPolicy::PreserveWhenNodeCarriesState,
            true,
        )
    }

    pub fn scroll_anchor() -> Self {
        Self::platform(
            WorthUiDurableStateFamilyId::ScrollAnchor,
            WorthUiDurableStateReplacementPolicy::ReconcileOnLaneChange,
            true,
        )
    }

    pub fn selection_range() -> Self {
        Self::platform(
            WorthUiDurableStateFamilyId::SelectionRange,
            WorthUiDurableStateReplacementPolicy::PreserveWhenNodeCarriesState,
            true,
        )
    }

    pub fn text_edit_buffer() -> Self {
        Self::platform(
            WorthUiDurableStateFamilyId::TextEditBuffer,
            WorthUiDurableStateReplacementPolicy::DropOnReplacement,
            true,
        )
    }

    pub fn splitter_position() -> Self {
        Self::platform(
            WorthUiDurableStateFamilyId::SplitterPosition,
            WorthUiDurableStateReplacementPolicy::ReconcileOnLaneChange,
            false,
        )
    }

    pub fn tab_state() -> Self {
        Self::platform(
            WorthUiDurableStateFamilyId::TabState,
            WorthUiDurableStateReplacementPolicy::ReconcileOnLaneChange,
            false,
        )
    }

    pub fn panel_visibility() -> Self {
        Self::platform(
            WorthUiDurableStateFamilyId::PanelVisibility,
            WorthUiDurableStateReplacementPolicy::ReconcileOnLaneChange,
            false,
        )
    }

    pub fn id(&self) -> &WorthUiDurableStateFamilyId {
        &self.id
    }

    pub fn replacement_policy(&self) -> WorthUiDurableStateReplacementPolicy {
        self.replacement_policy
    }

    pub fn contract_digest(&self) -> u64 {
        self.contract_digest
    }

    pub(super) fn from_admitted_definition(
        definition: WorthUiDurableStateFamilyDefinition,
    ) -> Self {
        Self {
            id: definition.id,
            owner_identity: definition.owner_identity,
            replacement_policy: definition.replacement_policy,
            persistence_posture: definition.persistence_posture,
            lane_constrained: definition.lane_constrained,
            contract_digest: definition.contract_digest,
        }
    }

    fn platform(
        id: WorthUiDurableStateFamilyId,
        replacement_policy: WorthUiDurableStateReplacementPolicy,
        lane_constrained: bool,
    ) -> Self {
        let contract_digest =
            crate::declaration::stable_text_digest(platform_contract_identity_basis(&id));
        let owner_identity =
            WorthUiStateOwnerIdentity::platform_state_family(platform_owner_identity_basis(&id));
        Self {
            id,
            owner_identity,
            replacement_policy,
            persistence_posture: WorthUiStatePersistencePosture::RuntimeOnly,
            lane_constrained,
            contract_digest,
        }
    }
}

fn platform_owner_identity_basis(id: &WorthUiDurableStateFamilyId) -> &'static str {
    match id {
        WorthUiDurableStateFamilyId::FocusChain => "worth-ui.platform.focus-chain",
        WorthUiDurableStateFamilyId::ScrollAnchor => "worth-ui.platform.scroll-anchor",
        WorthUiDurableStateFamilyId::SelectionRange => "worth-ui.platform.selection-range",
        WorthUiDurableStateFamilyId::TextEditBuffer => "worth-ui.platform.text-edit-buffer",
        WorthUiDurableStateFamilyId::SplitterPosition => "worth-ui.platform.splitter-position",
        WorthUiDurableStateFamilyId::TabState => "worth-ui.platform.tab-state",
        WorthUiDurableStateFamilyId::PanelVisibility => "worth-ui.platform.panel-visibility",
        WorthUiDurableStateFamilyId::Custom(_) => "worth-ui.platform.custom",
    }
}

fn platform_contract_identity_basis(id: &WorthUiDurableStateFamilyId) -> &'static str {
    match id {
        WorthUiDurableStateFamilyId::FocusChain => {
            "worth-ui.state.focus-chain|platform|preserve|runtime-only|lane-constrained"
        }
        WorthUiDurableStateFamilyId::ScrollAnchor => {
            "worth-ui.state.scroll-anchor|platform|reconcile-lane|runtime-only|lane-constrained"
        }
        WorthUiDurableStateFamilyId::SelectionRange => {
            "worth-ui.state.selection-range|platform|preserve|runtime-only|lane-constrained"
        }
        WorthUiDurableStateFamilyId::TextEditBuffer => {
            "worth-ui.state.text-edit-buffer|platform|drop|runtime-only|lane-constrained"
        }
        WorthUiDurableStateFamilyId::SplitterPosition => {
            "worth-ui.state.splitter-position|platform|reconcile-lane|runtime-only|lane-neutral"
        }
        WorthUiDurableStateFamilyId::TabState => {
            "worth-ui.state.tab-state|platform|reconcile-lane|runtime-only|lane-neutral"
        }
        WorthUiDurableStateFamilyId::PanelVisibility => {
            "worth-ui.state.panel-visibility|platform|reconcile-lane|runtime-only|lane-neutral"
        }
        WorthUiDurableStateFamilyId::Custom(_) => {
            unreachable!("custom families derive contract identity from admitted state slots")
        }
    }
}

use crate::runtime::{
    WorthUiDurableStateEligibility, WorthUiDurableStateFamilyHook, WorthUiDurableStateFamilyId,
    WorthUiDurableStateReplacementPolicy, WorthUiStateOwnerIdentity, WorthUiStateOwnershipClass,
    WorthUiStatePersistencePosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDurableStateFamily {
    id: WorthUiDurableStateFamilyId,
    owner_identity: WorthUiStateOwnerIdentity,
    replacement_policy: WorthUiDurableStateReplacementPolicy,
    persistence_posture: WorthUiStatePersistencePosture,
    lane_constrained: bool,
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

    pub fn owner_identity(&self) -> &WorthUiStateOwnerIdentity {
        &self.owner_identity
    }

    pub fn ownership_class(&self) -> WorthUiStateOwnershipClass {
        self.owner_identity.ownership_class()
    }

    pub fn replacement_policy(&self) -> WorthUiDurableStateReplacementPolicy {
        self.replacement_policy
    }

    pub fn persistence_posture(&self) -> WorthUiStatePersistencePosture {
        self.persistence_posture
    }

    pub fn eligibility(&self) -> WorthUiDurableStateEligibility {
        WorthUiDurableStateEligibility::Eligible
    }

    pub fn is_durable(&self) -> bool {
        self.eligibility() == WorthUiDurableStateEligibility::Eligible
    }

    pub fn is_lane_constrained(&self) -> bool {
        self.lane_constrained
    }

    pub(crate) fn from_validated_hook(hook: WorthUiDurableStateFamilyHook) -> Self {
        Self {
            id: hook.family_id().clone(),
            owner_identity: hook.owner_identity().expect("validated owner"),
            replacement_policy: hook.replacement_policy().expect("validated policy"),
            persistence_posture: hook.persistence_posture().expect("validated posture"),
            lane_constrained: hook.is_lane_constrained(),
        }
    }

    fn platform(
        id: WorthUiDurableStateFamilyId,
        replacement_policy: WorthUiDurableStateReplacementPolicy,
        lane_constrained: bool,
    ) -> Self {
        let owner_identity =
            WorthUiStateOwnerIdentity::platform_state_family(platform_owner_identity_basis(&id));
        Self {
            id,
            owner_identity,
            replacement_policy,
            persistence_posture: WorthUiStatePersistencePosture::RuntimeOnly,
            lane_constrained,
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

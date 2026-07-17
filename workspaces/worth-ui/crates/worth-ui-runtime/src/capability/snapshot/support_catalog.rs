use std::collections::{BTreeMap, BTreeSet};

use crate::capability::support::CapabilitySupportId;
use crate::capability::{
    CapabilitySupportKind, CapabilitySupportPosture, CommandId, CommandProjectionId, ComponentId,
    IconId, MosaicPlacementPolicyId, MosaicRegionKindId, MosaicSizingContractId, MosaicStateSlotId,
    RegistrationCandidate, SurfaceId, ThemeTokenId, ViewBindingId, COMMAND_FAMILY_NAME,
    COMMAND_PROJECTION_FAMILY_NAME, COMPONENT_FAMILY_NAME, ICON_FAMILY_NAME,
    MOSAIC_PLACEMENT_POLICY_FAMILY_NAME, MOSAIC_REGION_KIND_FAMILY_NAME,
    MOSAIC_SIZING_CONTRACT_FAMILY_NAME, MOSAIC_STATE_SLOT_FAMILY_NAME, SURFACE_FAMILY_NAME,
    THEME_TOKEN_FAMILY_NAME, VIEW_BINDING_FAMILY_NAME,
};

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub(crate) struct CapabilitySupportCatalog {
    posture_by_family_and_identity: BTreeMap<&'static str, BTreeMap<String, CapabilitySupportKind>>,
}

impl CapabilitySupportCatalog {
    #[cfg(test)]
    pub(crate) fn empty() -> Self {
        Self::default()
    }

    pub(crate) fn from_registration_candidates(candidates: &[RegistrationCandidate]) -> Self {
        let mut duplicate_keys = BTreeSet::new();
        let mut seen_keys = BTreeSet::new();

        for candidate in candidates {
            let candidate_key = (
                candidate.family_name(),
                candidate.identity_text().to_owned(),
            );
            if !seen_keys.insert(candidate_key.clone()) {
                duplicate_keys.insert(candidate_key);
            }
        }

        let mut posture_by_family_and_identity = BTreeMap::new();
        for candidate in candidates {
            let candidate_key = (
                candidate.family_name(),
                candidate.identity_text().to_owned(),
            );
            if duplicate_keys.contains(&candidate_key) {
                continue;
            }

            posture_by_family_and_identity
                .entry(candidate.family_name())
                .or_insert_with(BTreeMap::new)
                .insert(
                    candidate.identity_text().to_owned(),
                    candidate.support_kind(),
                );
        }

        Self {
            posture_by_family_and_identity,
        }
    }

    pub(crate) fn component_posture(
        &self,
        component_id: &ComponentId,
    ) -> Option<CapabilitySupportPosture<ComponentId>> {
        self.posture_for_component_family(COMPONENT_FAMILY_NAME, component_id)
    }

    pub(crate) fn command_posture(
        &self,
        command_id: &CommandId,
    ) -> Option<CapabilitySupportPosture<CommandId>> {
        self.posture_for_command_family(COMMAND_FAMILY_NAME, command_id)
    }

    pub(crate) fn command_projection_posture(
        &self,
        command_projection_id: &CommandProjectionId,
    ) -> Option<CapabilitySupportPosture<CommandProjectionId>> {
        self.posture_for_identity(COMMAND_PROJECTION_FAMILY_NAME, command_projection_id)
    }

    pub(crate) fn surface_posture(
        &self,
        surface_id: &SurfaceId,
    ) -> Option<CapabilitySupportPosture<SurfaceId>> {
        self.posture_for_surface_family(SURFACE_FAMILY_NAME, surface_id)
    }

    pub(crate) fn icon_posture(
        &self,
        icon_id: &IconId,
    ) -> Option<CapabilitySupportPosture<IconId>> {
        self.posture_for_identity(ICON_FAMILY_NAME, icon_id)
    }

    pub(crate) fn view_binding_posture(
        &self,
        view_binding_id: &ViewBindingId,
    ) -> Option<CapabilitySupportPosture<ViewBindingId>> {
        self.posture_for_view_binding_family(VIEW_BINDING_FAMILY_NAME, view_binding_id)
    }

    pub(crate) fn theme_token_posture(
        &self,
        theme_token_id: &ThemeTokenId,
    ) -> Option<CapabilitySupportPosture<ThemeTokenId>> {
        self.posture_for_theme_token_family(THEME_TOKEN_FAMILY_NAME, theme_token_id)
    }

    pub(crate) fn mosaic_region_posture(
        &self,
        region_id: &MosaicRegionKindId,
    ) -> Option<CapabilitySupportPosture<MosaicRegionKindId>> {
        self.posture_for_identity(MOSAIC_REGION_KIND_FAMILY_NAME, region_id)
    }

    pub(crate) fn mosaic_placement_posture(
        &self,
        placement_id: &MosaicPlacementPolicyId,
    ) -> Option<CapabilitySupportPosture<MosaicPlacementPolicyId>> {
        self.posture_for_identity(MOSAIC_PLACEMENT_POLICY_FAMILY_NAME, placement_id)
    }

    pub(crate) fn mosaic_sizing_posture(
        &self,
        sizing_id: &MosaicSizingContractId,
    ) -> Option<CapabilitySupportPosture<MosaicSizingContractId>> {
        self.posture_for_identity(MOSAIC_SIZING_CONTRACT_FAMILY_NAME, sizing_id)
    }

    pub(crate) fn mosaic_state_slot_posture(
        &self,
        state_slot_id: &MosaicStateSlotId,
    ) -> Option<CapabilitySupportPosture<MosaicStateSlotId>> {
        self.posture_for_identity(MOSAIC_STATE_SLOT_FAMILY_NAME, state_slot_id)
    }

    fn posture_for_component_family(
        &self,
        family_name: &'static str,
        component_id: &ComponentId,
    ) -> Option<CapabilitySupportPosture<ComponentId>> {
        self.lookup_support_kind(family_name, component_id.as_str())
            .map(|support_kind| support_posture_for_component(component_id.clone(), support_kind))
    }

    fn posture_for_command_family(
        &self,
        family_name: &'static str,
        command_id: &CommandId,
    ) -> Option<CapabilitySupportPosture<CommandId>> {
        self.lookup_support_kind(family_name, command_id.as_str())
            .map(|support_kind| support_posture_for_command(command_id.clone(), support_kind))
    }

    fn posture_for_surface_family(
        &self,
        family_name: &'static str,
        surface_id: &SurfaceId,
    ) -> Option<CapabilitySupportPosture<SurfaceId>> {
        self.lookup_support_kind(family_name, surface_id.as_str())
            .map(|support_kind| support_posture_for_surface(surface_id.clone(), support_kind))
    }

    fn posture_for_view_binding_family(
        &self,
        family_name: &'static str,
        view_binding_id: &ViewBindingId,
    ) -> Option<CapabilitySupportPosture<ViewBindingId>> {
        self.lookup_support_kind(family_name, view_binding_id.as_str())
            .map(|support_kind| {
                support_posture_for_view_binding(view_binding_id.clone(), support_kind)
            })
    }

    fn posture_for_theme_token_family(
        &self,
        family_name: &'static str,
        theme_token_id: &ThemeTokenId,
    ) -> Option<CapabilitySupportPosture<ThemeTokenId>> {
        self.lookup_support_kind(family_name, theme_token_id.as_str())
            .map(|support_kind| {
                support_posture_for_theme_token(theme_token_id.clone(), support_kind)
            })
    }

    fn lookup_support_kind(
        &self,
        family_name: &'static str,
        identity_text: &str,
    ) -> Option<CapabilitySupportKind> {
        self.posture_by_family_and_identity
            .get(family_name)
            .and_then(|posture_by_identity| posture_by_identity.get(identity_text))
            .copied()
    }

    fn posture_for_identity<T>(
        &self,
        family_name: &'static str,
        identity: &T,
    ) -> Option<CapabilitySupportPosture<T>>
    where
        T: Clone + CapabilitySupportId + CapabilityIdentityText,
    {
        self.lookup_support_kind(family_name, identity.identity_text())
            .map(|support_kind| support_posture_for_identity(identity.clone(), support_kind))
    }
}

fn support_posture_for_component(
    component_id: ComponentId,
    support_kind: CapabilitySupportKind,
) -> CapabilitySupportPosture<ComponentId> {
    match support_kind {
        CapabilitySupportKind::Admitted => CapabilitySupportPosture::admitted(component_id),
        CapabilitySupportKind::Deferred => CapabilitySupportPosture::deferred(component_id),
        CapabilitySupportKind::Unsupported => CapabilitySupportPosture::unsupported(component_id),
        CapabilitySupportKind::PlatformInternal => {
            CapabilitySupportPosture::platform_internal(component_id)
        }
    }
}

fn support_posture_for_command(
    command_id: CommandId,
    support_kind: CapabilitySupportKind,
) -> CapabilitySupportPosture<CommandId> {
    match support_kind {
        CapabilitySupportKind::Admitted => CapabilitySupportPosture::admitted(command_id),
        CapabilitySupportKind::Deferred => CapabilitySupportPosture::deferred(command_id),
        CapabilitySupportKind::Unsupported => CapabilitySupportPosture::unsupported(command_id),
        CapabilitySupportKind::PlatformInternal => {
            CapabilitySupportPosture::platform_internal(command_id)
        }
    }
}

fn support_posture_for_surface(
    surface_id: SurfaceId,
    support_kind: CapabilitySupportKind,
) -> CapabilitySupportPosture<SurfaceId> {
    match support_kind {
        CapabilitySupportKind::Admitted => CapabilitySupportPosture::admitted(surface_id),
        CapabilitySupportKind::Deferred => CapabilitySupportPosture::deferred(surface_id),
        CapabilitySupportKind::Unsupported => CapabilitySupportPosture::unsupported(surface_id),
        CapabilitySupportKind::PlatformInternal => {
            CapabilitySupportPosture::platform_internal(surface_id)
        }
    }
}

fn support_posture_for_view_binding(
    view_binding_id: ViewBindingId,
    support_kind: CapabilitySupportKind,
) -> CapabilitySupportPosture<ViewBindingId> {
    match support_kind {
        CapabilitySupportKind::Admitted => CapabilitySupportPosture::admitted(view_binding_id),
        CapabilitySupportKind::Deferred => CapabilitySupportPosture::deferred(view_binding_id),
        CapabilitySupportKind::Unsupported => {
            CapabilitySupportPosture::unsupported(view_binding_id)
        }
        CapabilitySupportKind::PlatformInternal => {
            CapabilitySupportPosture::platform_internal(view_binding_id)
        }
    }
}

fn support_posture_for_theme_token(
    theme_token_id: ThemeTokenId,
    support_kind: CapabilitySupportKind,
) -> CapabilitySupportPosture<ThemeTokenId> {
    match support_kind {
        CapabilitySupportKind::Admitted => CapabilitySupportPosture::admitted(theme_token_id),
        CapabilitySupportKind::Deferred => CapabilitySupportPosture::deferred(theme_token_id),
        CapabilitySupportKind::Unsupported => CapabilitySupportPosture::unsupported(theme_token_id),
        CapabilitySupportKind::PlatformInternal => {
            CapabilitySupportPosture::platform_internal(theme_token_id)
        }
    }
}

fn support_posture_for_identity<T: Clone + CapabilitySupportId>(
    identity: T,
    support_kind: CapabilitySupportKind,
) -> CapabilitySupportPosture<T> {
    match support_kind {
        CapabilitySupportKind::Admitted => CapabilitySupportPosture::admitted(identity),
        CapabilitySupportKind::Deferred => CapabilitySupportPosture::deferred(identity),
        CapabilitySupportKind::Unsupported => CapabilitySupportPosture::unsupported(identity),
        CapabilitySupportKind::PlatformInternal => {
            CapabilitySupportPosture::platform_internal(identity)
        }
    }
}

trait CapabilityIdentityText {
    fn identity_text(&self) -> &str;
}

impl CapabilityIdentityText for MosaicRegionKindId {
    fn identity_text(&self) -> &str {
        self.as_str()
    }
}

impl CapabilityIdentityText for CommandProjectionId {
    fn identity_text(&self) -> &str {
        self.as_str()
    }
}

impl CapabilityIdentityText for IconId {
    fn identity_text(&self) -> &str {
        self.as_str()
    }
}

impl CapabilityIdentityText for MosaicPlacementPolicyId {
    fn identity_text(&self) -> &str {
        self.as_str()
    }
}

impl CapabilityIdentityText for MosaicSizingContractId {
    fn identity_text(&self) -> &str {
        self.as_str()
    }
}

impl CapabilityIdentityText for MosaicStateSlotId {
    fn identity_text(&self) -> &str {
        self.as_str()
    }
}

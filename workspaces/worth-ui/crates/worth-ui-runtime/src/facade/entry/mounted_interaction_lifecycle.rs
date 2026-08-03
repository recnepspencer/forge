use super::WorthUiActiveApplicationSession;
use crate::facade::mounted::{
    UiHostSurfacePresentationMode, UiMountedIdentityDenial, UiMountedInstanceIdentity,
    UiSurfaceBindingGeneration, UiSurfaceBindingIdentityView, UiSurfaceBindingProfile,
};
use crate::runtime::interaction::{
    UiInteractionLifecycleSettlementReceipt, UiInteractionLifecycleStopReason,
};

/// Successful surface rebind plus the exact gestures retired by that boundary.
#[derive(Debug, Eq, PartialEq)]
pub struct UiSurfaceRebindInteractionReceipt {
    binding: UiSurfaceBindingIdentityView,
    interaction: UiInteractionLifecycleSettlementReceipt,
}

/// A rebind denial that preserves settlement if deregistration already occurred.
#[derive(Debug, Eq, PartialEq)]
pub enum UiSurfaceRebindInteractionDenial {
    BeforeMutation(UiMountedIdentityDenial),
    AfterInteractionSettlement {
        denial: UiMountedIdentityDenial,
        interaction: Box<UiInteractionLifecycleSettlementReceipt>,
    },
}

/// SUPPORT AUTHORITY for observing interaction settlement at mounted mutations.
pub trait WorthUiMountedInteractionLifecycleCertificationExt {
    fn unmount_instance_with_interaction_receipt(
        &mut self,
        identity: UiMountedInstanceIdentity,
    ) -> Result<UiInteractionLifecycleSettlementReceipt, UiMountedIdentityDenial>;

    fn rebind_host_surface_with_interaction_receipt(
        &mut self,
        binding: UiSurfaceBindingGeneration,
        mode: UiHostSurfacePresentationMode,
        profile: UiSurfaceBindingProfile,
    ) -> Result<UiSurfaceRebindInteractionReceipt, UiSurfaceRebindInteractionDenial>;
}

impl WorthUiActiveApplicationSession {
    pub(crate) fn unmount_instance_with_interaction_receipt(
        &mut self,
        identity: UiMountedInstanceIdentity,
    ) -> Result<UiInteractionLifecycleSettlementReceipt, UiMountedIdentityDenial> {
        self.mounted.unmount_instance(identity)?;
        Ok(self.interaction.cancel_instance(
            identity,
            UiInteractionLifecycleStopReason::MountedInstanceRemoved,
        ))
    }

    pub(crate) fn rebind_host_surface_with_interaction_receipt(
        &mut self,
        binding: UiSurfaceBindingGeneration,
        mode: UiHostSurfacePresentationMode,
        profile: UiSurfaceBindingProfile,
    ) -> Result<UiSurfaceRebindInteractionReceipt, UiSurfaceRebindInteractionDenial> {
        let semantic_surface = self
            .mounted
            .deregister_host_surface(&self.host_session, binding)
            .map_err(UiSurfaceRebindInteractionDenial::BeforeMutation)?;
        let interaction = self
            .interaction
            .cancel_binding(binding, UiInteractionLifecycleStopReason::SurfaceRebound);
        match self.mounted.register_host_surface(
            &self.host_session,
            semantic_surface,
            mode,
            profile,
        ) {
            Ok(binding) => Ok(UiSurfaceRebindInteractionReceipt {
                binding,
                interaction,
            }),
            Err(denial) => Err(
                UiSurfaceRebindInteractionDenial::AfterInteractionSettlement {
                    denial,
                    interaction: Box::new(interaction),
                },
            ),
        }
    }
}

impl WorthUiMountedInteractionLifecycleCertificationExt for WorthUiActiveApplicationSession {
    fn unmount_instance_with_interaction_receipt(
        &mut self,
        identity: UiMountedInstanceIdentity,
    ) -> Result<UiInteractionLifecycleSettlementReceipt, UiMountedIdentityDenial> {
        WorthUiActiveApplicationSession::unmount_instance_with_interaction_receipt(self, identity)
    }

    fn rebind_host_surface_with_interaction_receipt(
        &mut self,
        binding: UiSurfaceBindingGeneration,
        mode: UiHostSurfacePresentationMode,
        profile: UiSurfaceBindingProfile,
    ) -> Result<UiSurfaceRebindInteractionReceipt, UiSurfaceRebindInteractionDenial> {
        WorthUiActiveApplicationSession::rebind_host_surface_with_interaction_receipt(
            self, binding, mode, profile,
        )
    }
}

impl UiSurfaceRebindInteractionReceipt {
    pub const fn binding(&self) -> UiSurfaceBindingIdentityView {
        self.binding
    }

    pub const fn interaction(&self) -> &UiInteractionLifecycleSettlementReceipt {
        &self.interaction
    }
}

impl UiSurfaceRebindInteractionDenial {
    pub const fn mounted_denial(&self) -> UiMountedIdentityDenial {
        match self {
            Self::BeforeMutation(denial) | Self::AfterInteractionSettlement { denial, .. } => {
                *denial
            }
        }
    }
}

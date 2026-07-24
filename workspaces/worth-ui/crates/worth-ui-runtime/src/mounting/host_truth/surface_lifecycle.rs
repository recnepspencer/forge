use worth_ui_host_contract::{
    UiHostSurfaceBaselineReceipt, UiHostSurfaceDeregistrationOutcome,
    UiHostSurfaceRegistrationOutcome, UiHostSurfaceRegistrationRequest, UiSemanticSurfaceIdentity,
};

use super::binding_truth::UiMountedHostTruthCoordinator;
use super::registration_attempt::UiHostTruthNativeLifecycleKind;
use crate::facade::UiHostEffectPort;
use crate::mounting::identity_state::surface_lifecycle::map_registration_denial;
use crate::mounting::UiMountedIdentityDenial;

impl UiMountedHostTruthCoordinator {
    pub(crate) fn recover_surface_effect(
        &mut self,
        host: UiHostEffectPort<'_>,
        surface: UiSemanticSurfaceIdentity,
    ) -> Result<(), UiMountedIdentityDenial> {
        let blocked = self
            .blocked
            .values()
            .find(|record| {
                record.semantic_surface() == surface
                    && record.native_lifecycle_obligation().is_some()
            })
            .copied()
            .ok_or(UiMountedIdentityDenial::SurfaceRequiresReconciliation)?;
        let obligation = blocked
            .native_lifecycle_obligation()
            .ok_or(UiMountedIdentityDenial::SurfaceRequiresReconciliation)?;
        match obligation.kind() {
            UiHostTruthNativeLifecycleKind::Registration => {
                self.recover_registration_effect(host, obligation.request())
            }
            UiHostTruthNativeLifecycleKind::Deregistration => {
                self.recover_deregistration_effect(host, obligation.request())
            }
        }
    }

    pub(crate) fn register_surface(
        &mut self,
        host: UiHostEffectPort<'_>,
        request: UiHostSurfaceRegistrationRequest,
    ) -> Result<UiHostSurfaceBaselineReceipt, UiMountedIdentityDenial> {
        if self.surface_has_indeterminate_native_lifecycle(request.semantic_surface_identity()) {
            return Err(UiMountedIdentityDenial::SurfaceRequiresReconciliation);
        }
        match host.adapter().register_surface(host.authority(), request) {
            UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(denial) => {
                Err(map_registration_denial(denial))
            }
            UiHostSurfaceRegistrationOutcome::Registered(receipt)
                if receipt.registration() == request =>
            {
                self.known_empty
                    .insert(request.binding_generation(), receipt);
                Ok(receipt)
            }
            UiHostSurfaceRegistrationOutcome::Registered(_)
            | UiHostSurfaceRegistrationOutcome::RegistrationIndeterminate(_) => {
                self.block_native_lifecycle(UiHostTruthNativeLifecycleKind::Registration, request);
                Err(UiMountedIdentityDenial::HostSurfaceTruthIndeterminate)
            }
        }
    }

    pub(crate) fn deregister_surface(
        &mut self,
        host: UiHostEffectPort<'_>,
        request: UiHostSurfaceRegistrationRequest,
    ) -> Result<(), UiMountedIdentityDenial> {
        if let Some(blocked) = self.blocked.get(&request.binding_generation()).copied() {
            if blocked.native_lifecycle_obligation().is_some() {
                return Err(UiMountedIdentityDenial::SurfaceRequiresReconciliation);
            }
        } else if !self.known_empty.contains_key(&request.binding_generation()) {
            return Err(UiMountedIdentityDenial::SurfaceRequiresReconciliation);
        }
        match host.adapter().deregister_surface(host.authority(), request) {
            UiHostSurfaceDeregistrationOutcome::RejectedBeforeEffects(denial) => {
                Err(map_registration_denial(denial))
            }
            UiHostSurfaceDeregistrationOutcome::Deregistered(receipt)
                if receipt.host_session_identity() == request.host_session_identity()
                    && receipt.host_surface_identity() == request.host_surface_identity() =>
            {
                self.known_empty.remove(&request.binding_generation());
                self.clear_native_lifecycle(request.binding_generation());
                Ok(())
            }
            UiHostSurfaceDeregistrationOutcome::Deregistered(_)
            | UiHostSurfaceDeregistrationOutcome::DeregistrationIndeterminate(_) => {
                self.block_native_lifecycle(
                    UiHostTruthNativeLifecycleKind::Deregistration,
                    request,
                );
                Err(UiMountedIdentityDenial::HostSurfaceTruthIndeterminate)
            }
        }
    }

    fn recover_registration_effect(
        &mut self,
        host: UiHostEffectPort<'_>,
        request: UiHostSurfaceRegistrationRequest,
    ) -> Result<(), UiMountedIdentityDenial> {
        match host.adapter().deregister_surface(host.authority(), request) {
            UiHostSurfaceDeregistrationOutcome::Deregistered(receipt)
                if receipt.host_session_identity() == request.host_session_identity()
                    && receipt.host_surface_identity() == request.host_surface_identity() =>
            {
                self.known_empty.remove(&request.binding_generation());
                self.clear_native_lifecycle(request.binding_generation());
                Ok(())
            }
            UiHostSurfaceDeregistrationOutcome::RejectedBeforeEffects(_)
            | UiHostSurfaceDeregistrationOutcome::Deregistered(_)
            | UiHostSurfaceDeregistrationOutcome::DeregistrationIndeterminate(_) => {
                Err(UiMountedIdentityDenial::HostSurfaceTruthIndeterminate)
            }
        }
    }

    fn recover_deregistration_effect(
        &mut self,
        host: UiHostEffectPort<'_>,
        request: UiHostSurfaceRegistrationRequest,
    ) -> Result<(), UiMountedIdentityDenial> {
        match host.adapter().register_surface(host.authority(), request) {
            UiHostSurfaceRegistrationOutcome::Registered(receipt)
                if receipt.registration() == request =>
            {
                self.known_empty
                    .insert(request.binding_generation(), receipt);
                self.clear_native_lifecycle(request.binding_generation());
                Ok(())
            }
            UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(_)
            | UiHostSurfaceRegistrationOutcome::Registered(_)
            | UiHostSurfaceRegistrationOutcome::RegistrationIndeterminate(_) => {
                Err(UiMountedIdentityDenial::HostSurfaceTruthIndeterminate)
            }
        }
    }
}

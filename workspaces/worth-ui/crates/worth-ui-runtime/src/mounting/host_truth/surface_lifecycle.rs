use worth_ui_host_contract::{
    UiHostSurfaceBaselineIdentity, UiHostSurfaceDeregistrationOutcome,
    UiHostSurfaceRegistrationOutcome, UiHostSurfaceRegistrationRequest, UiSemanticSurfaceIdentity,
};

use super::binding_truth::UiMountedHostTruthCoordinator;
use super::registration_attempt::UiHostTruthNativeLifecycleKind;
use crate::facade::UiHostEffectPort;
use crate::mounting::identity_state::surface_lifecycle::map_registration_denial;
use crate::mounting::UiMountedIdentityDenial;

/// Move-only proof that this owner observed one successful known-empty
/// registration. Construction remains private to the host-truth response
/// transition so a copied request cannot mint another receipt.
#[must_use]
pub(super) struct UiMountedSurfaceBaselineReceipt {
    identity: UiHostSurfaceBaselineIdentity,
}

impl UiMountedSurfaceBaselineReceipt {
    pub(super) fn identity(&self) -> UiHostSurfaceBaselineIdentity {
        self.identity
    }
}

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
    ) -> Result<UiHostSurfaceBaselineIdentity, UiMountedIdentityDenial> {
        if self.surface_has_indeterminate_native_lifecycle(request.semantic_surface_identity()) {
            return Err(UiMountedIdentityDenial::SurfaceRequiresReconciliation);
        }
        if self.known_empty.contains_key(&request.binding_generation()) {
            return Err(UiMountedIdentityDenial::SurfaceAlreadyBound);
        }
        match host.adapter().register_surface(host.authority(), request) {
            UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(denial) => {
                Err(map_registration_denial(denial))
            }
            UiHostSurfaceRegistrationOutcome::RegisteredKnownEmpty => {
                let receipt = UiMountedSurfaceBaselineReceipt {
                    identity: request.baseline_identity(),
                };
                let identity = receipt.identity();
                self.known_empty
                    .insert(request.binding_generation(), receipt);
                Ok(identity)
            }
            UiHostSurfaceRegistrationOutcome::RegistrationIndeterminate(_) => {
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
            UiHostSurfaceRegistrationOutcome::RegisteredKnownEmpty => {
                let receipt = UiMountedSurfaceBaselineReceipt {
                    identity: request.baseline_identity(),
                };
                self.known_empty
                    .insert(request.binding_generation(), receipt);
                self.clear_native_lifecycle(request.binding_generation());
                Ok(())
            }
            UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(_)
            | UiHostSurfaceRegistrationOutcome::RegistrationIndeterminate(_) => {
                Err(UiMountedIdentityDenial::HostSurfaceTruthIndeterminate)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{UiMountedHostTruthCoordinator, UiMountedSurfaceBaselineReceipt};
    use worth_ui_host_contract::{
        UiHostProtocolContract, UiHostProtocolNegotiation, UiHostSurfaceIdentity,
        UiHostSurfacePresentationMode, UiHostSurfaceRegistrationInput,
        UiHostSurfaceRegistrationRequest, UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
        WorthUiHostCapabilityObservationGeneration,
    };

    #[test]
    fn live_baseline_receipt_is_required_and_removal_closes_admission() {
        let protocol = match UiHostProtocolContract::current().negotiate() {
            UiHostProtocolNegotiation::Compatible(protocol) => protocol,
            UiHostProtocolNegotiation::Incompatible(_) => panic!("current protocol must agree"),
        };
        let request =
            UiHostSurfaceRegistrationRequest::from_runtime(UiHostSurfaceRegistrationInput {
                host_session_identity: 1,
                semantic_surface_identity: UiSemanticSurfaceIdentity::mint_unbound().unwrap(),
                host_surface_identity: UiHostSurfaceIdentity::mint_unbound().unwrap(),
                binding_generation: UiSurfaceBindingGeneration::mint_unbound().unwrap(),
                protocol,
                capability_generation: WorthUiHostCapabilityObservationGeneration::new(1),
                capability_profile_digest: 7,
                presentation_mode: UiHostSurfacePresentationMode::NativeDisplay,
            });
        let baseline = request.baseline_identity();
        let binding = request.binding_generation();
        let mut truth = UiMountedHostTruthCoordinator::default();
        assert!(!truth.has_live_baseline(binding, baseline));
        truth.known_empty.insert(
            binding,
            UiMountedSurfaceBaselineReceipt { identity: baseline },
        );
        assert!(truth.has_live_baseline(binding, baseline));
        let receipt = truth.known_empty.remove(&binding).unwrap();
        assert_eq!(receipt.identity(), baseline);
        assert!(!truth.has_live_baseline(binding, baseline));
    }
}

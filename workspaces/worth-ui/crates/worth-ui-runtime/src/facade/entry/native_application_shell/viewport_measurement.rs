use super::WorthUiNativeApplicationShell;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct UiNativeViewportBasis {
    client_physical_extent: [u32; 2],
    scale_factor_milli: u32,
    binding: crate::mounting::UiSurfaceBindingGeneration,
}

struct UiPendingNativeViewportMeasurements {
    basis: UiNativeViewportBasis,
    capability: crate::facade::WorthUiHostMeasurementCapability,
    inputs: Vec<crate::facade::WorthUiHostMeasurementSessionInput>,
}

impl WorthUiNativeApplicationShell {
    pub(crate) fn observe_native_viewport_readiness(
        &mut self,
        client_physical_extent: [u32; 2],
        scale_factor_milli: u32,
        submit_successor: bool,
    ) {
        let basis = UiNativeViewportBasis {
            client_physical_extent,
            scale_factor_milli,
            binding: self.binding,
        };
        let changed = self.observed_viewport_basis != Some(basis);
        self.observed_viewport_basis = Some(basis);
        if changed && submit_successor && !self.viewport_measurement_authority.is_empty() {
            self.pending_viewport_basis = Some(basis);
        }
    }

    pub(super) fn observe_native_viewport_binding_successor(&mut self) {
        let Some(observed) = self.observed_viewport_basis else {
            return;
        };
        self.observe_native_viewport_readiness(
            observed.client_physical_extent,
            self.scale_factor_milli,
            true,
        );
    }

    pub(super) fn settle_pending_native_viewport_measurements(
        &mut self,
    ) -> Result<
        (),
        super::super::mounted_application_presentation::UiMountedHostMeasurementSettlementStop,
    > {
        let Some(pending) = self.pending_native_viewport_measurements() else {
            return Ok(());
        };
        let basis = pending.basis;
        self.session
            .settle_mounted_host_measurements(Some((pending.capability, pending.inputs)))?;
        if self.pending_viewport_basis == Some(basis) {
            self.pending_viewport_basis = None;
        }
        Ok(())
    }

    fn pending_native_viewport_measurements(&self) -> Option<UiPendingNativeViewportMeasurements> {
        let basis = self.pending_viewport_basis?;
        if self.viewport_measurement_authority.is_empty() {
            return None;
        }
        let capability = self.session.host_measurement_capability();
        let assumptions = crate::host::UiHostMeasurementAssumptionProfile::from_capability_report(
            capability.capability_report(),
            1,
            2,
            3,
            4,
        );
        let inputs = self
            .viewport_measurement_authority
            .iter()
            .copied()
            .map(|authority| {
                crate::facade::WorthUiHostMeasurementSessionInput::new(
                    authority.request(),
                    worth_ui_host_contract::UiMeasurementEvidenceFamily::ViewportExtent,
                    crate::host::UiHostMeasurementNeed::ViewportExtent(
                        worth_ui_host_contract::UiViewportExtentRequest,
                    ),
                    authority.evidence_generation(),
                    crate::host::UiHostMeasurementNormalizationContext::viewport_logical_exact(
                        assumptions,
                    ),
                )
            })
            .collect::<Vec<_>>();
        Some(UiPendingNativeViewportMeasurements {
            basis,
            capability,
            inputs,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::UiNativeViewportBasis;
    use crate::certification_support::ScriptedPresentationHost;
    use crate::runtime::tests::active_application_session_test_support::source_backed_component_app_with_host_and_viewport_allocation;

    #[test]
    fn same_physical_extent_with_new_scale_and_binding_remeasures_before_projection() {
        let host = ScriptedPresentationHost::native_display();
        let mut shell = source_backed_component_app_with_host_and_viewport_allocation(host.clone())
            .launch_native_surface()
            .expect("native viewport shell should launch");
        let baseline_calls = host.viewport_measurement_calls();
        shell.observe_native_viewport_readiness([800, 600], 1_000, false);

        host.set_viewport_extent([400.0, 300.0]);
        shell
            .rebind_native_surface_scale(2_000)
            .expect("scale successor should rebind the native surface");
        shell.observe_native_viewport_readiness([800, 600], 2_000, true);
        let pending = shell
            .pending_viewport_basis
            .expect("complete basis change must schedule measurement");
        assert_eq!(pending.client_physical_extent, [800, 600]);
        assert_eq!(pending.scale_factor_milli, 2_000);
        assert_eq!(pending.binding, shell.binding);
        host.push_native_display_presented();

        assert!(shell.present_frame(2, 0).is_ok());
        assert_eq!(host.viewport_measurement_calls(), baseline_calls + 1);
        assert_eq!(shell.pending_viewport_basis, None);
        assert_eq!(
            shell.observed_viewport_basis,
            Some(UiNativeViewportBasis {
                client_physical_extent: [800, 600],
                scale_factor_milli: 2_000,
                binding: shell.binding,
            })
        );
    }

    #[test]
    fn denied_viewport_settlement_retains_exact_basis_and_retries_before_frame_effects() {
        let host = ScriptedPresentationHost::native_display();
        let mut shell = source_backed_component_app_with_host_and_viewport_allocation(host.clone())
            .launch_native_surface()
            .expect("native viewport shell should launch");
        let baseline_calls = host.viewport_measurement_calls();
        host.push_in_flight(
            vec![crate::certification_support::ScriptedSurfaceCompletion::Pending],
            crate::facade::mounted::UiHostSurfaceCancellationOutcome::CancelledBeforeEffects,
        );
        let Ok(crate::facade::mounted::UiMountedFrameOutcome::InFlight(in_flight)) =
            shell.present_frame(1, 0)
        else {
            panic!("predecessor frame must remain in flight")
        };
        shell.observe_native_viewport_readiness([800, 600], 1_000, false);
        shell.observe_native_viewport_readiness([960, 600], 1_000, true);
        let pending = shell
            .pending_viewport_basis
            .expect("extent successor must schedule measurement");

        assert!(shell.present_frame(2, 1).is_err());
        assert_eq!(host.presentation_calls(), 1);
        assert_eq!(shell.pending_viewport_basis, Some(pending));

        let _ = shell.cancel_mounted_presentation(in_flight);
        host.set_viewport_extent([960.0, 600.0]);
        host.push_native_display_presented();
        assert!(shell.present_frame(3, 2).is_ok());
        assert_eq!(host.presentation_calls(), 2);
        assert_eq!(host.viewport_measurement_calls(), baseline_calls + 1);
        assert_eq!(shell.pending_viewport_basis, None);
    }
}

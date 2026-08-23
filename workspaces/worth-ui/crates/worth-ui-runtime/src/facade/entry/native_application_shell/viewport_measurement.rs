use super::WorthUiNativeApplicationShell;

impl WorthUiNativeApplicationShell {
    pub(crate) fn observe_native_viewport_readiness(
        &mut self,
        client_physical_size: [u32; 2],
        submit_successor: bool,
    ) {
        let changed = self.client_physical_size != Some(client_physical_size);
        self.client_physical_size = Some(client_physical_size);
        self.viewport_measurement_pending |=
            changed && submit_successor && !self.viewport_measurement_authority.is_empty();
    }

    pub(crate) fn commit_pending_native_viewport_measurement(&mut self) -> Result<(), ()> {
        if !self.viewport_measurement_pending {
            return Ok(());
        }
        if self.viewport_measurement_authority.is_empty() {
            self.viewport_measurement_pending = false;
            return Ok(());
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
        let mut collection_denial = false;
        let completion = self
            .session
            .execute_framework_turn(|turn| {
                turn.host_measurement(|source| {
                    for input in inputs {
                        collection_denial |= source
                            .collect_and_submit_capability(&capability, input)
                            .is_err();
                    }
                });
            })
            .map_err(|_| ())?;
        if collection_denial {
            return Err(());
        }
        let settled = matches!(
            &completion.completion,
            crate::runtime::WorthUiFrameworkTurnCompletion::ViewportResizeResolved { .. }
                | crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
                    transaction: crate::runtime::UiAllocationReplanTransactionOutcome::Committed(_)
                        | crate::runtime::UiAllocationReplanTransactionOutcome::Replayed(_),
                    ..
                }
        );
        drop(completion.into_completion());
        if !settled {
            return Err(());
        }
        self.viewport_measurement_pending = false;
        Ok(())
    }
}

use super::{
    WorthUiDurableResizeTurnSource, WorthUiFrameworkTurn, WorthUiHostMeasurementTurnSource,
    WorthUiInteractionTurnSource, WorthUiQueryProjectionTurnSource, WorthUiResizePreviewTurnSource,
    WorthUiScrollExtentTurnSource,
};
use crate::runtime::UiAllocationFrameGatewayOutcome;

impl WorthUiFrameworkTurn<'_> {
    pub fn host_measurement(
        &mut self,
        collect: impl FnOnce(&mut WorthUiHostMeasurementTurnSource<'_>),
    ) {
        collect(&mut WorthUiHostMeasurementTurnSource {
            runtime: self.runtime,
        });
    }
    pub fn query_projection(
        &mut self,
        collect: impl FnOnce(&mut WorthUiQueryProjectionTurnSource<'_>),
    ) {
        collect(&mut WorthUiQueryProjectionTurnSource {
            runtime: self.runtime,
        });
    }
    pub fn interaction(&mut self, collect: impl FnOnce(&mut WorthUiInteractionTurnSource<'_>)) {
        collect(&mut WorthUiInteractionTurnSource {
            runtime: self.runtime,
        });
    }
    pub fn resize_preview(
        &mut self,
        collect: impl FnOnce(&mut WorthUiResizePreviewTurnSource<'_>),
    ) {
        collect(&mut WorthUiResizePreviewTurnSource {
            runtime: self.runtime,
        });
    }
    pub fn durable_resize(
        &mut self,
        collect: impl FnOnce(&mut WorthUiDurableResizeTurnSource<'_>),
    ) {
        collect(&mut WorthUiDurableResizeTurnSource {
            runtime: self.runtime,
        });
    }
    /// Acquire admitted extent/allocation evidence. This lane cannot project
    /// or mutate semantic scroll offsets.
    pub fn scroll_extent(&mut self, collect: impl FnOnce(&mut WorthUiScrollExtentTurnSource<'_>)) {
        collect(&mut WorthUiScrollExtentTurnSource {
            runtime: self.runtime,
        });
    }
}

impl WorthUiScrollExtentTurnSource<'_> {
    pub fn acquire_host_owner(
        &self,
        result: &crate::evidence::UiMeasurementResult,
        receipt: &crate::runtime::UiAllocationReceipt,
    ) -> Result<
        crate::runtime::UiActivatedScrollOwner,
        crate::runtime::UiScrollOwnerAcquisitionDenial,
    > {
        self.runtime
            .allocation_invalidation_index
            .borrow()
            .acquire_host_scroll_projection(result.authority_witness(), receipt)
    }
    pub fn acquire_settled_query_owner(
        &self,
        query: &crate::evidence::UiSettledQueryFactReceipt,
        allocation_receipt: &crate::runtime::UiAllocationReceipt,
    ) -> Result<
        crate::runtime::UiActivatedScrollOwner,
        crate::runtime::UiScrollOwnerAcquisitionDenial,
    > {
        self.runtime
            .allocation_invalidation_index
            .borrow()
            .acquire_settled_query_scroll_projection(query, allocation_receipt)
    }
}

impl WorthUiHostMeasurementTurnSource<'_> {
    pub fn collect_and_submit_capability(
        &mut self,
        capability: &crate::facade::WorthUiHostMeasurementCapability,
        input: crate::facade::WorthUiHostMeasurementSessionInput,
    ) -> Result<UiAllocationFrameGatewayOutcome, crate::host::UiHostMeasurementEvidenceDenial> {
        admit_host_capability(
            self.runtime.host_session_identity,
            self.runtime.host_observation_generation,
            capability.session_identity(),
            capability.observation_generation(),
        )?;
        self.runtime.collect_and_submit_host_measurement(
            capability.adapter(),
            input.bind_report(capability.capability_report()),
        )
    }

    #[cfg(test)]
    pub fn collect_and_submit<A: worth_ui_host_contract::WorthUiMeasurementHostAdapter>(
        &mut self,
        adapter: &A,
        input: crate::host::UiHostMeasurementCollectionInput<'_>,
    ) -> Result<UiAllocationFrameGatewayOutcome, crate::host::UiHostMeasurementEvidenceDenial> {
        self.runtime
            .collect_and_submit_host_measurement(adapter, input)
    }
}

fn admit_host_capability(
    active_identity: Option<crate::facade::WorthUiHostSessionIdentity>,
    active_generation: Option<worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration>,
    observed_identity: crate::facade::WorthUiHostSessionIdentity,
    observed_generation: worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration,
) -> Result<(), crate::host::UiHostMeasurementEvidenceDenial> {
    let active_identity = active_identity
        .ok_or(crate::host::UiHostMeasurementEvidenceDenial::MissingOperationalHostSession)?;
    if active_identity != observed_identity {
        return Err(crate::host::UiHostMeasurementEvidenceDenial::ForeignHostSession);
    }
    let active_generation = active_generation
        .ok_or(crate::host::UiHostMeasurementEvidenceDenial::MissingOperationalHostSession)?;
    if active_generation != observed_generation {
        return Err(crate::host::UiHostMeasurementEvidenceDenial::StaleHostObservationGeneration);
    }
    Ok(())
}

impl WorthUiQueryProjectionTurnSource<'_> {
    /// Retains Query's exact settled projection and returns its UI observation.
    pub fn admit_settled(
        &mut self,
        projection: worth_ui_query_binding::WorthUiSettledSnapshotProjection,
    ) -> Result<
        std::sync::Arc<worth_ui_query_binding::WorthUiSettledSnapshotFact>,
        worth_ui_query_binding::WorthUiSettledSnapshotAdmissionStop,
    > {
        self.runtime.admit_settled_query_projection(projection)
    }

    /// Atomically replaces one retained settlement inside the active
    /// application generation. A denial returns the candidate projection and
    /// leaves the predecessor slot intact.
    pub fn refresh_settled(
        &mut self,
        projection: worth_ui_query_binding::WorthUiSettledSnapshotProjection,
    ) -> Result<
        std::sync::Arc<worth_ui_query_binding::WorthUiSettledSnapshotFact>,
        worth_ui_query_binding::WorthUiSettledSnapshotAdmissionStop,
    > {
        self.runtime.refresh_settled_query_projection(projection)
    }

    /// Resolve one requested active plan link to its retained UI fact and
    /// submit only that fact to allocation ingress.
    pub fn submit_settled(
        &mut self,
        link: &crate::runtime::WorthUiQueryLaneFactLink,
    ) -> Result<
        crate::runtime::WorthUiQueryFrameIngressOutcome,
        crate::runtime::WorthUiQueryFrameIngressDenial,
    > {
        self.runtime.submit_settled_query_fact(link)
    }

    pub fn admit_operation_live(
        &mut self,
        resource: worth_ui_query_binding::WorthUiOperationLiveResource,
    ) -> Result<(), worth_ui_query_binding::WorthUiOperationLiveAdmissionStop> {
        self.runtime.admit_operation_live(resource)
    }

    /// Stage one sealed UI collection consequence for atomic publication when
    /// this framework turn succeeds.
    ///
    /// Query progression remains entirely inside `worth-ui-query-binding`.
    /// Runtime receives only this WUI-owned artifact, and the predecessor
    /// admitted consequence remains current until the enclosing callback
    /// completes without unwinding.
    pub fn admit_collection_change(
        &mut self,
        consequence: worth_ui_query_binding::WorthUiCollectionChangeConsequence,
    ) -> Result<
        worth_ui_query_binding::WorthUiCollectionChangeStagingReceipt,
        worth_ui_query_binding::WorthUiCollectionChangeAdmissionStop,
    > {
        self.runtime.admit_operation_live_change(consequence)
    }

    pub fn refresh_operation_live(
        &mut self,
        request: worth_ui_query_binding::WorthUiOperationLiveRefreshRequest<'_>,
    ) -> Result<
        worth_ui_query_binding::WorthUiOperationLiveSourceRefreshOutcome,
        worth_ui_query_binding::WorthUiOperationLiveSourceRefreshStop,
    > {
        self.runtime.refresh_and_admit_operation_live(request)
    }
}

impl WorthUiInteractionTurnSource<'_> {
    pub fn admit_and_submit(
        &mut self,
        target: crate::graph::UiGraphNodeIdentity,
        state: crate::runtime::WorthUiTransientInteractionState,
    ) -> Result<
        UiAllocationFrameGatewayOutcome,
        crate::runtime::WorthUiTransientInteractionAdmissionDenial,
    > {
        self.runtime.admit_and_submit_interaction(target, state)
    }
}

impl WorthUiResizePreviewTurnSource<'_> {
    pub fn admit_and_submit(
        &mut self,
        sample: crate::runtime::UiResizePreviewSample,
    ) -> Result<
        UiAllocationFrameGatewayOutcome,
        crate::runtime::WorthUiTransientInteractionAdmissionDenial,
    > {
        self.runtime.admit_and_submit_resize_preview(sample)
    }
}

impl WorthUiDurableResizeTurnSource<'_> {
    pub fn admit_and_submit(
        &mut self,
        input: crate::runtime::UiDurableResizeCommitIntent,
    ) -> Result<
        UiAllocationFrameGatewayOutcome,
        crate::runtime::WorthUiDurableResizeSourceAdmissionDenial,
    > {
        self.runtime.admit_and_submit_durable_resize(input)
    }
}

#[cfg(test)]
mod host_capability_admission_tests {
    use super::admit_host_capability;
    use worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration;

    #[test]
    fn stale_observation_generation_denies_below_source_ingress() {
        let session = crate::runtime::tests::active_application_session_test_support::source_backed_component_session();
        let capability = session.host_measurement_capability();
        let current = WorthUiHostCapabilityObservationGeneration::new(
            capability.observation_generation().as_u64() + 1,
        );
        let denial = admit_host_capability(
            Some(capability.session_identity()),
            Some(current),
            capability.session_identity(),
            capability.observation_generation(),
        )
        .expect_err("stale host evidence must stop before source ingress");
        assert_eq!(
            denial,
            crate::host::UiHostMeasurementEvidenceDenial::StaleHostObservationGeneration
        );
    }
}

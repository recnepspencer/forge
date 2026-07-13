use super::{
    WorthUiDurableResizeTurnSource, WorthUiFrameworkTurn, WorthUiHostMeasurementTurnSource,
    WorthUiInteractionTurnSource, WorthUiQueryProjectionTurnSource, WorthUiResizePreviewTurnSource,
    WorthUiScrollOffsetTurnSource,
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
    pub fn scroll_offset(&mut self, project: impl FnOnce(&mut WorthUiScrollOffsetTurnSource<'_>)) {
        project(&mut WorthUiScrollOffsetTurnSource {
            runtime: self.runtime,
        });
    }
}

impl WorthUiScrollOffsetTurnSource<'_> {
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
    pub fn acquire_query_owner(
        &self,
        query: &crate::evidence::UiProjectionFactReceipt,
        allocation_receipt: &crate::runtime::UiAllocationReceipt,
    ) -> Result<
        crate::runtime::UiActivatedScrollOwner,
        crate::runtime::UiScrollOwnerAcquisitionDenial,
    > {
        self.runtime
            .allocation_invalidation_index
            .borrow()
            .acquire_query_scroll_projection(query.query_authority(), allocation_receipt)
    }
    pub fn project(
        &mut self,
        offset: crate::runtime::UiProjectedScrollOffset,
    ) -> Result<
        crate::runtime::UiProjectedScrollOffsetOutcome,
        crate::runtime::UiProjectedScrollOffsetDenial,
    > {
        let Some(active) = self
            .runtime
            .allocation_invalidation_index
            .borrow()
            .scroll_projection_target(offset.capability().owner_identity())
        else {
            return Err(crate::runtime::UiProjectedScrollOffsetDenial::TargetNotActivated);
        };
        if active != offset.capability() {
            return Err(crate::runtime::UiProjectedScrollOffsetDenial::ScrollOwnershipNotAdmitted);
        }
        if self
            .runtime
            .allocation_invalidation_index
            .borrow()
            .validate_scroll_projection_receipt(offset.capability(), offset.receipt_key())
            .is_err()
        {
            return Err(crate::runtime::UiProjectedScrollOffsetDenial::ScrollOwnershipNotAdmitted);
        }
        let ingress_before = self
            .runtime
            .allocation_frame_dispatcher_counters()
            .ingress_count();
        let truth_before = self.runtime.allocation_receipt_ledger.truth_revision();
        let projection_generation = self
            .runtime
            .scroll_offset_projection
            .record(offset.clone())?;
        let ingress_after = self
            .runtime
            .allocation_frame_dispatcher_counters()
            .ingress_count();
        let truth_after = self.runtime.allocation_receipt_ledger.truth_revision();
        let allocation_invalidations = ingress_after.checked_sub(ingress_before).ok_or(
            crate::runtime::UiProjectedScrollOffsetDenial::AllocationIngressCounterRegressed,
        )?;
        let committed_receipts = truth_after
            .delta_since(truth_before)
            .ok_or(crate::runtime::UiProjectedScrollOffsetDenial::AllocationTruthRevisionRegressed)?
            .committed_receipt_publications();
        Ok(crate::runtime::UiProjectedScrollOffsetOutcome::seal(
            offset,
            projection_generation,
            allocation_invalidations,
            committed_receipts,
        ))
    }
}

impl WorthUiHostMeasurementTurnSource<'_> {
    pub fn collect_and_submit<A: worth_ui_host_contract::WorthUiMeasurementHostAdapter>(
        &mut self,
        adapter: &A,
        identity: worth_ui_host_contract::UiMeasurementRequestIdentity,
        evidence_family: worth_ui_host_contract::UiMeasurementEvidenceFamily,
        need: crate::host::UiHostMeasurementNeed,
        capability_report: &worth_ui_host_contract::WorthUiHostCapabilityReport,
        evidence_generation: worth_ui_inspection::UiEvidenceAuthorityGeneration,
        normalization_context: crate::host::UiHostMeasurementNormalizationContext,
    ) -> Result<UiAllocationFrameGatewayOutcome, crate::host::UiHostMeasurementEvidenceDenial> {
        self.runtime.collect_and_submit_host_measurement(
            adapter,
            identity,
            evidence_family,
            need,
            capability_report,
            evidence_generation,
            normalization_context,
        )
    }
}

impl WorthUiQueryProjectionTurnSource<'_> {
    pub fn admit_and_submit(
        &mut self,
        prerequisites: worth_ui_query_binding::WorthUiQueryPrerequisiteEvidence,
        authority: worth_query::facade::foundation::ProjectionAuthorityOutcome,
    ) -> Result<
        UiAllocationFrameGatewayOutcome,
        worth_ui_query_binding::WorthUiQueryMeasurementFactSettlementDenial,
    > {
        self.runtime
            .admit_and_submit_query_projection(prerequisites, authority)
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

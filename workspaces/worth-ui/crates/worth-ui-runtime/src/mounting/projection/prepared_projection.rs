use super::frame_storage::{UiMountedProjectionFrameInput, UiMountedSemanticProjection};
use super::{UiMountedProjectionDenial, UiMountedProjectionFrame};

pub(crate) struct UiPreparedMountedProjection {
    plan_digest: u64,
    semantic: UiMountedSemanticProjection,
    ordinary: Option<crate::runtime::WorthUiOrdinaryLaneFrameReceipt>,
    virtualized: Option<crate::runtime::WorthUiVirtualizedDataFrameReceipt>,
    canvas: Option<(crate::runtime::WorthUiCanvasSpatialFrameReceipt, u64)>,
    realtime: Option<crate::runtime::WorthUiRealtimeFrameReceipt>,
    preview: Option<super::lowering::UiMountedPreviewProjectionInput>,
    visual_overlay: Option<super::super::UiMountedVisualOverlayProjectionInput>,
    projection_changes: super::super::UiMountedProjectionChangeSnapshot,
    counters: super::super::UiMountStageCounters,
}

pub(super) struct UiPreparedMountedProjectionInput {
    pub(super) plan_digest: u64,
    pub(super) semantic: UiMountedSemanticProjection,
    pub(super) preview: Option<super::lowering::UiMountedPreviewProjectionInput>,
    pub(super) visual_overlay: Option<super::super::UiMountedVisualOverlayProjectionInput>,
    pub(super) projection_changes: super::super::UiMountedProjectionChangeSnapshot,
    pub(super) counters: super::super::UiMountStageCounters,
}

#[derive(Clone)]
pub struct UiProjectedMountedFrameCandidate {
    pub(in crate::mounting) frame: UiMountedProjectionFrame,
    pub(in crate::mounting) identity_candidate:
        super::super::identity_state::UiMountedIdentityFrameCandidate,
    pub(in crate::mounting) projection_changes: super::super::UiMountedProjectionChangeSnapshot,
}

impl UiProjectedMountedFrameCandidate {
    pub fn frame(&self) -> &UiMountedProjectionFrame {
        &self.frame
    }

    pub fn is_unpublished(&self) -> bool {
        let _ = &self.identity_candidate;
        true
    }

    pub(crate) fn presented_receipt_basis(&self) -> &super::super::UiMountedNodeReceiptBasis {
        self.identity_candidate.receipt_basis()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        UiMountedProjectionFrame,
        super::super::identity_state::UiMountedIdentityFrameCandidate,
        super::super::UiMountedProjectionChangeSnapshot,
    ) {
        (self.frame, self.identity_candidate, self.projection_changes)
    }
}

impl UiPreparedMountedProjection {
    pub(super) fn new(input: UiPreparedMountedProjectionInput) -> Self {
        Self {
            plan_digest: input.plan_digest,
            semantic: input.semantic,
            ordinary: None,
            virtualized: None,
            canvas: None,
            realtime: None,
            preview: input.preview,
            visual_overlay: input.visual_overlay,
            projection_changes: input.projection_changes,
            counters: input.counters,
        }
    }

    pub(crate) fn record_ordinary(
        &mut self,
        receipt: &crate::runtime::WorthUiOrdinaryLaneFrameReceipt,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_vacant(&self.ordinary)?;
        self.ordinary = Some(receipt.clone());
        Ok(())
    }

    pub(crate) fn record_virtualized(
        &mut self,
        receipt: &crate::runtime::WorthUiVirtualizedDataFrameReceipt,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_vacant(&self.virtualized)?;
        self.virtualized = Some(receipt.clone());
        Ok(())
    }

    pub(crate) fn record_canvas(
        &mut self,
        receipt: &crate::runtime::WorthUiCanvasSpatialFrameReceipt,
        resource_content_identity: u64,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_vacant(&self.canvas)?;
        self.canvas = Some((receipt.clone(), resource_content_identity));
        Ok(())
    }

    pub(crate) fn record_realtime(
        &mut self,
        receipt: &crate::runtime::WorthUiRealtimeFrameReceipt,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_vacant(&self.realtime)?;
        self.realtime = Some(receipt.clone());
        Ok(())
    }

    pub(crate) fn finish(
        self,
        state: &super::super::UiMountedIdentityState,
    ) -> Result<UiProjectedMountedFrameCandidate, UiMountedProjectionDenial> {
        self.validate_capacity()?;
        let identity_candidate = state.prepare_frame_candidate_for(self.semantic.membership())?;
        let mut frame = UiMountedProjectionFrame::new(UiMountedProjectionFrameInput {
            frame: identity_candidate.frame(),
            receipt_basis: identity_candidate.receipt_basis().clone(),
            plan_digest: self.plan_digest,
            semantic: self.semantic,
            counters: self.counters,
        });
        frame.complete_static_paint()?;
        frame.complete_hit_tests()?;
        if let Some(receipt) = self.ordinary.as_ref() {
            frame.record_ordinary(receipt)?;
        }
        if let Some(receipt) = self.virtualized.as_ref() {
            frame.record_virtualized(receipt)?;
        }
        if let Some((receipt, resource)) = self.canvas.as_ref() {
            frame.record_canvas(receipt, *resource)?;
        }
        if let Some(receipt) = self.realtime.as_ref() {
            frame.record_realtime(receipt)?;
        }
        if let Some(preview) = self.preview {
            frame.record_preview(preview)?;
        }
        frame.record_visual_overlay(self.visual_overlay)?;
        Ok(UiProjectedMountedFrameCandidate {
            frame,
            identity_candidate,
            projection_changes: self.projection_changes,
        })
    }

    fn validate_capacity(&self) -> Result<(), UiMountedProjectionDenial> {
        let paint_rows = usize::from(self.ordinary.is_some())
            + usize::from(self.virtualized.is_some())
            + usize::from(self.canvas.is_some())
            + usize::from(self.realtime.is_some());
        if paint_rows > 2_048 {
            return Err(UiMountedProjectionDenial::TableCapacityExceeded);
        }
        Ok(())
    }
}

fn require_vacant<T>(slot: &Option<T>) -> Result<(), UiMountedProjectionDenial> {
    slot.is_none()
        .then_some(())
        .ok_or(UiMountedProjectionDenial::DuplicateLaneContribution)
}

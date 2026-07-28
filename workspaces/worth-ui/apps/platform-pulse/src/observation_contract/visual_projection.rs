use worth_ui::facade::inspection::{
    UiClearedVisualOverlayReceipt, UiClientPhysicalPixel, UiPixelsRequired,
    UiPublishedVisualOverlay, UiVisualOverlayDenial, UiVisualPointAdjudication,
    UiVisualSnapshotDisposalReceipt, UiVisualSnapshotIdentity, UiVisualSnapshotReceipt,
};

use super::envelope::PlatformPulseLifecycleObservationEnvelope;
use super::lifecycle::PlatformPulseLifecycleObservation;
use super::projection::{
    PlatformPulseLifecycleObservationProjectionDenial, PlatformPulseLifecycleObservationStream,
    PlatformPulseVisualObservationState,
};
use super::visual::{
    PlatformPulseVisualOverlayCleared, PlatformPulseVisualOverlayPublished,
    PlatformPulseVisualPointTrace, PlatformPulseVisualSnapshotRelationObservation,
    PlatformPulseVisualSnapshotRetired,
};
use super::visual_value_projection::{project_point_resolution, project_snapshot, rect};

pub struct PlatformPulseVisualPointObservation<'a> {
    point: UiClientPhysicalPixel,
    adjudication: &'a UiVisualPointAdjudication,
}

impl<'a> PlatformPulseVisualPointObservation<'a> {
    pub fn new(point: UiClientPhysicalPixel, adjudication: &'a UiVisualPointAdjudication) -> Self {
        Self {
            point,
            adjudication,
        }
    }
}

pub struct PlatformPulseVisualPointTraceInput<'a> {
    receipt: &'a UiVisualSnapshotReceipt<UiPixelsRequired>,
    target: PlatformPulseVisualPointObservation<'a>,
    background: PlatformPulseVisualPointObservation<'a>,
}

impl<'a> PlatformPulseVisualPointTraceInput<'a> {
    pub fn new(
        receipt: &'a UiVisualSnapshotReceipt<UiPixelsRequired>,
        target: PlatformPulseVisualPointObservation<'a>,
        background: PlatformPulseVisualPointObservation<'a>,
    ) -> Self {
        Self {
            receipt,
            target,
            background,
        }
    }
}

impl PlatformPulseLifecycleObservationStream {
    pub fn project_visual_snapshot(
        &mut self,
        receipt: &UiVisualSnapshotReceipt<UiPixelsRequired>,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        let PlatformPulseVisualObservationState::AwaitingSnapshot { frame } = self.visual_state
        else {
            return Err(
                PlatformPulseLifecycleObservationProjectionDenial::VisualObservationOutOfOrder,
            );
        };
        let captured = project_snapshot(receipt)?;
        if captured.affinity.frame != frame
            || captured.affinity.relation != PlatformPulseVisualSnapshotRelationObservation::Current
        {
            return Err(PlatformPulseLifecycleObservationProjectionDenial::VisualAffinityMismatch);
        }
        let snapshot = captured.affinity.snapshot;
        let envelope = self.next_envelope(
            PlatformPulseLifecycleObservation::VisualSnapshotCaptured(captured),
        )?;
        self.visual_state =
            PlatformPulseVisualObservationState::SnapshotCaptured { snapshot, frame };
        Ok(envelope)
    }

    pub fn project_visual_point_trace(
        &mut self,
        input: PlatformPulseVisualPointTraceInput<'_>,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        let PlatformPulseVisualPointTraceInput {
            receipt,
            target,
            background,
        } = input;
        let PlatformPulseVisualObservationState::SnapshotCaptured { snapshot, frame } =
            self.visual_state
        else {
            return Err(
                PlatformPulseLifecycleObservationProjectionDenial::VisualObservationOutOfOrder,
            );
        };
        if receipt.identity().diagnostic_value() != snapshot || receipt.affinity().frame() != frame
        {
            return Err(PlatformPulseLifecycleObservationProjectionDenial::VisualAffinityMismatch);
        }
        let target = project_point_resolution(target.point, target.adjudication)?;
        let background = project_point_resolution(background.point, background.adjudication)?;
        let target_receipt = target.hit.mounted.node_receipt;
        if target.visible.mounted.node_receipt != target_receipt
            || background.visible.mounted.node_receipt != background.hit.mounted.node_receipt
            || background.hit.mounted.node_receipt == target_receipt
        {
            return Err(
                PlatformPulseLifecycleObservationProjectionDenial::VisualPointIdentityMismatch,
            );
        }
        let envelope = self.next_envelope(PlatformPulseLifecycleObservation::VisualPointTrace(
            PlatformPulseVisualPointTrace {
                snapshot,
                target,
                background,
            },
        ))?;
        self.visual_state = PlatformPulseVisualObservationState::IdentityTraced {
            snapshot,
            frame,
            target_receipt,
        };
        Ok(envelope)
    }

    pub fn project_visual_overlay_published(
        &mut self,
        published: &UiPublishedVisualOverlay,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        let PlatformPulseVisualObservationState::IdentityTraced {
            snapshot,
            frame,
            target_receipt,
        } = self.visual_state
        else {
            return Err(
                PlatformPulseLifecycleObservationProjectionDenial::VisualObservationOutOfOrder,
            );
        };
        let target = published.target().mounted_node();
        if published.base_snapshot().diagnostic_value() != snapshot
            || published.base_frame().diagnostic_value() != frame
            || target.node_receipt() != target_receipt
            || published.published_frame() == published.base_frame()
        {
            return Err(PlatformPulseLifecycleObservationProjectionDenial::VisualOverlayMismatch);
        }
        let observation = PlatformPulseVisualOverlayPublished {
            overlay: published.identity().diagnostic_value(),
            base_snapshot: snapshot,
            base_frame: frame,
            target_region: rect(published.target_region()),
            published_frame: published.published_frame().diagnostic_value(),
        };
        let envelope = self.next_envelope(
            PlatformPulseLifecycleObservation::VisualOverlayPublished(observation),
        )?;
        self.visual_state = PlatformPulseVisualObservationState::OverlayPublished {
            snapshot,
            snapshot_frame: frame,
            overlay: observation.overlay,
            published_frame: observation.published_frame,
        };
        Ok(envelope)
    }

    pub fn project_visual_overlay_cleared(
        &mut self,
        cleared: UiClearedVisualOverlayReceipt,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        let PlatformPulseVisualObservationState::OverlayPublished {
            snapshot,
            snapshot_frame,
            overlay,
            published_frame,
        } = self.visual_state
        else {
            return Err(
                PlatformPulseLifecycleObservationProjectionDenial::VisualObservationOutOfOrder,
            );
        };
        if cleared.identity().diagnostic_value() != overlay
            || cleared.published_frame().diagnostic_value() != published_frame
            || cleared.cleared_frame() == cleared.published_frame()
        {
            return Err(PlatformPulseLifecycleObservationProjectionDenial::VisualOverlayMismatch);
        }
        let observation = PlatformPulseVisualOverlayCleared {
            overlay,
            published_frame,
            cleared_frame: cleared.cleared_frame().diagnostic_value(),
        };
        let envelope = self.next_envelope(
            PlatformPulseLifecycleObservation::VisualOverlayCleared(observation),
        )?;
        self.visual_state = PlatformPulseVisualObservationState::OverlayCleared {
            snapshot,
            snapshot_frame,
            overlay,
            published_frame,
            cleared_frame: observation.cleared_frame,
        };
        Ok(envelope)
    }

    pub fn project_visual_snapshot_retired(
        &mut self,
        snapshot: UiVisualSnapshotIdentity,
        denial: UiVisualOverlayDenial,
        disposal: UiVisualSnapshotDisposalReceipt,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        let PlatformPulseVisualObservationState::AwaitingRetirement {
            snapshot: expected,
            snapshot_frame,
            successor_frame,
        } = self.visual_state
        else {
            return Err(
                PlatformPulseLifecycleObservationProjectionDenial::VisualObservationOutOfOrder,
            );
        };
        if snapshot.diagnostic_value() != expected
            || disposal.identity() != snapshot
            || denial != UiVisualOverlayDenial::Superseded
        {
            return Err(
                PlatformPulseLifecycleObservationProjectionDenial::VisualRetirementMismatch,
            );
        }
        if !disposal.released_registered_resource() {
            return Err(
                PlatformPulseLifecycleObservationProjectionDenial::VisualResourceNotReleased,
            );
        }
        let observation = PlatformPulseVisualSnapshotRetired {
            snapshot: expected,
            predecessor_frame: snapshot_frame,
            successor_frame,
            explicitly_superseded: true,
            released_registered_resource: true,
        };
        let envelope = self.next_envelope(
            PlatformPulseLifecycleObservation::VisualSnapshotRetired(observation),
        )?;
        self.visual_state = PlatformPulseVisualObservationState::Retired;
        Ok(envelope)
    }
}

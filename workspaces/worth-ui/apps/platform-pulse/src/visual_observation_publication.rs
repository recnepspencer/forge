use worth_ui::facade::inspection::{
    UiClearedVisualOverlayReceipt, UiPixelsRequired, UiPublishedVisualOverlay,
    UiVisualSnapshotDisposalReceipt, UiVisualSnapshotIdentity, UiVisualSnapshotReceipt,
    UiVisualSnapshotRelation,
};

use crate::lifecycle_observation_publication::{
    PlatformPulseObservationPublicationDenial, PlatformPulseObservationPublisher,
};
use worth_ui_platform_pulse::observation_contract::PlatformPulseVisualPointTraceInput;

impl PlatformPulseObservationPublisher {
    pub(crate) fn visual_snapshot(
        &self,
        receipt: &UiVisualSnapshotReceipt<UiPixelsRequired>,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.project_observation(|stream| stream.project_visual_snapshot(receipt))
    }

    pub(crate) fn successor_visual_snapshot(
        &self,
        receipt: &UiVisualSnapshotReceipt<UiPixelsRequired>,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.project_observation(|stream| stream.project_successor_visual_snapshot(receipt))
    }

    pub(crate) fn refreshed_visual_snapshot(
        &self,
        receipt: &UiVisualSnapshotReceipt<UiPixelsRequired>,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.project_observation(|stream| stream.project_refreshed_visual_snapshot(receipt))
    }

    pub(crate) fn visual_point_trace(
        &self,
        input: PlatformPulseVisualPointTraceInput<'_>,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.project_observation(|stream| stream.project_visual_point_trace(input))
    }

    pub(crate) fn visual_overlay_published(
        &self,
        published: &UiPublishedVisualOverlay,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.project_observation(|stream| stream.project_visual_overlay_published(published))
    }

    pub(crate) fn visual_overlay_cleared(
        &self,
        cleared: UiClearedVisualOverlayReceipt,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.project_observation(|stream| stream.project_visual_overlay_cleared(cleared))
    }

    pub(crate) fn visual_snapshot_retired(
        &self,
        snapshot: UiVisualSnapshotIdentity,
        relation: UiVisualSnapshotRelation,
        disposal: UiVisualSnapshotDisposalReceipt,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.project_observation(|stream| {
            stream.project_visual_snapshot_retired(snapshot, relation, disposal)
        })
    }

    pub(crate) fn visual_snapshot_retired_after_current_successor(
        &self,
        snapshot: UiVisualSnapshotIdentity,
        predecessor_frame: u64,
        successor_frame: u64,
        disposal: UiVisualSnapshotDisposalReceipt,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.project_observation(|stream| {
            stream.project_visual_snapshot_retired_after_current_successor(
                snapshot,
                predecessor_frame,
                successor_frame,
                disposal,
            )
        })
    }

    pub(crate) fn visual_identity_failure(
        &self,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.project_observation(|stream| stream.project_visual_identity_failure())
    }
}

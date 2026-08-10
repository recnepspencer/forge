use std::collections::BTreeMap;
use worth_ui_host_contract::{
    UiHostSurfacePresentationDenial, UiMountedLogicalDamage, UiMountedPaintCommandChange,
    UiMountedPaintOrderEdit, UiMountedPaintOrderIntegrity,
};

use crate::headless_transcript::UiHeadlessTranscriptSuccessorIdentity;
use crate::UiHeadlessMountedFrameTranscript;

#[derive(Clone)]
pub(super) enum UiHeadlessRecordedFrame {
    Complete(UiHeadlessMountedFrameTranscript),
    Delta(UiHeadlessRecordedDelta),
}

#[derive(Clone)]
pub(super) struct UiHeadlessRecordedDelta {
    identity: UiHeadlessTranscriptSuccessorIdentity,
    changes: Box<[UiMountedPaintCommandChange]>,
    order: Box<[UiMountedPaintOrderEdit]>,
    order_integrity: UiMountedPaintOrderIntegrity,
    damage: Box<[UiMountedLogicalDamage]>,
}

impl UiHeadlessRecordedFrame {
    pub(super) fn complete(transcript: UiHeadlessMountedFrameTranscript) -> Self {
        Self::Complete(transcript)
    }

    pub(super) fn delta(
        view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
        delta: &worth_ui_host_contract::UiMountedPresentationDelta,
    ) -> Self {
        Self::Delta(UiHeadlessRecordedDelta {
            identity: UiHeadlessTranscriptSuccessorIdentity {
                host_session_identity: view.host_session_identity(),
                protocol: view.protocol(),
                attempt: view.attempt(),
                frame: view.frame(),
                binding: view.binding(),
            },
            changes: delta.changes().into(),
            order: delta.order().into(),
            order_integrity: delta.order_integrity(),
            damage: delta.damage().into(),
        })
    }

    fn materialize(
        &self,
        predecessor: Option<&UiHeadlessMountedFrameTranscript>,
    ) -> Result<UiHeadlessMountedFrameTranscript, UiHostSurfacePresentationDenial> {
        match self {
            Self::Complete(transcript) => Ok(transcript.clone()),
            Self::Delta(delta) => predecessor
                .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?
                .successor_recorded_delta(
                    delta.identity,
                    &delta.changes,
                    &delta.order,
                    delta.order_integrity,
                    &delta.damage,
                ),
        }
    }

    fn binding(&self) -> worth_ui_host_contract::UiSurfaceBindingGeneration {
        match self {
            Self::Complete(transcript) => transcript.binding(),
            Self::Delta(delta) => delta.identity.binding,
        }
    }
}

pub(super) fn materialize_frames(
    records: impl IntoIterator<Item = UiHeadlessRecordedFrame>,
    checkpoints: &mut BTreeMap<
        worth_ui_host_contract::UiSurfaceBindingGeneration,
        UiHeadlessMountedFrameTranscript,
    >,
) -> Result<Box<[UiHeadlessMountedFrameTranscript]>, UiHostSurfacePresentationDenial> {
    let mut transcripts = Vec::new();
    for record in records {
        let binding = record.binding();
        let transcript = record.materialize(checkpoints.get(&binding))?;
        checkpoints.insert(binding, transcript.clone());
        transcripts.push(transcript);
    }
    Ok(transcripts.into_boxed_slice())
}

pub(super) fn materialize_latest(
    records: impl IntoIterator<Item = UiHeadlessRecordedFrame>,
) -> Result<UiHeadlessMountedFrameTranscript, UiHostSurfacePresentationDenial> {
    let mut latest = None;
    for record in records {
        latest = Some(record.materialize(latest.as_ref())?);
    }
    latest.ok_or(UiHostSurfacePresentationDenial::MalformedProjection)
}

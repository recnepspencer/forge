use std::sync::atomic::{AtomicU64, Ordering};

use worth_ui::facade::app::{
    UiMountedFramePublicationReceipt, WorthUiApplicationCutoverReceipt,
    WorthUiPreparedApplicationGenerationIdentity,
};
use worth_ui::facade::source::{
    WorthUiSourcePackageRevision, WorthUiWatchedCandidateSubmissionDenial,
};

use super::envelope::{
    PlatformPulseLifecycleObservationEnvelope, PlatformPulseObservationRunIdentity,
    PlatformPulseObservationSequence,
};
use super::lifecycle::{
    PlatformPulseApplicationGenerationObservation, PlatformPulseFirstFramePublished,
    PlatformPulseLifecycleObservation, PlatformPulseMountedFrameObservation,
    PlatformPulseProcessStarted, PlatformPulseReplacementDenialFamily,
    PlatformPulseReplacementPreserved, PlatformPulseReplacementPublished,
    PlatformPulseSourceSnapshotObservation,
};

static NEXT_RUN_ORDINAL: AtomicU64 = AtomicU64::new(1);

pub struct PlatformPulseLifecycleObservationStream {
    run: PlatformPulseObservationRunIdentity,
    next_sequence: u64,
    pub(super) state: PlatformPulseObservationState,
    pub(super) visual_state: PlatformPulseVisualObservationState,
}

pub(super) enum PlatformPulseObservationState {
    Started,
    Published {
        generation: WorthUiPreparedApplicationGenerationIdentity,
        generation_observation: PlatformPulseApplicationGenerationObservation,
        frame: PlatformPulseMountedFrameObservation,
    },
    Terminal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PlatformPulseVisualObservationState {
    AwaitingFirstFrame,
    AwaitingSnapshot {
        frame: u64,
    },
    SnapshotCaptured {
        snapshot: u64,
        frame: u64,
    },
    IdentityTraced {
        snapshot: u64,
        frame: u64,
        target_receipt: u64,
    },
    OverlayPublished {
        snapshot: u64,
        snapshot_frame: u64,
        overlay: u64,
        published_frame: u64,
    },
    OverlayCleared {
        snapshot: u64,
        snapshot_frame: u64,
        overlay: u64,
        published_frame: u64,
        cleared_frame: u64,
    },
    AwaitingRetirement {
        snapshot: u64,
        snapshot_frame: u64,
        successor_frame: u64,
    },
    Retired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlatformPulseLifecycleObservationProjectionDenial {
    SequenceExhausted,
    FirstFrameAlreadyPublished,
    PublishedPredecessorUnavailable,
    PriorGenerationMismatch,
    ActiveGenerationMismatch,
    OutcomeIsNotFailure,
    VisualObservationOutOfOrder,
    VisualAffinityMismatch,
    VisualPointUnsupported,
    VisualPointIdentityMismatch,
    VisualOverlayMismatch,
    VisualPulseIncomplete,
    VisualRetirementMismatch,
    VisualResourceNotReleased,
    ObservationValueOverflow,
    StreamTerminated,
}

impl PlatformPulseLifecycleObservationStream {
    pub fn start() -> (Self, PlatformPulseLifecycleObservationEnvelope) {
        let ordinal = NEXT_RUN_ORDINAL.fetch_add(1, Ordering::Relaxed);
        let run = PlatformPulseObservationRunIdentity::for_current_process(ordinal);
        let started = PlatformPulseLifecycleObservationEnvelope::new(
            run.clone(),
            PlatformPulseObservationSequence::new(1),
            PlatformPulseLifecycleObservation::ProcessStarted(PlatformPulseProcessStarted::new()),
        );
        (
            Self {
                run,
                next_sequence: 2,
                state: PlatformPulseObservationState::Started,
                visual_state: PlatformPulseVisualObservationState::AwaitingFirstFrame,
            },
            started,
        )
    }

    pub fn project_first_frame(
        &mut self,
        source: &WorthUiSourcePackageRevision,
        publication: &UiMountedFramePublicationReceipt,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        if !matches!(self.state, PlatformPulseObservationState::Started) {
            return Err(self.state_denial());
        }
        let generation = publication.generation().clone();
        let generation_observation =
            PlatformPulseApplicationGenerationObservation::from_generation(&generation);
        let frame = PlatformPulseMountedFrameObservation {
            diagnostic_value: publication.frame().diagnostic_value(),
        };
        let outcome = PlatformPulseLifecycleObservation::FirstFramePublished(
            PlatformPulseFirstFramePublished {
                source: PlatformPulseSourceSnapshotObservation::from_revision(source),
                generation: generation_observation,
                frame,
                actual_native_effect_count: publication.cost_report().adapter().translated_rows(),
            },
        );
        let envelope = self.next_envelope(outcome)?;
        self.state = PlatformPulseObservationState::Published {
            generation,
            generation_observation,
            frame,
        };
        self.visual_state = PlatformPulseVisualObservationState::AwaitingSnapshot {
            frame: frame.diagnostic_value,
        };
        Ok(envelope)
    }

    pub fn project_replacement(
        &mut self,
        source: &WorthUiSourcePackageRevision,
        application: &WorthUiApplicationCutoverReceipt,
        mounted: &UiMountedFramePublicationReceipt,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        let (prior, _, _) = self.published_predecessor()?;
        if &prior != application.prior_generation() {
            return Err(PlatformPulseLifecycleObservationProjectionDenial::PriorGenerationMismatch);
        }
        if mounted.generation() != application.active_generation() {
            return Err(
                PlatformPulseLifecycleObservationProjectionDenial::ActiveGenerationMismatch,
            );
        }
        let generation = application.active_generation().clone();
        let generation_observation =
            PlatformPulseApplicationGenerationObservation::from_generation(&generation);
        let frame = PlatformPulseMountedFrameObservation {
            diagnostic_value: mounted.frame().diagnostic_value(),
        };
        let next_visual_state = self
            .visual_state
            .after_replacement(frame.diagnostic_value)?;
        let outcome = PlatformPulseLifecycleObservation::ReplacementPublished(
            PlatformPulseReplacementPublished {
                source: PlatformPulseSourceSnapshotObservation::from_revision(source),
                predecessor_generation:
                    PlatformPulseApplicationGenerationObservation::from_generation(
                        application.prior_generation(),
                    ),
                active_generation: generation_observation,
                successor_frame: frame,
                actual_native_effect_count: mounted.cost_report().adapter().translated_rows(),
            },
        );
        let envelope = self.next_envelope(outcome)?;
        self.state = PlatformPulseObservationState::Published {
            generation,
            generation_observation,
            frame,
        };
        self.visual_state = next_visual_state;
        Ok(envelope)
    }

    pub fn project_preserved_predecessor(
        &mut self,
        source: &WorthUiSourcePackageRevision,
        denial: &WorthUiWatchedCandidateSubmissionDenial,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        let (_, generation, frame) = self.published_predecessor()?;
        let denial_family = match denial {
            WorthUiWatchedCandidateSubmissionDenial::DslCompilation(_) => {
                PlatformPulseReplacementDenialFamily::DslCompilation
            }
            WorthUiWatchedCandidateSubmissionDenial::SourceIngress(_) => {
                PlatformPulseReplacementDenialFamily::SourceIngress
            }
            WorthUiWatchedCandidateSubmissionDenial::RuntimePreparation(_) => {
                PlatformPulseReplacementDenialFamily::RuntimePreparation
            }
            WorthUiWatchedCandidateSubmissionDenial::Candidate(_) => {
                PlatformPulseReplacementDenialFamily::Candidate
            }
        };
        self.next_envelope(
            PlatformPulseLifecycleObservation::ReplacementDeniedPreserving(
                PlatformPulseReplacementPreserved {
                    source: PlatformPulseSourceSnapshotObservation::from_revision(source),
                    active_generation: generation,
                    active_frame: frame,
                    denial_family,
                },
            ),
        )
    }

    pub(super) fn published_predecessor(
        &self,
    ) -> Result<
        (
            WorthUiPreparedApplicationGenerationIdentity,
            PlatformPulseApplicationGenerationObservation,
            PlatformPulseMountedFrameObservation,
        ),
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        match &self.state {
            PlatformPulseObservationState::Published {
                generation,
                generation_observation,
                frame,
            } => Ok((generation.clone(), *generation_observation, *frame)),
            PlatformPulseObservationState::Started => Err(
                PlatformPulseLifecycleObservationProjectionDenial::PublishedPredecessorUnavailable,
            ),
            PlatformPulseObservationState::Terminal => {
                Err(PlatformPulseLifecycleObservationProjectionDenial::StreamTerminated)
            }
        }
    }

    pub(super) fn next_envelope(
        &mut self,
        outcome: PlatformPulseLifecycleObservation,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        let sequence = self.next_sequence;
        self.next_sequence = sequence
            .checked_add(1)
            .ok_or(PlatformPulseLifecycleObservationProjectionDenial::SequenceExhausted)?;
        Ok(PlatformPulseLifecycleObservationEnvelope::new(
            self.run.clone(),
            PlatformPulseObservationSequence::new(sequence),
            outcome,
        ))
    }

    fn state_denial(&self) -> PlatformPulseLifecycleObservationProjectionDenial {
        match self.state {
            PlatformPulseObservationState::Started => {
                PlatformPulseLifecycleObservationProjectionDenial::PublishedPredecessorUnavailable
            }
            PlatformPulseObservationState::Published { .. } => {
                PlatformPulseLifecycleObservationProjectionDenial::FirstFrameAlreadyPublished
            }
            PlatformPulseObservationState::Terminal => {
                PlatformPulseLifecycleObservationProjectionDenial::StreamTerminated
            }
        }
    }
}

impl PlatformPulseVisualObservationState {
    pub(super) fn after_replacement(
        self,
        successor_frame: u64,
    ) -> Result<Self, PlatformPulseLifecycleObservationProjectionDenial> {
        match self {
            Self::AwaitingSnapshot { .. } => Ok(Self::AwaitingSnapshot {
                frame: successor_frame,
            }),
            Self::OverlayCleared {
                snapshot,
                snapshot_frame,
                ..
            } => Ok(Self::AwaitingRetirement {
                snapshot,
                snapshot_frame,
                successor_frame,
            }),
            Self::Retired => Ok(Self::Retired),
            Self::AwaitingFirstFrame
            | Self::SnapshotCaptured { .. }
            | Self::IdentityTraced { .. }
            | Self::OverlayPublished { .. }
            | Self::AwaitingRetirement { .. } => {
                Err(PlatformPulseLifecycleObservationProjectionDenial::VisualPulseIncomplete)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PlatformPulseLifecycleObservationProjectionDenial, PlatformPulseLifecycleObservationStream,
    };

    #[test]
    fn process_start_origin_and_terminal_progression_are_monotonic_and_closed() {
        let (mut stream, started) = PlatformPulseLifecycleObservationStream::start();
        assert_eq!(started.sequence().value(), 1);
        let terminal = stream
            .project_native_event_loop_failure()
            .expect("terminal event");
        assert_eq!(terminal.sequence().value(), 2);
        assert_eq!(
            stream.project_source_worker_panic(),
            Err(PlatformPulseLifecycleObservationProjectionDenial::StreamTerminated)
        );
    }
}

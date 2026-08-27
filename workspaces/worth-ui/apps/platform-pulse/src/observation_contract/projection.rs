use std::sync::atomic::{AtomicU64, Ordering};

use worth_ui::facade::app::{
    UiMountedFramePublicationReceipt, WorthUiPreparedApplicationGenerationIdentity,
};
use worth_ui::facade::source::{
    UiSourceRebindAttemptDenial, UiSourceRebindAttemptFailure, WorthUiSourcePackageRevision,
};

use super::envelope::{
    PlatformPulseLifecycleObservationEnvelope, PlatformPulseObservationRunIdentity,
    PlatformPulseObservationSequence,
};
use super::lifecycle::{
    PlatformPulseApplicationGenerationObservation, PlatformPulseFirstFramePublished,
    PlatformPulseLifecycleObservation, PlatformPulseMountedFrameObservation,
    PlatformPulseProcessStarted, PlatformPulseReplacementDenialFamily,
    PlatformPulseReplacementPreserved, PlatformPulseSourceSnapshotObservation,
};

mod content_publication;
mod replacement_projection;
mod visual_state;

pub(super) use visual_state::PlatformPulseVisualObservationState;

static NEXT_RUN_ORDINAL: AtomicU64 = AtomicU64::new(1);

pub struct PlatformPulseLifecycleObservationStream {
    run: PlatformPulseObservationRunIdentity,
    next_sequence: u64,
    pointer_input_published: bool,
    keyboard_input_published: bool,
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
pub enum PlatformPulseLifecycleObservationProjectionDenial {
    SequenceExhausted,
    FirstFrameAlreadyPublished,
    NativeInputEvidenceNotNovel,
    PublishedPredecessorUnavailable,
    PriorGenerationMismatch,
    ActiveGenerationMismatch,
    MountedGenerationMismatch,
    OutcomeIsNotFailure,
    VisualObservationOutOfOrder,
    VisualAffinityMismatch,
    VisualSnapshotAffinityMismatch {
        expected_frame: u64,
        observed_frame: u64,
        observed_current: bool,
    },
    VisualSuccessorSnapshotAffinityMismatch {
        predecessor_snapshot: u64,
        expected_frame: u64,
        observed_snapshot: u64,
        observed_frame: u64,
        observed_current: bool,
    },
    VisualRefreshSnapshotAffinityMismatch {
        expected_frame: u64,
        observed_frame: u64,
        observed_current: bool,
    },
    VisualPointUnsupported,
    VisualPointIdentityMismatch,
    VisualOverlayMismatch,
    VisualPulseIncomplete,
    VisualRetirementMismatch,
    VisualResourceNotReleased,
    ObservationValueOverflow,
    QueryProjectionUnsupported,
    MultipleSchemaTransitions,
    UnexpectedSchemaTransitionIdentity,
    UnsupportedSchemaTransitionField,
    UnsupportedSchemaTransitionShape,
    MissingMountedPublication,
    SemanticFocusPublicationMismatch,
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
                pointer_input_published: false,
                keyboard_input_published: false,
                state: PlatformPulseObservationState::Started,
                visual_state: PlatformPulseVisualObservationState::AwaitingFirstFrame,
            },
            started,
        )
    }

    pub fn project_native_input_reached(
        &mut self,
        reached: super::native_input::PlatformPulseNativeInputReached,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        self.published_predecessor()?;
        let pointer_discovered =
            reached.pointer_button_events() > 0 && !self.pointer_input_published;
        let keyboard_discovered = reached.keyboard_events() > 0 && !self.keyboard_input_published;
        if !pointer_discovered
            && !keyboard_discovered
            && reached.posture()
                != super::native_input::PlatformPulseNativeInputIngressPosture::Stopped
        {
            return Err(
                PlatformPulseLifecycleObservationProjectionDenial::NativeInputEvidenceNotNovel,
            );
        }
        let envelope = self.next_envelope(
            PlatformPulseLifecycleObservation::NativeInputReached(reached),
        )?;
        self.pointer_input_published |= pointer_discovered;
        self.keyboard_input_published |= keyboard_discovered;
        Ok(envelope)
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

    pub fn project_preserved_predecessor(
        &mut self,
        source: &WorthUiSourcePackageRevision,
        denial: &UiSourceRebindAttemptFailure,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        let (_, generation, frame) = self.published_predecessor()?;
        let denial_family = match denial {
            UiSourceRebindAttemptFailure::CompilationDenied(_) => {
                PlatformPulseReplacementDenialFamily::DslCompilation
            }
            UiSourceRebindAttemptFailure::Denied(receipt) => match receipt.denial() {
                UiSourceRebindAttemptDenial::SourceIngress(_) => {
                    PlatformPulseReplacementDenialFamily::SourceIngress
                }
                UiSourceRebindAttemptDenial::RuntimePreparation(_) => {
                    PlatformPulseReplacementDenialFamily::RuntimePreparation
                }
                UiSourceRebindAttemptDenial::Candidate(_) => {
                    PlatformPulseReplacementDenialFamily::Candidate
                }
            },
        };
        self.next_envelope(PlatformPulseLifecycleObservation::RebindDeniedPreserving(
            PlatformPulseReplacementPreserved {
                source: PlatformPulseSourceSnapshotObservation::from_revision(source),
                active_generation: generation,
                active_frame: frame,
                denial_family,
            },
        ))
    }

    pub fn project_visual_comparison(
        &mut self,
        comparison: worth_ui::facade::inspection::UiVisualSnapshotComparison,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        let PlatformPulseVisualObservationState::AwaitingComparison {
            predecessor_snapshot,
            predecessor_frame,
            successor_snapshot,
            successor_frame,
        } = self.visual_state
        else {
            return Err(
                PlatformPulseLifecycleObservationProjectionDenial::VisualObservationOutOfOrder,
            );
        };
        let observation =
            super::lifecycle::PlatformPulseVisualComparison::from_comparison(comparison);
        if observation.predecessor_snapshot() != predecessor_snapshot
            || observation.successor_snapshot() != successor_snapshot
        {
            return Err(PlatformPulseLifecycleObservationProjectionDenial::VisualAffinityMismatch);
        }
        let envelope = self.next_envelope(PlatformPulseLifecycleObservation::VisualComparison(
            observation,
        ))?;
        self.visual_state = PlatformPulseVisualObservationState::AwaitingRetirement {
            snapshot: predecessor_snapshot,
            snapshot_frame: predecessor_frame,
            successor_frame,
        };
        Ok(envelope)
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

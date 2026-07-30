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

mod replacement_projection;

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
    AwaitingRefreshRetirement {
        snapshot: u64,
        snapshot_frame: u64,
        refresh_frame: u64,
    },
    AwaitingRefreshSnapshot {
        refresh_frame: u64,
    },
    Refreshed {
        snapshot: u64,
        frame: u64,
    },
    AwaitingSuccessorSnapshot {
        predecessor_snapshot: u64,
        predecessor_frame: u64,
        successor_frame: u64,
    },
    AwaitingComparison {
        predecessor_snapshot: u64,
        predecessor_frame: u64,
        successor_snapshot: u64,
        successor_frame: u64,
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
    MountedGenerationMismatch,
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
    QueryProjectionUnsupported,
    MultipleSchemaTransitions,
    UnexpectedSchemaTransitionIdentity,
    UnsupportedSchemaTransitionField,
    UnsupportedSchemaTransitionShape,
    MissingMountedPublication,
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
            } => Ok(Self::AwaitingSuccessorSnapshot {
                predecessor_snapshot: snapshot,
                predecessor_frame: snapshot_frame,
                successor_frame,
            }),
            Self::Refreshed { snapshot, frame } => Ok(Self::AwaitingSuccessorSnapshot {
                predecessor_snapshot: snapshot,
                predecessor_frame: frame,
                successor_frame,
            }),
            Self::Retired => Ok(Self::Retired),
            Self::AwaitingFirstFrame
            | Self::SnapshotCaptured { .. }
            | Self::IdentityTraced { .. }
            | Self::OverlayPublished { .. }
            | Self::AwaitingRefreshRetirement { .. }
            | Self::AwaitingRefreshSnapshot { .. }
            | Self::AwaitingSuccessorSnapshot { .. }
            | Self::AwaitingComparison { .. }
            | Self::AwaitingRetirement { .. } => {
                Err(PlatformPulseLifecycleObservationProjectionDenial::VisualPulseIncomplete)
            }
        }
    }
}

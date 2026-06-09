use forge_query::facade::{ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryPostureKind};

use crate::bindings::rebinding::{BindingContinuityClass, RebindingOutcomeClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RebindingOrdinaryOutcomeShape {
    kind: &'static str,
    posture_kind: Option<ForgeQueryOrdinaryPostureKind>,
    next_step: Option<ForgeQueryOrdinaryNextStep>,
}

impl RebindingOrdinaryOutcomeShape {
    pub(crate) fn new(
        kind: &'static str,
        posture_kind: Option<ForgeQueryOrdinaryPostureKind>,
        next_step: Option<ForgeQueryOrdinaryNextStep>,
    ) -> Self {
        Self {
            kind,
            posture_kind,
            next_step,
        }
    }

    pub(crate) fn kind(&self) -> &'static str {
        self.kind
    }

    pub(crate) fn posture_kind(&self) -> Option<ForgeQueryOrdinaryPostureKind> {
        self.posture_kind
    }

    pub(crate) fn next_step(&self) -> Option<ForgeQueryOrdinaryNextStep> {
        self.next_step
    }
}

pub struct PrimitiveRebindingReplayParity {
    replay_digest: String,
    binding_identity: String,
    anchor_identity: String,
    outcome_class: RebindingOutcomeClass,
    continuity_class: BindingContinuityClass,
    selected_candidate_identity: Option<String>,
    selected_candidate_label: Option<String>,
    unsupported_reason: String,
    next_step: Option<ForgeQueryOrdinaryNextStep>,
    ordinary_kind: &'static str,
    left_source_kind: &'static str,
    right_source_kind: &'static str,
}

impl PrimitiveRebindingReplayParity {
    pub(crate) fn new(
        replay_digest: String,
        binding_identity: String,
        anchor_identity: String,
        outcome_class: RebindingOutcomeClass,
        continuity_class: BindingContinuityClass,
        selected_candidate_identity: Option<String>,
        selected_candidate_label: Option<String>,
        unsupported_reason: String,
        next_step: Option<ForgeQueryOrdinaryNextStep>,
        ordinary_kind: &'static str,
        left_source_kind: &'static str,
        right_source_kind: &'static str,
    ) -> Self {
        Self {
            replay_digest,
            binding_identity,
            anchor_identity,
            outcome_class,
            continuity_class,
            selected_candidate_identity,
            selected_candidate_label,
            unsupported_reason,
            next_step,
            ordinary_kind,
            left_source_kind,
            right_source_kind,
        }
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub fn anchor_identity(&self) -> &str {
        &self.anchor_identity
    }

    pub fn outcome_class(&self) -> RebindingOutcomeClass {
        self.outcome_class
    }

    pub fn continuity_class(&self) -> BindingContinuityClass {
        self.continuity_class
    }

    pub fn selected_candidate_identity(&self) -> Option<&str> {
        self.selected_candidate_identity.as_deref()
    }

    pub fn selected_candidate_label(&self) -> Option<&str> {
        self.selected_candidate_label.as_deref()
    }

    pub fn unsupported_reason(&self) -> &str {
        &self.unsupported_reason
    }

    pub fn next_step(&self) -> Option<ForgeQueryOrdinaryNextStep> {
        self.next_step
    }

    pub fn ordinary_kind(&self) -> &str {
        self.ordinary_kind
    }

    pub fn left_source_kind(&self) -> &str {
        self.left_source_kind
    }

    pub fn right_source_kind(&self) -> &str {
        self.right_source_kind
    }
}

pub enum PrimitiveRebindingReplayParityError {
    EntryOutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
    RetainedIdentityMismatch {
        reason: &'static str,
    },
    ExplanationBasisMismatch {
        reason: &'static str,
    },
    ReplayNextStepMismatch {
        reason: &'static str,
        left_kind: &'static str,
        right_kind: &'static str,
        left_next_step: Option<ForgeQueryOrdinaryNextStep>,
        right_next_step: Option<ForgeQueryOrdinaryNextStep>,
        left_posture_kind: Option<ForgeQueryOrdinaryPostureKind>,
        right_posture_kind: Option<ForgeQueryOrdinaryPostureKind>,
    },
}

impl PrimitiveRebindingReplayParityError {
    pub fn reason(&self) -> &'static str {
        match self {
            Self::EntryOutcomeNotBound { .. } => {
                "geometry replay parity requires an admitted retained-view declaration envelope"
            }
            Self::RetainedIdentityMismatch { reason }
            | Self::ExplanationBasisMismatch { reason }
            | Self::ReplayNextStepMismatch { reason, .. } => reason,
        }
    }
}

impl std::fmt::Debug for PrimitiveRebindingReplayParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("PrimitiveRebindingReplayParityError");
        debug.field("reason", &self.reason());
        match self {
            Self::EntryOutcomeNotBound {
                kind,
                reason,
                next_step,
            } => debug
                .field("outcome_kind", kind)
                .field("outcome_reason", reason)
                .field("next_step", next_step),
            Self::ReplayNextStepMismatch {
                left_kind,
                right_kind,
                left_next_step,
                right_next_step,
                left_posture_kind,
                right_posture_kind,
                ..
            } => debug
                .field("left_kind", left_kind)
                .field("right_kind", right_kind)
                .field("left_next_step", left_next_step)
                .field("right_next_step", right_next_step)
                .field("left_posture_kind", left_posture_kind)
                .field("right_posture_kind", right_posture_kind),
            Self::RetainedIdentityMismatch { .. } | Self::ExplanationBasisMismatch { .. } => {
                &mut debug
            }
        };
        debug.finish()
    }
}

pub(crate) fn ensure_equal<T: Eq>(
    left: T,
    right: T,
    reason: &'static str,
) -> Result<(), PrimitiveRebindingReplayParityError> {
    if left == right {
        Ok(())
    } else if reason.contains("explanation")
        || reason.contains("diagnostics")
        || reason.contains("candidate")
    {
        Err(PrimitiveRebindingReplayParityError::ExplanationBasisMismatch { reason })
    } else {
        Err(PrimitiveRebindingReplayParityError::RetainedIdentityMismatch { reason })
    }
}

pub(crate) fn sorted_join(values: &[String]) -> String {
    let mut values = values.to_vec();
    values.sort();
    values.join("|")
}

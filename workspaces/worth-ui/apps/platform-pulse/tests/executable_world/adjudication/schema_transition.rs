use std::fmt;

use worth_ui_platform_pulse::observation_contract::{
    PlatformPulseProjectionSchemaField, PlatformPulseProjectionSchemaTransitionKind,
    PlatformPulseProjectionSchemaTransitionObservation, PlatformPulseQueryProjectionEvidence,
    PlatformPulseQueryProjectionPosture,
};

use crate::external_observation::NativeClientPixelCapture;

use super::ExecutableReplacementEvidence;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExpectedSchemaTransition {
    Stopped,
    Recovered,
}

#[derive(Debug)]
pub(crate) struct ExecutableSchemaTransitionEvidence<Kind> {
    replacement: ExecutableReplacementEvidence<Kind>,
    transition: PlatformPulseProjectionSchemaTransitionObservation,
    query_basis: PlatformPulseQueryProjectionEvidence,
    retained_upper_pixel_bytes: usize,
    changed_lower_pixel_bytes: usize,
    canonical_current_restored: bool,
}

#[derive(Debug)]
pub(crate) enum ExecutableSchemaTransitionFailure {
    MissingTransition,
    WrongTransitionKind,
    WrongFieldProgression,
    PredecessorValueNotPreserved,
    QueryValueNotCurrent,
    QueryOwnerDidNotReachSecondCurrent,
    QueryGenerationMissing,
    CaptureExtentChanged,
    ValueRegionChanged,
    PostureRegionDidNotChange,
    CanonicalCurrentPixelsNotRestored,
}

pub(crate) fn adjudicate_schema_transition<Kind>(
    replacement: ExecutableReplacementEvidence<Kind>,
    predecessor_pixels: &NativeClientPixelCapture,
    canonical_current_pixels: Option<&NativeClientPixelCapture>,
    query_basis: &PlatformPulseQueryProjectionEvidence,
    expected: ExpectedSchemaTransition,
) -> Result<ExecutableSchemaTransitionEvidence<Kind>, ExecutableSchemaTransitionFailure> {
    let transition = replacement
        .replacement()
        .schema_transition()
        .copied()
        .ok_or(ExecutableSchemaTransitionFailure::MissingTransition)?;
    require_transition(transition, expected)?;
    require_query_basis(query_basis)?;
    let (retained_upper_pixel_bytes, changed_lower_pixel_bytes) =
        require_visible_preservation(predecessor_pixels, replacement.pixels())?;
    let canonical_current_restored = match (expected, canonical_current_pixels) {
        (ExpectedSchemaTransition::Recovered, Some(canonical)) => {
            if canonical.width() != replacement.pixels().width()
                || canonical.height() != replacement.pixels().height()
                || canonical.rgba() != replacement.pixels().rgba()
            {
                return Err(ExecutableSchemaTransitionFailure::CanonicalCurrentPixelsNotRestored);
            }
            true
        }
        (ExpectedSchemaTransition::Recovered, None) => {
            return Err(ExecutableSchemaTransitionFailure::CanonicalCurrentPixelsNotRestored)
        }
        (ExpectedSchemaTransition::Stopped, _) => false,
    };
    Ok(ExecutableSchemaTransitionEvidence {
        replacement,
        transition,
        query_basis: query_basis.clone(),
        retained_upper_pixel_bytes,
        changed_lower_pixel_bytes,
        canonical_current_restored,
    })
}

fn require_transition(
    transition: PlatformPulseProjectionSchemaTransitionObservation,
    expected: ExpectedSchemaTransition,
) -> Result<(), ExecutableSchemaTransitionFailure> {
    let expected_fields = match expected {
        ExpectedSchemaTransition::Stopped => (
            PlatformPulseProjectionSchemaTransitionKind::Stopped,
            PlatformPulseProjectionSchemaField::Status,
            PlatformPulseProjectionSchemaField::Revision,
        ),
        ExpectedSchemaTransition::Recovered => (
            PlatformPulseProjectionSchemaTransitionKind::Recovered,
            PlatformPulseProjectionSchemaField::Revision,
            PlatformPulseProjectionSchemaField::Status,
        ),
    };
    if transition.kind() != expected_fields.0 {
        return Err(ExecutableSchemaTransitionFailure::WrongTransitionKind);
    }
    if transition.predecessor_selected_field() != expected_fields.1
        || transition.candidate_selected_field() != expected_fields.2
        || transition.installed_selected_field() != PlatformPulseProjectionSchemaField::Status
    {
        return Err(ExecutableSchemaTransitionFailure::WrongFieldProgression);
    }
    if !transition.predecessor_value_preserved() {
        return Err(ExecutableSchemaTransitionFailure::PredecessorValueNotPreserved);
    }
    Ok(())
}

fn require_query_basis(
    query: &PlatformPulseQueryProjectionEvidence,
) -> Result<(), ExecutableSchemaTransitionFailure> {
    if query.posture() != PlatformPulseQueryProjectionPosture::Current
        || query.native_value() != Some("UPDATED-LONG")
    {
        return Err(ExecutableSchemaTransitionFailure::QueryValueNotCurrent);
    }
    if query.owner_order() != 5 {
        return Err(ExecutableSchemaTransitionFailure::QueryOwnerDidNotReachSecondCurrent);
    }
    if query.source_generation().is_empty() || query.result_generation().is_empty() {
        return Err(ExecutableSchemaTransitionFailure::QueryGenerationMissing);
    }
    Ok(())
}

fn require_visible_preservation(
    predecessor: &NativeClientPixelCapture,
    successor: &NativeClientPixelCapture,
) -> Result<(usize, usize), ExecutableSchemaTransitionFailure> {
    if predecessor.width() != successor.width() || predecessor.height() != successor.height() {
        return Err(ExecutableSchemaTransitionFailure::CaptureExtentChanged);
    }
    let row_bytes = usize::try_from(successor.width())
        .ok()
        .and_then(|width| width.checked_mul(4))
        .ok_or(ExecutableSchemaTransitionFailure::CaptureExtentChanged)?;
    let split = usize::try_from(successor.height() / 2)
        .ok()
        .and_then(|rows| rows.checked_mul(row_bytes))
        .ok_or(ExecutableSchemaTransitionFailure::CaptureExtentChanged)?;
    let (predecessor_upper, predecessor_lower) = predecessor.rgba().split_at(split);
    let (successor_upper, successor_lower) = successor.rgba().split_at(split);
    if predecessor_upper != successor_upper {
        return Err(ExecutableSchemaTransitionFailure::ValueRegionChanged);
    }
    let changed_lower_pixel_bytes = predecessor_lower
        .iter()
        .zip(successor_lower)
        .filter(|(predecessor, successor)| predecessor != successor)
        .count();
    if changed_lower_pixel_bytes == 0 {
        return Err(ExecutableSchemaTransitionFailure::PostureRegionDidNotChange);
    }
    Ok((successor_upper.len(), changed_lower_pixel_bytes))
}

impl<Kind> ExecutableSchemaTransitionEvidence<Kind> {
    pub(crate) fn replacement(&self) -> &ExecutableReplacementEvidence<Kind> {
        &self.replacement
    }

    pub(crate) const fn transition(&self) -> PlatformPulseProjectionSchemaTransitionObservation {
        self.transition
    }

    pub(crate) fn query_basis(&self) -> &PlatformPulseQueryProjectionEvidence {
        &self.query_basis
    }

    pub(crate) const fn retained_upper_pixel_bytes(&self) -> usize {
        self.retained_upper_pixel_bytes
    }

    pub(crate) const fn changed_lower_pixel_bytes(&self) -> usize {
        self.changed_lower_pixel_bytes
    }

    pub(crate) const fn canonical_current_restored(&self) -> bool {
        self.canonical_current_restored
    }
}

impl fmt::Display for ExecutableSchemaTransitionFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::MissingTransition => "replacement omitted its schema transition",
            Self::WrongTransitionKind => "schema transition carried the wrong kind",
            Self::WrongFieldProgression => "schema transition carried the wrong field progression",
            Self::PredecessorValueNotPreserved => {
                "schema transition did not preserve the predecessor value"
            }
            Self::QueryValueNotCurrent => "retained Query basis is not UPDATED-LONG/current",
            Self::QueryOwnerDidNotReachSecondCurrent => {
                "retained Query basis is not owner order five"
            }
            Self::QueryGenerationMissing => "retained Query basis omitted generation identity",
            Self::CaptureExtentChanged => "schema transition changed native capture extent",
            Self::ValueRegionChanged => "schema transition changed the native value region",
            Self::PostureRegionDidNotChange => {
                "schema transition did not visibly change the native posture region"
            }
            Self::CanonicalCurrentPixelsNotRestored => {
                "schema recovery did not restore canonical current pixels"
            }
        })
    }
}

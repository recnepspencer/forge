use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessPhaseFourSeed, WorthGraphReadAccessResolvedPosture,
};

use super::super::errors::{
    WorthGraphReadAccessFirstVerticalSliceError, WorthGraphReadAccessFirstVerticalSliceErrorKind,
};
use super::selected_slice::WorthGraphReadAccessSelectedVerticalSlice;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessSliceSelectionReason {
    FirstInlineIndexedCandidate,
    FirstBoundedEphemeralIndexCandidate,
    FirstAdmittedPagedStreamingCandidate,
    FirstRequiredOrDeniedCandidate,
}

impl WorthGraphReadAccessSliceSelectionReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstInlineIndexedCandidate => "first_inline_indexed_candidate",
            Self::FirstBoundedEphemeralIndexCandidate => "first_bounded_ephemeral_index_candidate",
            Self::FirstAdmittedPagedStreamingCandidate => {
                "first_admitted_paged_streaming_candidate"
            }
            Self::FirstRequiredOrDeniedCandidate => "first_required_or_denied_candidate",
        }
    }
}

pub(crate) fn select_first_vertical_slice(
    seed: &WorthGraphReadAccessPhaseFourSeed,
) -> Result<WorthGraphReadAccessSelectedVerticalSlice, WorthGraphReadAccessFirstVerticalSliceError>
{
    select_by_posture(
        seed.resolved_postures(),
        "inline_indexed",
        WorthGraphReadAccessSliceSelectionReason::FirstInlineIndexedCandidate,
    )
    .or_else(|| {
        select_by_posture(
            seed.resolved_postures(),
            "bounded_ephemeral_index",
            WorthGraphReadAccessSliceSelectionReason::FirstBoundedEphemeralIndexCandidate,
        )
    })
    .or_else(|| {
        select_by_posture(
            seed.resolved_postures(),
            "admitted_paged_streaming",
            WorthGraphReadAccessSliceSelectionReason::FirstAdmittedPagedStreamingCandidate,
        )
    })
    .or_else(|| {
        seed.resolved_postures().first().map(|posture| {
            WorthGraphReadAccessSelectedVerticalSlice::from_resolved_posture(
                posture,
                WorthGraphReadAccessSliceSelectionReason::FirstRequiredOrDeniedCandidate,
            )
        })
    })
    .ok_or_else(|| {
        WorthGraphReadAccessFirstVerticalSliceError::new(
            WorthGraphReadAccessFirstVerticalSliceErrorKind::MissingSelectedSlice,
        )
    })
}

fn select_by_posture(
    postures: &[WorthGraphReadAccessResolvedPosture],
    query_posture: &str,
    reason: WorthGraphReadAccessSliceSelectionReason,
) -> Option<WorthGraphReadAccessSelectedVerticalSlice> {
    postures
        .iter()
        .find(|posture| posture.query_posture() == query_posture)
        .map(|posture| {
            WorthGraphReadAccessSelectedVerticalSlice::from_resolved_posture(posture, reason)
        })
}

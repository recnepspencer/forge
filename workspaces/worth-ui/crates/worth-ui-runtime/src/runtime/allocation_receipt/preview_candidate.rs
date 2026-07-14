use super::UiAllocationCandidate;

/// Candidate geometry projected for an interaction frame; it has no durable authority.
///
/// ```compile_fail
/// use worth_ui_runtime::facade::{
///     runtime_handoff::WorthUiRuntime, UiAllocationPreviewCandidate,
/// };
///
/// fn preview_cannot_enter_committed_host_lane(
///     runtime: &WorthUiRuntime,
///     preview: &UiAllocationPreviewCandidate,
/// ) {
///     let _ = runtime.allocate_runtime_handles(preview);
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct UiAllocationPreviewCandidate {
    candidate: UiAllocationCandidate,
}

impl UiAllocationPreviewCandidate {
    pub(in crate::runtime) fn from_candidate(candidate: UiAllocationCandidate) -> Self {
        Self { candidate }
    }

    pub fn candidate_is_admitted(&self) -> bool {
        self.candidate.is_admitted()
    }

    pub fn truth_category(&self) -> crate::evidence::allocation::UiAllocationTruthCategory {
        crate::evidence::allocation::UiAllocationTruthCategory::PreviewCandidate
    }
    pub fn resize_basis(&self) -> Option<&crate::runtime::UiResizeAllocationPlanningBasis> {
        self.candidate.resize_basis()
    }
}

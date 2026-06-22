use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanFragmentContinuationIndex;

pub struct PlanarBooleanClosedWalkCandidateSetInput<'a> {
    continuation_index: &'a PlanarBooleanFragmentContinuationIndex,
}

impl<'a> PlanarBooleanClosedWalkCandidateSetInput<'a> {
    pub fn from_continuation_index(
        continuation_index: &'a PlanarBooleanFragmentContinuationIndex,
    ) -> Self {
        Self { continuation_index }
    }

    pub(crate) fn continuation_index(&self) -> &'a PlanarBooleanFragmentContinuationIndex {
        self.continuation_index
    }
}

use crate::branch::{
    ProductBranchObservation, ProductBranchObservationMismatch, ProductBranchReferenceSnapshot,
};
use crate::publication::ProductBranchIntent;

/// Product-head observation admitted for the next phase. The observation is
/// carried unchanged; a caller cannot swap a basis between planning steps.
#[derive(Debug)]
pub struct ResolvedExpectedProductHead {
    intent: ProductBranchIntent,
    expected: ProductBranchObservation,
}

impl ResolvedExpectedProductHead {
    /// Admit an expected observation only when the branch-local reference
    /// cell still selects the complete same image. The observation is carried
    /// unchanged; this function never re-observes a component or asks for an
    /// ambient latest basis.
    pub(crate) fn from_current(
        intent: ProductBranchIntent,
        expected: ProductBranchObservation,
        current: &ProductBranchReferenceSnapshot,
    ) -> Result<Self, ProductBranchObservationMismatch> {
        if let Some(mismatch) = expected.mismatch_against_snapshot(current) {
            return Err(mismatch);
        }
        Ok(Self { intent, expected })
    }

    pub fn expected(&self) -> &ProductBranchObservation {
        &self.expected
    }

    pub fn intent(&self) -> &ProductBranchIntent {
        &self.intent
    }

    pub(crate) fn into_parts(self) -> (ProductBranchIntent, ProductBranchObservation) {
        (self.intent, self.expected)
    }

    pub(crate) fn take_plan_inputs(
        &mut self,
    ) -> (
        Option<worth_relational::facade::mvcc::PreparedRelationalCommitCandidate>,
        Option<worth_relational::facade::branch::AdmittedRelationalForkSourceBasis>,
        Option<worth_signal::facade::branch::ValidatedSignalBranchName>,
    ) {
        self.intent.take_plan_inputs()
    }
}

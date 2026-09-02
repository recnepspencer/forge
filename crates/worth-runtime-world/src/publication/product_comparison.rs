use crate::branch::ProductBranchObservation;
use crate::publication::ProductBranchIntent;

/// Product-head observation admitted for the next phase. The observation is
/// carried unchanged; a caller cannot swap a basis between planning steps.
#[derive(Debug)]
pub struct ResolvedExpectedProductHead {
    intent: ProductBranchIntent,
    expected: ProductBranchObservation,
}

impl ResolvedExpectedProductHead {
    pub(crate) fn new(intent: ProductBranchIntent, expected: ProductBranchObservation) -> Self {
        Self { intent, expected }
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
}

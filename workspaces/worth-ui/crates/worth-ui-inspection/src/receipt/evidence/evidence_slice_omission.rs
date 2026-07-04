use crate::{UiEvidenceBudget, UiInspectionScope};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiEvidenceSliceOmission {
    ByBudget { budget: UiEvidenceBudget },
    ByScope { scope: UiInspectionScope },
}

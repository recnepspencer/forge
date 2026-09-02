use super::AdmittedCompositeRuntimeWorldBasis;
use crate::identity::CompositeBasisIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CompositeBasisMismatch {
    expected: CompositeBasisIdentity,
    observed: CompositeBasisIdentity,
}

impl CompositeBasisMismatch {
    pub(crate) fn expected(&self) -> &CompositeBasisIdentity {
        &self.expected
    }

    pub(crate) fn observed(&self) -> &CompositeBasisIdentity {
        &self.observed
    }
}

pub(crate) fn compare_exact(
    expected: &AdmittedCompositeRuntimeWorldBasis,
    observed: &AdmittedCompositeRuntimeWorldBasis,
) -> Result<(), CompositeBasisMismatch> {
    if expected.identity() == observed.identity() {
        Ok(())
    } else {
        Err(CompositeBasisMismatch {
            expected: expected.identity().clone(),
            observed: observed.identity().clone(),
        })
    }
}

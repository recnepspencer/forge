use super::SnapshotReferenceViolation;

/// Canonical validation report for frozen snapshot references.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotReferenceValidationReport {
    violations: Vec<SnapshotReferenceViolation>,
}

impl SnapshotReferenceValidationReport {
    pub(crate) fn new(mut violations: Vec<SnapshotReferenceViolation>) -> Self {
        violations.sort_by(|left, right| left.ordering_key().cmp(&right.ordering_key()));
        Self { violations }
    }

    pub fn lowering_is_admissible(&self) -> bool {
        self.violations.is_empty()
    }

    pub fn violations(&self) -> &[SnapshotReferenceViolation] {
        &self.violations
    }
}

/// Why an immutable branch root remains retained by the Relational owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationalBasisRetentionReason {
    Observation,
    ExternalComponentBasis,
}

/// One owner-issued observation obligation shared by all clones of an
/// admitted basis and its repeatable observations.
#[derive(Debug)]
pub(crate) struct RelationalObservationRetentionObligation {
    reason: RelationalBasisRetentionReason,
}

impl RelationalObservationRetentionObligation {
    pub(crate) fn new() -> Self {
        Self {
            reason: RelationalBasisRetentionReason::Observation,
        }
    }

    pub(crate) const fn reason(&self) -> RelationalBasisRetentionReason {
        self.reason
    }
}
